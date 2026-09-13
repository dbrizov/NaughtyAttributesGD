use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

use godot::classes::{
    EditorInspectorPlugin, EditorInterface, EditorProperty, EditorUndoRedoManager,
    IEditorInspectorPlugin, Script,
};
use godot::obj::InstanceId;
use godot::prelude::*;
use godot::register::info::{PropertyHint, PropertyUsageFlags};

use na_core::descriptor::{PropertyDescriptor, ScriptDescriptor};

use crate::property_blocks::{self, PropertyBlock};
use crate::property_undo_redo::{EditSession, PropertyChange, PropertyEditAction};
use crate::property_utils;

/// Set while a default editor is instantiated to prevent internal mutability race conditions.
struct InstantiationScope(Rc<Cell<bool>>);

impl InstantiationScope {
    fn enter(flag: &Rc<Cell<bool>>) -> Self {
        flag.set(true);
        Self(Rc::clone(flag))
    }
}

impl Drop for InstantiationScope {
    fn drop(&mut self) {
        self.0.set(false);
    }
}

/// The state of a Node or Resource the inspector is editing.
struct ObjectState {
    object: Gd<Object>,
    script: Rc<ScriptDescriptor>,
    property_blocks: HashMap<StringName, PropertyBlock>,
    edit_action: Option<PropertyEditAction>,
}

impl ObjectState {
    fn is_stale(&self) -> bool {
        let is_built = self.edit_action.is_none();
        let has_editors = self
            .property_blocks
            .values()
            .any(|property_block| property_block.editor.is_instance_valid());

        !self.object.is_instance_valid() || (is_built && !has_editors)
    }
}

#[derive(Default)]
struct InspectorState {
    object_states: HashMap<InstanceId, ObjectState>,
    edit_session: Option<EditSession>,
}

/// Draws the claimed properties of a naughty script, and leaves every other property to Godot.
#[derive(GodotClass)]
#[class(tool, init, base = EditorInspectorPlugin)]
pub struct NaughtyEditorInspectorPlugin {
    state: RefCell<InspectorState>,
    instantiating: Rc<Cell<bool>>,
    base: Base<EditorInspectorPlugin>,
}

#[godot_api]
impl IEditorInspectorPlugin for NaughtyEditorInspectorPlugin {
    fn can_handle(&self, object: Option<Gd<Object>>) -> bool {
        if self.instantiating.get() {
            return false;
        }

        object
            .and_then(|object| self.create_naughty_script(&object))
            .is_some()
    }

    fn parse_begin(&mut self, object: Option<Gd<Object>>) {
        let Some(object) = object else {
            return;
        };

        let Some(script) = self.create_naughty_script(&object) else {
            return;
        };

        let mut state = self.state.borrow_mut();
        state
            .object_states
            .retain(|_, object_state| !object_state.is_stale());
        state.object_states.insert(
            object.instance_id(),
            ObjectState {
                object: Gd::clone(&object),
                script,
                property_blocks: HashMap::new(),
                edit_action: Some(PropertyEditAction::new(&object)),
            },
        );
    }

    fn parse_end(&mut self, object: Option<Gd<Object>>) {
        let Some(object) = object else {
            return;
        };

        let (script_name, edit_action) = {
            let mut state = self.state.borrow_mut();
            let Some(object_state) = state.object_states.get_mut(&object.instance_id()) else {
                return;
            };

            (
                object_state.script.name.clone(),
                object_state.edit_action.take(),
            )
        };

        let action_name = format!("Validate {script_name}");
        let mut edit_action = edit_action;
        Callable::from_fn("commit_validation", move |_args| {
            if let Some(edit_action) = edit_action.take() {
                edit_action.commit(&action_name);
            }

            Variant::nil()
        })
        .call_deferred(&[]);

        self.apply_property_labels(object.instance_id());
        self.refresh_property_blocks();
    }

    fn parse_property(
        &mut self,
        object: Option<Gd<Object>>,
        _variant_type: VariantType,
        name: GString,
        _hint: PropertyHint,
        _hint_text: GString,
        _usage: PropertyUsageFlags,
        wide: bool,
    ) -> bool {
        let Some(object) = object else {
            return false;
        };

        if self.instantiating.get() {
            return false;
        }

        let name = StringName::from(&name);
        let instance_id = object.instance_id();

        let (script, edit_action) = {
            let mut state = self.state.borrow_mut();
            let Some(object_state) = state.object_states.get_mut(&instance_id) else {
                return false;
            };

            (
                Rc::clone(&object_state.script),
                object_state.edit_action.take(),
            )
        };

        let Some(mut edit_action) = edit_action else {
            return false;
        };

        let property_block = script
            .find_property(&name)
            .filter(|property| property.claimed)
            .and_then(|property| {
                self.create_property_block(&mut edit_action, &object, property, wide)
            });

        if let Some(object_state) = self.state.borrow_mut().object_states.get_mut(&instance_id) {
            object_state.edit_action = Some(edit_action);
        }

        let Some(property_block) = property_block else {
            return false;
        };

        if let Some(container) = &property_block.decorations_container {
            self.base_mut().add_custom_control(container);
        }

        self.base_mut()
            .add_property_editor(&GString::from(&name), &property_block.editor);

        if let Some(object_state) = self.state.borrow_mut().object_states.get_mut(&instance_id) {
            object_state.property_blocks.insert(name, property_block);
        }

        true
    }
}

#[godot_api]
impl NaughtyEditorInspectorPlugin {
    #[func]
    fn on_inspector_edit(
        &mut self,
        undo_redo: Gd<EditorUndoRedoManager>,
        object: Gd<Object>,
        name: GString,
        value: Variant,
    ) {
        let name = StringName::from(&name);
        if name == "script" {
            return;
        }

        let Some(script) = self
            .find_script(object.instance_id())
            .or_else(|| self.create_naughty_script(&object))
        else {
            return;
        };

        let mut undo_redo = undo_redo;
        let mut edit_session = self.state.borrow_mut().edit_session.take();
        let mut edit_action = PropertyEditAction::new(&object);

        {
            let _base = self.base_mut();
            let session =
                EditSession::resume(&mut edit_session, &undo_redo, &object, &script, &name);
            edit_action.set_property_value(&name, &value);
            property_utils::validate_properties(&mut edit_action, &object, &script.properties);
            edit_action.add_to(&mut undo_redo, session, &value);
        }

        self.state.borrow_mut().edit_session = edit_session;
        self.defer_value_changed_callbacks(object, script, edit_action.into_changes());
        self.base_mut().call_deferred("sync_property_blocks", &[]);
    }

    #[func]
    fn sync_property_blocks(&mut self) {
        let editors: Vec<Gd<EditorProperty>> = self
            .state
            .borrow()
            .object_states
            .values()
            .flat_map(|object_state| object_state.property_blocks.values())
            .map(|property_block| Gd::clone(&property_block.editor))
            .collect();

        {
            let _base = self.base_mut();
            for mut editor in editors {
                if editor.is_instance_valid() {
                    editor.update_property();
                }
            }
        }

        self.refresh_property_blocks();
    }

    #[func]
    fn rebuild_if_stale(&self) {
        let Some(mut object) = EditorInterface::singleton()
            .get_inspector()
            .and_then(|inspector| inspector.get_edited_object())
        else {
            return;
        };

        let is_tool_script = object
            .get("script")
            .try_to::<Gd<Script>>()
            .is_ok_and(|script| script.is_tool());

        if !is_tool_script {
            return;
        }

        let stale = match self.find_script(object.instance_id()) {
            Some(script) => script.is_stale(&object),
            None => ScriptDescriptor::from_object(&object).is_naughty(),
        };

        if stale {
            object.notify_property_list_changed();
        }
    }
}

impl NaughtyEditorInspectorPlugin {
    /// Returns `None` if the object's script is not naughty.
    fn create_naughty_script(&self, object: &Gd<Object>) -> Option<Rc<ScriptDescriptor>> {
        let script = Rc::new(ScriptDescriptor::from_object(object));
        script.is_naughty().then_some(script)
    }

    fn create_property_block(
        &mut self,
        edit_action: &mut PropertyEditAction,
        object: &Gd<Object>,
        property: &PropertyDescriptor,
        wide: bool,
    ) -> Option<PropertyBlock> {
        let _scope = InstantiationScope::enter(&self.instantiating);
        let _base = self.base_mut();
        property_blocks::create_property_block(edit_action, object, property, wide)
    }

    fn find_script(&self, instance_id: InstanceId) -> Option<Rc<ScriptDescriptor>> {
        self.state
            .borrow()
            .object_states
            .get(&instance_id)
            .map(|object_state| Rc::clone(&object_state.script))
    }

    fn find_property_block(
        &self,
        instance_id: InstanceId,
        name: &StringName,
    ) -> Option<PropertyBlock> {
        self.state
            .borrow()
            .object_states
            .get(&instance_id)
            .and_then(|object_state| object_state.property_blocks.get(name).cloned())
    }

    fn refresh_property_blocks(&self) {
        let objects: Vec<(InstanceId, Gd<Object>, Rc<ScriptDescriptor>)> = self
            .state
            .borrow()
            .object_states
            .iter()
            .filter(|(_, object_state)| object_state.object.is_instance_valid())
            .map(|(instance_id, object_state)| {
                (
                    *instance_id,
                    Gd::clone(&object_state.object),
                    Rc::clone(&object_state.script),
                )
            })
            .collect();

        for (instance_id, object, script) in &objects {
            for property in &script.properties {
                let Some(mut property_block) =
                    self.find_property_block(*instance_id, &property.name)
                else {
                    continue;
                };

                let visible = property_utils::is_visible(object, property);
                property_block.set_visible(visible);

                let enabled = property_utils::is_enabled(object, property);
                property_block.set_enabled(enabled);
            }
        }
    }

    fn apply_property_labels(&self, instance_id: InstanceId) {
        let Some(script) = self.find_script(instance_id) else {
            return;
        };

        for property in &script.properties {
            let Some(label) = property_utils::get_label(property) else {
                continue;
            };

            if let Some(mut property_block) = self.find_property_block(instance_id, &property.name)
            {
                property_block.set_label(label);
            }
        }
    }

    fn defer_value_changed_callbacks(
        &self,
        mut object: Gd<Object>,
        script: Rc<ScriptDescriptor>,
        changes: Vec<PropertyChange>,
    ) {
        if changes.is_empty() {
            return;
        }

        Callable::from_fn("call_value_changed_callbacks", move |_args| {
            if object.is_instance_valid() {
                for change in &changes {
                    if let Some(property) = script.find_property(&change.name) {
                        property_utils::call_value_changed_callbacks(
                            &mut object,
                            property,
                            &change.old_value,
                            &change.new_value,
                        );
                    }
                }
            }

            Variant::nil()
        })
        .call_deferred(&[]);
    }
}

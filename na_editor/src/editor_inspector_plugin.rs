use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

use godot::classes::{
    Control, EditorInspectorPlugin, EditorInterface, EditorProperty, EditorUndoRedoManager,
    IEditorInspectorPlugin, Script,
};
use godot::obj::InstanceId;
use godot::prelude::*;
use godot::register::info::{PropertyHint, PropertyUsageFlags};

use na_core::descriptor::{ClassDescriptor, PropertyDescriptor};

use crate::property_editors::{self, PropertyEditor};
use crate::property_undo_redo::{EditSession, PropertyEditAction};
use crate::property_utils;

/// Set while a default editor is instantiated to prevent internal mutability raise conditions.
struct InstantiationScope(Rc<Cell<bool>>);

impl InstantiationScope {
    fn enter(flag: &Rc<Cell<bool>>) -> Self {
        flag.set(true);
        Self(flag.clone())
    }
}

impl Drop for InstantiationScope {
    fn drop(&mut self) {
        self.0.set(false);
    }
}

struct ObjectState {
    class: Rc<ClassDescriptor>,
    object: Gd<Object>,
    property_editors: HashMap<StringName, PropertyEditor>,
    edit_action: Option<PropertyEditAction>,
}

impl ObjectState {
    fn is_stale(&self) -> bool {
        let is_built = self.edit_action.is_none();
        let has_editors = self
            .property_editors
            .values()
            .any(|property_editor| property_editor.editor.is_instance_valid());

        !self.object.is_instance_valid() || (is_built && !has_editors)
    }
}

#[derive(Default)]
struct InspectorState {
    objects: HashMap<InstanceId, ObjectState>,
    edit_session: Option<EditSession>,
}

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
            .and_then(|object| self.create_naughty_class(&object))
            .is_some()
    }

    fn parse_begin(&mut self, object: Option<Gd<Object>>) {
        let Some(object) = object else {
            return;
        };

        let Some(class) = self.create_naughty_class(&object) else {
            return;
        };

        let mut state = self.state.borrow_mut();
        state
            .objects
            .retain(|_, object_state| !object_state.is_stale());
        state.objects.insert(
            object.instance_id(),
            ObjectState {
                class,
                object: object.clone(),
                property_editors: HashMap::new(),
                edit_action: Some(PropertyEditAction::new(&object)),
            },
        );
    }

    fn parse_end(&mut self, object: Option<Gd<Object>>) {
        let Some(object) = object else {
            return;
        };

        let (edit_action, script_name) = {
            let mut state = self.state.borrow_mut();
            let Some(object_state) = state.objects.get_mut(&object.instance_id()) else {
                return;
            };

            (
                object_state.edit_action.take(),
                object_state.class.script_name.clone(),
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

        let (class, edit_action) = {
            let mut state = self.state.borrow_mut();
            let Some(object_state) = state.objects.get_mut(&instance_id) else {
                return false;
            };

            (object_state.class.clone(), object_state.edit_action.take())
        };

        let Some(mut edit_action) = edit_action else {
            return false;
        };

        let property_editor = class
            .find(&name)
            .filter(|property| property.claimed)
            .and_then(|property| {
                self.create_property_editor(&mut edit_action, &object, property, wide)
            });

        if let Some(object_state) = self.state.borrow_mut().objects.get_mut(&instance_id) {
            object_state.edit_action = Some(edit_action);
        }

        let Some(property_editor) = property_editor else {
            return false;
        };

        if let Some(decorations) = &property_editor.decorations {
            self.base_mut().add_custom_control(decorations);
        }

        let editor: Gd<Control> = property_editor.editor.clone().upcast();
        self.base_mut()
            .add_property_editor(&GString::from(&name), &editor);

        if let Some(object_state) = self.state.borrow_mut().objects.get_mut(&instance_id) {
            object_state.property_editors.insert(name, property_editor);
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

        let class = self
            .state
            .borrow()
            .objects
            .get(&object.instance_id())
            .map(|object_state| object_state.class.clone())
            .or_else(|| self.create_naughty_class(&object));

        let Some(class) = class else {
            return;
        };

        let mut undo_redo = undo_redo;
        let mut edit_session = self.state.borrow_mut().edit_session.take();

        {
            let _base = self.base_mut();
            let session =
                EditSession::resume(&mut edit_session, &undo_redo, &object, &class, &name);
            let mut edit_action = PropertyEditAction::new(&object);
            edit_action.set_property_value(&name, &value);
            property_utils::validate_properties(&mut edit_action, &object, &class.properties);
            edit_action.add_to(&mut undo_redo, session, &value);
        }

        self.state.borrow_mut().edit_session = edit_session;
        self.base_mut().call_deferred("sync_property_editors", &[]);
    }

    #[func]
    fn sync_property_editors(&mut self) {
        let editors: Vec<Gd<EditorProperty>> = self
            .state
            .borrow()
            .objects
            .values()
            .flat_map(|object_state| object_state.property_editors.values())
            .map(|property_editor| property_editor.editor.clone())
            .collect();

        {
            let _base = self.base_mut();
            for mut editor in editors {
                if editor.is_instance_valid() {
                    editor.update_property();
                }
            }
        }

        self.refresh_property_editors();
    }

    fn refresh_property_editors(&self) {
        let objects: Vec<(InstanceId, Gd<Object>, Rc<ClassDescriptor>)> = self
            .state
            .borrow()
            .objects
            .iter()
            .filter(|(_, object_state)| object_state.object.is_instance_valid())
            .map(|(instance_id, object_state)| {
                (
                    *instance_id,
                    object_state.object.clone(),
                    object_state.class.clone(),
                )
            })
            .collect();

        for (instance_id, object, class) in &objects {
            for property in &class.properties {
                let Some(property_editor) = self.find_property_editor(*instance_id, &property.name)
                else {
                    continue;
                };

                let visible = property_utils::is_visible(object, property);
                property_editor.set_visible(visible);
            }
        }
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

        let class = self
            .state
            .borrow()
            .objects
            .get(&object.instance_id())
            .map(|object_state| object_state.class.clone());

        let stale = match class {
            Some(class) => class.is_stale(&object),
            None => ClassDescriptor::from_object(&object).is_naughty(),
        };

        if stale {
            object.notify_property_list_changed();
        }
    }
}

impl NaughtyEditorInspectorPlugin {
    /// Returns `None` if the object's script is not naughty.
    fn create_naughty_class(&self, object: &Gd<Object>) -> Option<Rc<ClassDescriptor>> {
        let class = Rc::new(ClassDescriptor::from_object(object));
        class.is_naughty().then_some(class)
    }

    fn create_property_editor(
        &mut self,
        edit_action: &mut PropertyEditAction,
        object: &Gd<Object>,
        property: &PropertyDescriptor,
        wide: bool,
    ) -> Option<PropertyEditor> {
        let _scope = InstantiationScope::enter(&self.instantiating);
        let _base = self.base_mut();
        property_editors::create_property_editor(edit_action, object, property, wide)
    }

    fn find_property_editor(
        &self,
        instance_id: InstanceId,
        name: &StringName,
    ) -> Option<PropertyEditor> {
        self.state
            .borrow()
            .objects
            .get(&instance_id)
            .and_then(|object_state| object_state.property_editors.get(name).cloned())
    }
}

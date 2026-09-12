use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use godot::classes::{
    Control, EditorInspectorPlugin, EditorInterface, IEditorInspectorPlugin, Script, VBoxContainer,
};
use godot::prelude::*;
use godot::register::info::{PropertyHint, PropertyUsageFlags};

use na_core::descriptor::ClassDescriptor;

use crate::property_changes;
use crate::property_editors::{self, PropertyEditor};
use crate::property_utils;

#[derive(Default)]
struct InspectorState {
    class: Option<Rc<ClassDescriptor>>,
    object: Option<Gd<Object>>,
    property_editors: HashMap<StringName, PropertyEditor>,
    pending_container: Option<Gd<Control>>,
}

#[derive(GodotClass)]
#[class(tool, init, base = EditorInspectorPlugin)]
pub struct NaughtyEditorInspectorPlugin {
    state: Rc<RefCell<InspectorState>>,
    base: Base<EditorInspectorPlugin>,
}

#[godot_api]
impl IEditorInspectorPlugin for NaughtyEditorInspectorPlugin {
    fn can_handle(&self, object: Option<Gd<Object>>) -> bool {
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

        let container: Gd<Control> = VBoxContainer::new_alloc().upcast();

        {
            let mut state = self.state.borrow_mut();
            state.class = Some(class.clone());
            state.object = Some(object.clone());
            state.property_editors.clear();
            state.pending_container = Some(container.clone());
        }

        let state = self.state.clone();
        let plugin_id = self.base().instance_id();
        let mut container = container;

        Callable::from_fn("create_property_editors", move |_args| {
            if !container.is_instance_valid() {
                return Variant::nil();
            }

            let mut editors =
                property_editors::create_property_editors(&mut container, &object, &class);
            for property_editor in editors.values_mut() {
                property_changes::connect_property_changed(
                    &mut property_editor.editor,
                    &object,
                    &class,
                    plugin_id,
                );
            }

            state.borrow_mut().property_editors = editors;
            Variant::nil()
        })
        .call_deferred(&[]);
    }

    fn parse_end(&mut self, _object: Option<Gd<Object>>) {
        self.attach_pending_container();
    }

    fn parse_category(&mut self, _object: Option<Gd<Object>>, category: GString) {
        let matches = {
            let state = self.state.borrow();
            state
                .class
                .as_ref()
                .is_some_and(|class| class.category == category.to_string())
        };

        if matches {
            self.attach_pending_container();
        }
    }

    fn parse_property(
        &mut self,
        object: Option<Gd<Object>>,
        _variant_type: VariantType,
        name: GString,
        _hint: PropertyHint,
        _hint_string: GString,
        _usage: PropertyUsageFlags,
        _wide: bool,
    ) -> bool {
        object
            .and_then(|object| self.create_naughty_class(&object))
            .is_some_and(|class| class.find(&StringName::from(&name)).is_some())
    }
}

#[godot_api]
impl NaughtyEditorInspectorPlugin {
    #[func]
    fn sync_property_editors(&self) {
        let state = self.state.borrow();
        for property_editor in state.property_editors.values() {
            if property_editor.editor.is_instance_valid() {
                property_editor.editor.clone().update_property();
            }
        }
        drop(state);
        self.refresh_conditions();
    }

    #[func]
    fn refresh_conditions(&self) {
        let state = self.state.borrow();
        let (Some(object), Some(class)) = (state.object.as_ref(), state.class.as_ref()) else {
            return;
        };

        for property in &class.properties {
            let Some(property_editor) = state.property_editors.get(&property.name) else {
                continue;
            };

            if !property_editor.container.is_instance_valid() {
                continue;
            }

            let visible = property_utils::is_visible(object, property);
            if property_editor.container.is_visible() != visible {
                property_editor.container.clone().set_visible(visible);
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

        let stale = {
            let state = self.state.borrow();
            match (&state.object, &state.class) {
                (Some(inspected), Some(class)) if *inspected == object => class.is_stale(&object),
                _ => ClassDescriptor::from_object(&object).is_naughty(),
            }
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

    fn attach_pending_container(&mut self) {
        let Some(container) = self.state.borrow_mut().pending_container.take() else {
            return;
        };
        self.base_mut().add_custom_control(&container);
    }
}

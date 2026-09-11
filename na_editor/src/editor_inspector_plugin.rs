use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use godot::classes::{
    Control, EditorInspectorPlugin, EditorProperty, IEditorInspectorPlugin, VBoxContainer,
};
use godot::prelude::*;
use godot::register::info::{PropertyHint, PropertyUsageFlags};

use na_core::descriptor::ClassDescriptor;

use crate::property_utils;

#[derive(Default)]
struct InspectorState {
    class: Option<Rc<ClassDescriptor>>,
    object: Option<Gd<Object>>,
    property_editors: HashMap<StringName, Gd<EditorProperty>>,
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

        Callable::from_fn("naughty_populate_containter", move |_args| {
            property_editors::build_property_editors(
                &mut container,
                &object,
                &class,
                &state,
                plugin_id,
            );
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
        for editor in state.property_editors.values() {
            if editor.is_instance_valid() {
                editor.clone().update_property();
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
            let Some(editor) = state.property_editors.get(&property.name) else {
                continue;
            };

            if !editor.is_instance_valid() {
                continue;
            }

            let visible = property_utils::is_visible(object, property);
            if editor.is_visible() != visible {
                editor.clone().set_visible(visible);
            }
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

mod property_editors {
    use std::cell::RefCell;
    use std::rc::Rc;

    use godot::classes::{Control, EditorProperty};
    use godot::obj::InstanceId;
    use godot::prelude::*;
    use na_core::descriptor::ClassDescriptor;

    use crate::edit_action::EditAction;
    use crate::property_utils;

    use super::InspectorState;
    use super::NaughtyEditorInspectorPlugin;

    pub fn build_property_editors(
        container: &mut Gd<Control>,
        object: &Gd<Object>,
        class: &Rc<ClassDescriptor>,
        state: &Rc<RefCell<InspectorState>>,
        plugin_id: InstanceId,
    ) {
        if !container.is_instance_valid() {
            return;
        }

        let mut edit_action = EditAction::new(object);

        for property in &class.properties {
            let Some(mut property_editor) =
                property_utils::draw(container, &mut edit_action, object, property)
            else {
                continue;
            };

            connect_property_changed(&mut property_editor, object, class, plugin_id);
            state
                .borrow_mut()
                .property_editors
                .insert(property.name.clone(), property_editor);
        }

        edit_action.commit(&format!("Validate {}", class.category));
    }

    fn connect_property_changed(
        editor: &mut Gd<EditorProperty>,
        object: &Gd<Object>,
        class: &Rc<ClassDescriptor>,
        plugin_id: InstanceId,
    ) {
        let object = object.clone();
        let class = class.clone();
        let callable = Callable::from_linked_fn("naughty_property_changed", editor, move |args| {
            apply_property_change(&object, &class, args, plugin_id);
            Variant::nil()
        });

        editor.connect("property_changed", &callable);
    }

    fn apply_property_change(
        object: &Gd<Object>,
        class: &ClassDescriptor,
        args: &[&Variant],
        plugin_id: InstanceId,
    ) {
        let (Some(name), Some(value)) = (args.first(), args.get(1)) else {
            return;
        };

        let name = name.to::<StringName>();
        let mut edit_action = EditAction::new(object);
        edit_action.set_property_value(&name, value);
        property_utils::validate_properties(&mut edit_action, object, &class.properties);

        let value_was_clamped = object.get(&name) != **value;
        let mut changed_other_properties = false;
        if let Some(changed_properties) = edit_action.commit(&format!("Set {name}")) {
            changed_other_properties = changed_properties.iter().any(|property| property != &name);
        }

        if let Ok(mut plugin) = Gd::<NaughtyEditorInspectorPlugin>::try_from_instance_id(plugin_id)
        {
            let method = if value_was_clamped || changed_other_properties {
                "sync_property_editors"
            } else {
                "refresh_conditions"
            };

            plugin.call_deferred(method, &[]);
        }
    }
}

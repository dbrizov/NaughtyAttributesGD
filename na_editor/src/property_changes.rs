use std::rc::Rc;

use godot::classes::{Button, EditorProperty};
use godot::obj::InstanceId;
use godot::prelude::*;

use na_core::descriptor::ClassDescriptor;

use crate::editor_inspector_plugin::NaughtyEditorInspectorPlugin;
use crate::property_edit_action::PropertyEditAction;
use crate::property_utils;

pub fn connect_property_changed(
    editor: &mut Gd<EditorProperty>,
    object: &Gd<Object>,
    class: &Rc<ClassDescriptor>,
    plugin_id: InstanceId,
) {
    let object = object.clone();
    let class = class.clone();
    let callable = Callable::from_linked_fn("naughty_property_changed", editor, move |args| {
        if let (Some(name), Some(value)) = (args.first(), args.get(1)) {
            let from_editor = true;
            apply_property_change(
                &object,
                &class,
                &name.to::<StringName>(),
                value,
                plugin_id,
                from_editor,
            );
        }

        Variant::nil()
    });

    editor.connect("property_changed", &callable);
}

pub fn connect_revert_pressed(
    button: &mut Gd<Button>,
    object: &Gd<Object>,
    class: &Rc<ClassDescriptor>,
    name: &StringName,
    plugin_id: InstanceId,
) {
    let object = object.clone();
    let class = class.clone();
    let name = name.clone();
    let callable = Callable::from_linked_fn("naughty_revert_pressed", button, move |_args| {
        if let Some(property) = class.find(&name) {
            let default_value = property.default_value.clone();
            let from_editor = false;
            apply_property_change(
                &object,
                &class,
                &name,
                &default_value,
                plugin_id,
                from_editor,
            );
        }

        Variant::nil()
    });

    button.connect("pressed", &callable);
}

fn apply_property_change(
    object: &Gd<Object>,
    class: &ClassDescriptor,
    name: &StringName,
    value: &Variant,
    plugin_id: InstanceId,
    from_editor: bool,
) {
    let mut edit_action = PropertyEditAction::new(object);
    edit_action.set_property_value(name, value);
    property_utils::validate_properties(&mut edit_action, object, &class.properties);

    let value_was_clamped = object.get(name) != *value;
    let mut changed_other_properties = false;
    if let Some(changed_properties) = edit_action.commit(&format!("Set {name}")) {
        changed_other_properties = changed_properties.iter().any(|property| property != name);
    }

    if let Ok(mut plugin) = Gd::<NaughtyEditorInspectorPlugin>::try_from_instance_id(plugin_id) {
        let method = if !from_editor || value_was_clamped || changed_other_properties {
            "sync_property_editors"
        } else {
            "refresh_property_editors"
        };

        plugin.call_deferred(method, &[]);
    }
}

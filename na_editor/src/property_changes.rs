use std::rc::Rc;

use godot::classes::EditorProperty;
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
    let mut edit_action = PropertyEditAction::new(object);
    edit_action.set_property_value(&name, value);
    property_utils::validate_properties(&mut edit_action, object, &class.properties);

    let value_was_clamped = object.get(&name) != **value;
    let mut changed_other_properties = false;
    if let Some(changed_properties) = edit_action.commit(&format!("Set {name}")) {
        changed_other_properties = changed_properties.iter().any(|property| property != &name);
    }

    if let Ok(mut plugin) = Gd::<NaughtyEditorInspectorPlugin>::try_from_instance_id(plugin_id) {
        let method = if value_was_clamped || changed_other_properties {
            "sync_property_editors"
        } else {
            "refresh_conditions"
        };

        plugin.call_deferred(method, &[]);
    }
}

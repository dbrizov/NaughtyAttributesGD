use std::collections::HashMap;

use godot::classes::{Control, EditorInspector, EditorProperty};
use godot::prelude::*;

use na_core::descriptor::{ClassDescriptor, PropertyDescriptor};
use na_logging::na_error;

use crate::property_edit_action::PropertyEditAction;
use crate::property_utils;

/// Returns `None` if Godot has no editor for the property.
fn create_property_editor(
    container: &mut Gd<Control>,
    edit_action: &mut PropertyEditAction,
    object: &Gd<Object>,
    property: &PropertyDescriptor,
) -> Option<Gd<EditorProperty>> {
    let visible = property_utils::is_visible(object, property);
    if visible {
        property_utils::validate_property(edit_action, object, property);
    }

    let editor = EditorInspector::instantiate_property_editor(
        object,
        property.variant_type,
        &GString::from(&property.name),
        property.hint,
        &property.hint_string,
        property.usage.ord() as u32,
    );

    let Some(mut editor) = editor else {
        na_error!("no property editor for '{}'", property.name);
        return None;
    };

    editor.set_label(&property_utils::capitalize_name(&property.name));
    editor.set_visible(visible);
    container.add_child(&editor);
    editor.set_object_and_property(object, &property.name);
    editor.update_property();

    Some(editor)
}

/// Returns the created editors keyed by property name. Properties Godot has no editor for are left out.
pub fn create_property_editors(
    container: &mut Gd<Control>,
    object: &Gd<Object>,
    class: &ClassDescriptor,
) -> HashMap<StringName, Gd<EditorProperty>> {
    let mut edit_action = PropertyEditAction::new(object);
    let mut editors = HashMap::new();

    for property in &class.properties {
        if let Some(editor) = create_property_editor(container, &mut edit_action, object, property)
        {
            editors.insert(property.name.clone(), editor);
        }
    }

    edit_action.commit(&format!("Validate {}", class.category));
    editors
}

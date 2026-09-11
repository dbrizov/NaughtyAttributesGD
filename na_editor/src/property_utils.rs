use godot::classes::{Control, EditorInspector, EditorProperty};
use godot::prelude::*;

use na_core::attributes::meta::{MetaAttribute, show_if};
use na_core::descriptor::{self, PropertyDescriptor};
use na_logging::na_error;

use crate::attribute_registry;
use crate::edit_action::EditAction;

/// Returns `None` if Godot has no editor for the property.
pub fn draw(
    container: &mut Gd<Control>,
    edit_action: &mut EditAction,
    object: &Gd<Object>,
    property: &PropertyDescriptor,
) -> Option<Gd<EditorProperty>> {
    let visible = is_visible(object, property);
    if visible {
        validate_property(edit_action, object, property);
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

    editor.set_label(&capitalize_name(&property.name));
    editor.set_visible(visible);
    container.add_child(&editor);
    editor.set_object_and_property(object, &property.name);
    editor.update_property();

    Some(editor)
}

/// A condition that fails to evaluate counts as visible.
pub fn is_visible(object: &Gd<Object>, property: &PropertyDescriptor) -> bool {
    property.metas.iter().all(|meta| match meta {
        MetaAttribute::ShowIf(condition) => condition.is_visible(object).unwrap_or_else(|error| {
            na_error!(
                "{}.{} - {}: {error}",
                descriptor::script_path(object),
                property.name,
                show_if::KEY
            );
            true
        }),
    })
}

pub fn validate_property(
    edit_action: &mut EditAction,
    object: &Gd<Object>,
    property: &PropertyDescriptor,
) {
    for attribute in &property.validators {
        match attribute_registry::get_validator(attribute).validate(object, property) {
            Ok(Some(value)) => edit_action.set_property_value(&property.name, &value),
            Ok(None) => {}
            Err(error) => {
                na_error!(
                    "{}.{} - {}: {error}",
                    descriptor::script_path(object),
                    property.name,
                    attribute.key()
                );
            }
        }
    }
}

pub fn validate_properties(
    edit_action: &mut EditAction,
    object: &Gd<Object>,
    properties: &[PropertyDescriptor],
) {
    for property in properties {
        if is_visible(object, property) {
            validate_property(edit_action, object, property);
        }
    }
}

fn capitalize_name(name: &StringName) -> GString {
    GString::from(name).capitalize()
}

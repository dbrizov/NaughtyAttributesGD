use godot::classes::EditorProperty;
use godot::prelude::*;

use na_core::attributes::meta::{MetaAttribute, show_if};
use na_core::descriptor::{self, PropertyDescriptor};
use na_logging::na_error;

use crate::attribute_registry;
use crate::property_undo_redo::PropertyEditAction;

/// Returns `None` when the property has no drawer, or when its drawer failed.
pub fn create_drawer_editor(
    object: &Gd<Object>,
    property: &PropertyDescriptor,
) -> Option<Gd<EditorProperty>> {
    let attribute = property.drawer.as_ref()?;
    let drawer = attribute_registry::get_drawer(attribute);
    match drawer.create_editor(object, property) {
        Ok(editor) => Some(editor),
        Err(error) => {
            na_error!(
                "{}.{} - {}: {error}",
                descriptor::get_script_path(object),
                property.name,
                attribute.get_key()
            );
            None
        }
    }
}

pub fn is_visible(object: &Gd<Object>, property: &PropertyDescriptor) -> bool {
    property.metas.iter().all(|meta| match meta {
        MetaAttribute::ShowIf(condition) => condition.is_visible(object).unwrap_or_else(|error| {
            na_error!(
                "{}.{} - {}: {error}",
                descriptor::get_script_path(object),
                property.name,
                show_if::KEY
            );
            true
        }),
    })
}

pub fn validate_property(
    edit_action: &mut PropertyEditAction,
    object: &Gd<Object>,
    property: &PropertyDescriptor,
) {
    for attribute in &property.validators {
        let validator = attribute_registry::get_validator(attribute);
        match validator.validate(object, property) {
            Ok(Some(value)) => edit_action.set_property_value(&property.name, &value),
            Ok(None) => {}
            Err(error) => {
                na_error!(
                    "{}.{} - {}: {error}",
                    descriptor::get_script_path(object),
                    property.name,
                    attribute.get_key()
                );
            }
        }
    }
}

pub fn validate_properties(
    edit_action: &mut PropertyEditAction,
    object: &Gd<Object>,
    properties: &[PropertyDescriptor],
) {
    for property in properties {
        if is_visible(object, property) {
            validate_property(edit_action, object, property);
        }
    }
}

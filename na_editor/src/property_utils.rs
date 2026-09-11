use godot::prelude::*;

use na_core::attributes::meta::{MetaAttribute, show_if};
use na_core::descriptor::{self, PropertyDescriptor};
use na_logging::na_error;

use crate::attribute_registry;
use crate::property_edit_action::PropertyEditAction;

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
    edit_action: &mut PropertyEditAction,
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

pub fn capitalize_name(name: &StringName) -> GString {
    GString::from(name).capitalize()
}

use godot::classes::EditorProperty;
use godot::prelude::*;

use na_core::attributes::meta::MetaAttribute::{
    self, DisableIf, EnableIf, HideIf, Label, OnValueChanged, ReadOnly, ShowIf,
};
use na_core::descriptor::{self, PropertyDescriptor};
use na_logging::na_error;

use crate::attribute_registry;
use crate::property_undo_redo::PropertyEditAction;
use crate::variant_utils;

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

pub fn get_label(property: &PropertyDescriptor) -> Option<&str> {
    property
        .metas
        .iter()
        .filter_map(|meta| match meta {
            Label(attribute) => Some(attribute.text.as_str()),
            ShowIf(_) | HideIf(_) | EnableIf(_) | DisableIf(_) | ReadOnly | OnValueChanged(_) => {
                None
            }
        })
        .next_back()
}

pub fn is_visible(object: &Gd<Object>, property: &PropertyDescriptor) -> bool {
    is_meta_query_satisfied(object, property, |meta, obj| match meta {
        ShowIf(attribute) => attribute.is_visible(obj),
        HideIf(attribute) => attribute.is_visible(obj),
        EnableIf(_) | DisableIf(_) | ReadOnly | Label(_) | OnValueChanged(_) => Ok(true),
    })
}

pub fn is_enabled(object: &Gd<Object>, property: &PropertyDescriptor) -> bool {
    is_meta_query_satisfied(object, property, |meta, obj| match meta {
        EnableIf(attribute) => attribute.is_enabled(obj),
        DisableIf(attribute) => attribute.is_enabled(obj),
        ReadOnly => Ok(false),
        ShowIf(_) | HideIf(_) | Label(_) | OnValueChanged(_) => Ok(true),
    })
}

fn is_meta_query_satisfied(
    object: &Gd<Object>,
    property: &PropertyDescriptor,
    query: impl Fn(&MetaAttribute, &Gd<Object>) -> Result<bool, String>,
) -> bool {
    let mut satisfied = true;

    for meta in &property.metas {
        satisfied &= query(meta, object).unwrap_or_else(|error| {
            na_error!(
                "{}.{} - {}: {error}",
                descriptor::get_script_path(object),
                property.name,
                meta.get_key()
            );
            true
        });
    }

    satisfied
}

pub fn clamp_property(
    object: &Gd<Object>,
    property: &PropertyDescriptor,
    min: f64,
    max: f64,
) -> Result<Option<Variant>, String> {
    let value = object.get(&property.name);
    let clamped_value = variant_utils::clamp(&value, min, max)?;
    let is_clamped = clamped_value != value;

    Ok(is_clamped.then_some(clamped_value))
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

pub fn call_value_changed_callbacks(
    object: &mut Gd<Object>,
    property: &PropertyDescriptor,
    old_value: &Variant,
    new_value: &Variant,
) {
    for meta in &property.metas {
        let result = match meta {
            OnValueChanged(attribute) => attribute.call(object, old_value, new_value),
            ShowIf(_) | HideIf(_) | EnableIf(_) | DisableIf(_) | ReadOnly | Label(_) => Ok(()),
        };

        if let Err(error) = result {
            na_error!(
                "{}.{} - {}: {error}",
                descriptor::get_script_path(object),
                property.name,
                meta.get_key()
            );
        }
    }
}

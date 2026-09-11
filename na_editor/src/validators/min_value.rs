use godot::prelude::*;

use na_core::attributes::validator::min_value::{KEY, MinValue};
use na_core::descriptor::{self, PropertyDescriptor};
use na_logging::na_error;

use crate::validators::IValidator;
use crate::variant_utils;

impl IValidator for MinValue {
    fn validate(&self, object: &Gd<Object>, property: &PropertyDescriptor) -> Option<Variant> {
        let min = match self.min_value.evaluate_number(object) {
            Ok(min) => min,
            Err(error) => {
                na_error!(
                    "{}.{} - {KEY}: {error}",
                    descriptor::script_path(object),
                    property.name
                );
                return None;
            }
        };

        let value = object.get(&property.name);
        let clamped = variant_utils::clamp(&value, min, f64::INFINITY)?;
        (clamped != value).then_some(clamped)
    }
}

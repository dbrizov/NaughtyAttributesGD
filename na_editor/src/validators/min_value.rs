use godot::prelude::*;

use na_core::attributes;
use na_core::attributes::validator::min_value::{KEY, MinValue};
use na_core::descriptor::PropertyDescriptor;

use crate::validators::IValidator;
use crate::variant_utils;

impl IValidator for MinValue {
    fn validate(&self, object: &Gd<Object>, property: &PropertyDescriptor) -> Option<Variant> {
        let min = match self.bound.resolve(object) {
            Ok(min) => min,
            Err(error) => {
                attributes::warn(object, &property.name, KEY, &error);
                return None;
            }
        };

        let value = object.get(&property.name);
        let clamped = variant_utils::clamp(&value, min, f64::INFINITY)?;
        (clamped != value).then_some(clamped)
    }
}

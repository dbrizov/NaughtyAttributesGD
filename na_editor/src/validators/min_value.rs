use godot::prelude::*;

use na_core::attributes::validator::min_value::MinValue;
use na_core::descriptor::PropertyDescriptor;

use crate::validators::IValidator;
use crate::variant_utils;

impl IValidator for MinValue {
    fn validate(
        &self,
        object: &Gd<Object>,
        property: &PropertyDescriptor,
    ) -> Result<Option<Variant>, String> {
        let min = self.min_value.evaluate_number(object)?;
        let value = object.get(&property.name);
        let clamped_value = variant_utils::clamp(&value, min, f64::INFINITY)?;
        let is_clamped = clamped_value != value;
        Ok(is_clamped.then_some(clamped_value))
    }
}

use godot::prelude::*;

use na_core::attributes::validator::min_value::MinValue;
use na_core::descriptor::PropertyDescriptor;

use crate::property_utils;
use crate::validators::{IValidator, Validation};

impl IValidator for MinValue {
    fn validate(
        &self,
        object: &Gd<Object>,
        property: &PropertyDescriptor,
    ) -> Result<Validation, String> {
        let (min, max) = self.get_bounds(object)?;
        property_utils::clamp_property(object, property, min, max)
    }
}

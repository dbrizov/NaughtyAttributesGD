use godot::prelude::*;

use na_core::attributes::validator::max_value::MaxValue;
use na_core::descriptor::PropertyDescriptor;

use crate::property_utils;
use crate::validators::IValidator;

impl IValidator for MaxValue {
    fn validate(
        &self,
        object: &Gd<Object>,
        property: &PropertyDescriptor,
    ) -> Result<Option<Variant>, String> {
        let (min, max) = self.get_bounds(object)?;
        property_utils::clamp_property(object, property, min, max)
    }
}

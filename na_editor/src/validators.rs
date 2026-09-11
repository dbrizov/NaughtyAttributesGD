mod min_value;

use godot::prelude::*;

use na_core::descriptor::PropertyDescriptor;

pub trait IValidator {
    /// Returns the corrected value, `None` if the value is already valid.
    fn validate(
        &self,
        object: &Gd<Object>,
        property: &PropertyDescriptor,
    ) -> Result<Option<Variant>, String>;
}

mod max_value;
mod min_value;
mod require;

use godot::prelude::*;

use na_core::descriptor::PropertyDescriptor;

pub enum Validation {
    Valid,
    Corrected(Variant),
    Rejected(String),
}

pub trait IValidator {
    fn validate(
        &self,
        object: &Gd<Object>,
        property: &PropertyDescriptor,
    ) -> Result<Validation, String>;
}

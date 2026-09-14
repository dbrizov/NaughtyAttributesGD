use godot::prelude::*;

use na_core::attributes::validator::require::Require;
use na_core::descriptor::PropertyDescriptor;

use crate::validators::{IValidator, Validation};

impl IValidator for Require {
    fn validate(
        &self,
        object: &Gd<Object>,
        property: &PropertyDescriptor,
    ) -> Result<Validation, String> {
        if self.is_satisfied(object)? {
            return Ok(Validation::Valid);
        }

        Ok(Validation::Rejected(
            self.get_message(&property.name.to_string()),
        ))
    }
}

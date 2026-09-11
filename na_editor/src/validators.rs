mod min_value;

use godot::prelude::*;

use na_core::descriptor::PropertyDescriptor;

pub trait IValidator {
    fn validate(&self, object: &Gd<Object>, property: &PropertyDescriptor) -> Option<Variant>;
}

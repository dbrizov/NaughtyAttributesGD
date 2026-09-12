use na_core::attributes::decorator::DecoratorAttribute;
use na_core::attributes::validator::ValidatorAttribute;

use crate::decorators::IDecorator;
use crate::validators::IValidator;

pub fn get_decorator(attribute: &DecoratorAttribute) -> &dyn IDecorator {
    match attribute {
        DecoratorAttribute::InfoBox(info_box) => info_box,
    }
}

pub fn get_validator(attribute: &ValidatorAttribute) -> &dyn IValidator {
    match attribute {
        ValidatorAttribute::MinValue(min_value) => min_value,
    }
}

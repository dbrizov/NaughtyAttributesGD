use na_core::attributes::validator::ValidatorAttribute;

use crate::validators::IValidator;

pub fn get_validator(attribute: &ValidatorAttribute) -> &dyn IValidator {
    match attribute {
        ValidatorAttribute::MinValue(min_value) => min_value,
    }
}

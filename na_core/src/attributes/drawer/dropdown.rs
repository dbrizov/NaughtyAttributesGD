use godot::global::type_string;
use godot::prelude::*;

use crate::attributes::parse_context::ParseContext;
use crate::expressions::options_expression::{DropdownOption, OptionsExpression};

pub const KEY: &str = "dropdown";

#[derive(Clone)]
pub struct Dropdown {
    options: OptionsExpression,
    variant_type: VariantType,
}

impl Dropdown {
    pub fn parse(raw_args: &str, context: &ParseContext) -> Result<Self, String> {
        Ok(Self {
            options: OptionsExpression::parse(raw_args, context)?,
            variant_type: context.variant_type,
        })
    }

    pub fn evaluate_options(&self, object: &Gd<Object>) -> Result<Vec<DropdownOption>, String> {
        let options = self.options.evaluate(object)?;
        for option in &options {
            if !is_assignable(&option.value, self.variant_type) {
                return Err(format!(
                    "option '{}' is {}, not {}",
                    option.label,
                    type_string(option.value.get_type().ord() as i64),
                    type_string(self.variant_type.ord() as i64)
                ));
            }
        }

        Ok(options)
    }
}

fn is_assignable(value: &Variant, variant_type: VariantType) -> bool {
    let value_type = value.get_type();
    if value_type == variant_type {
        return true;
    }

    match variant_type {
        VariantType::INT | VariantType::FLOAT => {
            matches!(value_type, VariantType::INT | VariantType::FLOAT)
        }
        VariantType::STRING | VariantType::STRING_NAME => {
            matches!(value_type, VariantType::STRING | VariantType::STRING_NAME)
        }
        VariantType::OBJECT => value_type == VariantType::NIL,
        _ => false,
    }
}

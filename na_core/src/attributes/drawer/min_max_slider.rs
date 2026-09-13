use godot::prelude::*;

use crate::annotation::split_args;
use crate::attributes::ParseContext;
use crate::expressions::number_expression::NumberExpression;

pub const KEY: &str = "min_max_slider";

pub const SUPPORTED_TYPES: &[VariantType] = &[VariantType::VECTOR2, VariantType::VECTOR2I];

#[derive(Clone)]
pub struct MinMaxSlider {
    min_value: NumberExpression,
    max_value: NumberExpression,
}

impl MinMaxSlider {
    pub fn parse(raw_args: &str, context: &ParseContext) -> Result<Self, String> {
        if !SUPPORTED_TYPES.contains(&context.variant_type) {
            return Err("can be used only on Vector2 and Vector2i properties".to_string());
        }

        let args = split_args(raw_args);
        let [min_value, max_value] = args.as_slice() else {
            return Err("expected a minimum and a maximum".to_string());
        };

        Ok(Self {
            min_value: NumberExpression::parse(min_value, context)?,
            max_value: NumberExpression::parse(max_value, context)?,
        })
    }

    pub fn evaluate_bounds(&self, object: &Gd<Object>) -> Result<(f64, f64), String> {
        let min = self.min_value.evaluate(object)?;
        let max = self.max_value.evaluate(object)?;
        if min > max {
            return Err(format!(
                "the minimum {min} is greater than the maximum {max}"
            ));
        }

        Ok((min, max))
    }
}

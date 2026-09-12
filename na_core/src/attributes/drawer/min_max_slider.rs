use godot::prelude::*;

use crate::annotation::split_args;
use crate::attributes::ParseContext;
use crate::expression::Expression;

pub const KEY: &str = "min_max_slider";

pub const SUPPORTED_TYPES: &[VariantType] = &[VariantType::VECTOR2, VariantType::VECTOR2I];

#[derive(Clone)]
pub struct MinMaxSlider {
    pub min_value: Expression,
    pub max_value: Expression,
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
            min_value: compile(min_value, context)?,
            max_value: compile(max_value, context)?,
        })
    }

    pub fn evaluate_bounds(&self, object: &Gd<Object>) -> Result<(f64, f64), String> {
        let min = self.min_value.evaluate_number(object)?;
        let max = self.max_value.evaluate_number(object)?;
        if min > max {
            return Err(format!(
                "the minimum {min} is greater than the maximum {max}"
            ));
        }

        Ok((min, max))
    }
}

fn compile(source: &str, context: &ParseContext) -> Result<Expression, String> {
    if source.is_empty() {
        return Err("expected a numeric expression".to_string());
    }

    let expression = Expression::compile(source, context.constants);
    if !expression.is_valid() {
        return Err(expression.get_error().to_string());
    }

    Ok(expression)
}

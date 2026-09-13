use godot::classes::Object;
use godot::prelude::*;

use crate::attributes::parse_context::ParseContext;
use crate::expressions::number_expression::NumberExpression;

pub const KEY: &str = "min_value";

pub struct MinValue {
    min_value: NumberExpression,
}

impl MinValue {
    pub fn parse(raw_args: &str, context: &ParseContext) -> Result<Self, String> {
        NumberExpression::parse(raw_args, context).map(|min_value| Self { min_value })
    }

    pub fn get_bounds(&self, object: &Gd<Object>) -> Result<(f64, f64), String> {
        self.min_value
            .evaluate(object)
            .map(|min| (min, f64::INFINITY))
    }
}

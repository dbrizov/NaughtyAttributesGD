use godot::classes::Object;
use godot::prelude::*;

use crate::attributes::parse_context::ParseContext;
use crate::expressions::number_expression::NumberExpression;

pub const KEY: &str = "max_value";

pub struct MaxValue {
    max_value: NumberExpression,
}

impl MaxValue {
    pub fn parse(raw_args: &str, context: &ParseContext) -> Result<Self, String> {
        NumberExpression::parse(raw_args, context).map(|max_value| Self { max_value })
    }

    pub fn get_bounds(&self, object: &Gd<Object>) -> Result<(f64, f64), String> {
        self.max_value
            .evaluate(object)
            .map(|max| (f64::NEG_INFINITY, max))
    }
}

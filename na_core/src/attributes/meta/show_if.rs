use godot::classes::Object;
use godot::prelude::*;

use crate::attributes::ParseContext;
use crate::expressions::bool_expression::BoolExpression;

pub const KEY: &str = "show_if";

pub struct ShowIf {
    expression: BoolExpression,
}

impl ShowIf {
    pub fn parse(raw_args: &str, context: &ParseContext) -> Result<Self, String> {
        BoolExpression::parse(raw_args, context).map(|expression| Self { expression })
    }

    pub fn is_visible(&self, object: &Gd<Object>) -> Result<bool, String> {
        self.expression.evaluate(object)
    }
}

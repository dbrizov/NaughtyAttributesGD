use godot::classes::Object;
use godot::prelude::*;

use crate::attributes::parse_context::ParseContext;
use crate::expressions::bool_expression::BoolExpression;

pub const KEY: &str = "require";

pub struct Require {
    expression: BoolExpression,
}

impl Require {
    pub fn parse(raw_args: &str, context: &ParseContext) -> Result<Self, String> {
        BoolExpression::parse(raw_args, context).map(|expression| Self { expression })
    }

    pub fn is_satisfied(&self, object: &Gd<Object>) -> Result<bool, String> {
        self.expression.evaluate(object)
    }

    pub fn get_message(&self, property_name: &str) -> String {
        format!(
            "{property_name} is not valid: {}",
            self.expression.get_text()
        )
    }
}

use godot::classes::Object;
use godot::prelude::*;

use crate::attributes::ParseContext;
use crate::expression::Expression;

pub struct Condition {
    expression: Expression,
}

impl Condition {
    pub fn parse(raw_args: &str, context: &ParseContext) -> Result<Self, String> {
        let source = raw_args.trim();
        if source.is_empty() {
            return Err("expected a boolean expression".to_string());
        }

        let expression = Expression::compile(source, context.constants);
        if !expression.is_valid() {
            return Err(expression.get_error().to_string());
        }

        Ok(Self { expression })
    }

    pub fn is_satisfied(&self, object: &Gd<Object>) -> Result<bool, String> {
        self.expression.evaluate_bool(object)
    }
}

use godot::classes::Object;
use godot::prelude::*;

use crate::attributes::ParseContext;
use crate::expression::Expression;

pub const KEY: &str = "show_if";

pub struct ShowIf {
    condition: Expression,
}

impl ShowIf {
    pub fn parse(raw_args: &str, context: &ParseContext) -> Result<Self, String> {
        let source = raw_args.trim();
        if source.is_empty() {
            return Err("expected a boolean expression".to_string());
        }

        let condition = Expression::compile(source, context.constants);
        if !condition.is_valid() {
            return Err(condition.get_error().to_string());
        }

        Ok(Self { condition })
    }

    pub fn is_visible(&self, object: &Gd<Object>) -> Result<bool, String> {
        self.condition.evaluate_bool(object)
    }
}

use godot::classes::Object;
use godot::prelude::*;

use crate::attributes::ParseContext;
use crate::expression::Expression;

pub const KEY: &str = "show_if";

pub struct ShowIf {
    condition: Expression,
}

impl ShowIf {
    pub fn parse(raw_args: &str, context: &ParseContext) -> Option<Self> {
        let source = raw_args.trim();
        if source.is_empty() {
            context.warn(KEY, "expected a boolean expression");
            return None;
        }

        let condition = Expression::compile(source, context.constants);
        if !condition.is_valid() {
            context.warn(KEY, condition.error());
        }

        Some(Self { condition })
    }

    pub fn is_visible(&self, object: &Gd<Object>) -> bool {
        match self.condition.evaluate_bool(object) {
            Ok(visible) => visible,
            Err(error) => {
                godot_warn!(
                    "{} {} '{}' - {}",
                    crate::LOG_PREFIX,
                    KEY,
                    self.condition.expression_text(),
                    error
                );
                true
            }
        }
    }
}

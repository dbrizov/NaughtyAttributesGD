use godot::classes::Object;
use godot::prelude::*;

use na_logging::na_error;

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
            na_error!(
                "{}.{} - {KEY}: expected a boolean expression",
                context.script_path,
                context.property
            );
            return None;
        }

        let condition = Expression::compile(source, context.constants);
        if !condition.is_valid() {
            na_error!(
                "{}.{} - {KEY}: {}",
                context.script_path,
                context.property,
                condition.error()
            );
        }

        Some(Self { condition })
    }

    pub fn is_visible(&self, object: &Gd<Object>) -> bool {
        match self.condition.evaluate_bool(object) {
            Ok(visible) => visible,
            Err(error) => {
                na_error!("{KEY} '{}' - {error}", self.condition.expression_text());
                true
            }
        }
    }
}

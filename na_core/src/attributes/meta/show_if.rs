use godot::classes::Object;
use godot::prelude::*;

use crate::attributes::ParseContext;
use crate::condition::Condition;
use crate::member;

pub const KEY: &str = "show_if";

pub struct ShowIf {
    condition: Condition,
}

impl ShowIf {
    pub fn parse(raw_args: &str, context: &ParseContext) -> Option<Self> {
        let source = raw_args.trim();
        if source.is_empty() {
            context.warn(KEY, "expected a boolean expression");
            return None;
        }

        let condition = Condition::compile(source, context.constants);
        if !condition.is_valid() {
            context.warn(KEY, condition.error());
        }

        Some(Self { condition })
    }

    pub fn is_visible(&self, object: &Gd<Object>) -> bool {
        match self.condition.evaluate(object) {
            Ok(visible) => visible,
            Err(error) => {
                godot_warn!(
                    "{} {} '{}' - {}{}",
                    crate::LOG_PREFIX,
                    KEY,
                    self.condition.expression_text(),
                    error,
                    tool_hint(object, self.condition.expression_text())
                );
                true
            }
        }
    }
}

fn tool_hint(object: &Gd<Object>, expression_text: &str) -> &'static str {
    if !expression_text.contains('(') {
        return "";
    }

    member::tool_hint(object)
}

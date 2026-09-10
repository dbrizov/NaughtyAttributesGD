use godot::classes::Object;
use godot::prelude::*;

use crate::condition::Condition;
use crate::parse_context::ParseContext;

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
                    self.condition.source(),
                    error,
                    tool_hint(object, self.condition.source())
                );
                true
            }
        }
    }
}

fn tool_hint(object: &Gd<Object>, source: &str) -> &'static str {
    if !source.contains('(') {
        return "";
    }

    let is_tool = object
        .get("script")
        .try_to::<Gd<godot::classes::Script>>()
        .map(|script| script.is_tool())
        .unwrap_or(true);

    if is_tool {
        ""
    } else {
        " (calling a method needs @tool on the script)"
    }
}

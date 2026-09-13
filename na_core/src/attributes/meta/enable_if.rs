use godot::classes::Object;
use godot::prelude::*;

use crate::attributes::ParseContext;
use crate::attributes::meta::condition::Condition;

pub const KEY: &str = "enable_if";

pub struct EnableIf {
    condition: Condition,
}

impl EnableIf {
    pub fn parse(raw_args: &str, context: &ParseContext) -> Result<Self, String> {
        Condition::parse(raw_args, context).map(|condition| Self { condition })
    }

    pub fn is_enabled(&self, object: &Gd<Object>) -> Result<bool, String> {
        self.condition.is_satisfied(object)
    }
}

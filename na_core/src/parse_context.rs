use godot::prelude::*;

use crate::LOG_PREFIX;

pub struct ParseContext<'a> {
    pub script_path: &'a str,
    pub property: &'a str,
    pub constants: &'a VarDictionary,
}

impl ParseContext<'_> {
    pub fn warn(&self, key: &str, message: &str) {
        godot_warn!(
            "{} {}.{} - {}: {}",
            LOG_PREFIX,
            self.script_path,
            self.property,
            key,
            message
        );
    }
}

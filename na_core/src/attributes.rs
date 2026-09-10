pub mod meta;

use godot::prelude::*;

use crate::LOG_PREFIX;
use meta::MetaAttribute;

pub enum NaughtyAttribute {
    Meta(MetaAttribute),
}

impl NaughtyAttribute {
    pub fn parse(key: &str, raw_args: &str, context: &ParseContext) -> Option<Self> {
        MetaAttribute::parse(key, raw_args, context).map(Self::Meta)
    }
}

pub fn is_known_key(key: &str) -> bool {
    MetaAttribute::is_known_key(key)
}

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

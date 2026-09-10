pub mod meta;

use crate::parse_context::ParseContext;
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

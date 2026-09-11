pub mod show_if;

use crate::attributes::ParseContext;
use show_if::ShowIf;

pub enum MetaAttribute {
    ShowIf(ShowIf),
}

impl MetaAttribute {
    pub fn parse(key: &str, raw_args: &str, context: &ParseContext) -> Result<Self, String> {
        match key {
            show_if::KEY => ShowIf::parse(raw_args, context).map(Self::ShowIf),
            _ => Err("unknown attribute".to_string()),
        }
    }

    pub fn is_known_key(key: &str) -> bool {
        matches!(key, show_if::KEY)
    }
}

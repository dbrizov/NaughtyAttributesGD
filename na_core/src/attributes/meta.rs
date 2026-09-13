pub mod condition;
pub mod hide_if;
pub mod show_if;

use crate::attributes::ParseContext;
use hide_if::HideIf;
use show_if::ShowIf;

pub enum MetaAttribute {
    ShowIf(ShowIf),
    HideIf(HideIf),
}

impl MetaAttribute {
    pub fn parse(key: &str, raw_args: &str, context: &ParseContext) -> Result<Self, String> {
        match key {
            show_if::KEY => ShowIf::parse(raw_args, context).map(Self::ShowIf),
            hide_if::KEY => HideIf::parse(raw_args, context).map(Self::HideIf),
            _ => Err("unknown attribute".to_string()),
        }
    }

    pub fn is_known_key(key: &str) -> bool {
        matches!(key, show_if::KEY | hide_if::KEY)
    }

    pub fn get_key(&self) -> &'static str {
        match self {
            Self::ShowIf(_) => show_if::KEY,
            Self::HideIf(_) => hide_if::KEY,
        }
    }
}

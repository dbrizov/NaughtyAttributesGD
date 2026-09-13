pub mod condition;
pub mod disable_if;
pub mod enable_if;
pub mod hide_if;
pub mod read_only;
pub mod show_if;

use crate::attributes::ParseContext;
use disable_if::DisableIf;
use enable_if::EnableIf;
use hide_if::HideIf;
use show_if::ShowIf;

pub enum MetaAttribute {
    ShowIf(ShowIf),
    HideIf(HideIf),
    EnableIf(EnableIf),
    DisableIf(DisableIf),
    ReadOnly,
}

impl MetaAttribute {
    pub fn parse(key: &str, raw_args: &str, context: &ParseContext) -> Result<Self, String> {
        match key {
            show_if::KEY => ShowIf::parse(raw_args, context).map(Self::ShowIf),
            hide_if::KEY => HideIf::parse(raw_args, context).map(Self::HideIf),
            enable_if::KEY => EnableIf::parse(raw_args, context).map(Self::EnableIf),
            disable_if::KEY => DisableIf::parse(raw_args, context).map(Self::DisableIf),
            read_only::KEY => Ok(Self::ReadOnly),
            _ => Err("unknown attribute".to_string()),
        }
    }

    pub fn is_known_key(key: &str) -> bool {
        matches!(
            key,
            show_if::KEY | hide_if::KEY | enable_if::KEY | disable_if::KEY | read_only::KEY
        )
    }

    pub fn get_key(&self) -> &'static str {
        match self {
            Self::ShowIf(_) => show_if::KEY,
            Self::HideIf(_) => hide_if::KEY,
            Self::EnableIf(_) => enable_if::KEY,
            Self::DisableIf(_) => disable_if::KEY,
            Self::ReadOnly => read_only::KEY,
        }
    }
}

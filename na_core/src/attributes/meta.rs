pub mod disable_if;
pub mod enable_if;
pub mod hide_if;
pub mod label;
pub mod on_value_changed;
pub mod read_only;
pub mod show_if;

use crate::attributes::parse_context::ParseContext;
use disable_if::DisableIf;
use enable_if::EnableIf;
use hide_if::HideIf;
use label::Label;
use on_value_changed::OnValueChanged;
use show_if::ShowIf;

pub enum MetaAttribute {
    ShowIf(ShowIf),
    HideIf(HideIf),
    EnableIf(EnableIf),
    DisableIf(DisableIf),
    ReadOnly,
    Label(Label),
    OnValueChanged(OnValueChanged),
}

impl MetaAttribute {
    pub fn parse(key: &str, raw_args: &str, context: &ParseContext) -> Result<Self, String> {
        match key {
            show_if::KEY => ShowIf::parse(raw_args, context).map(Self::ShowIf),
            hide_if::KEY => HideIf::parse(raw_args, context).map(Self::HideIf),
            enable_if::KEY => EnableIf::parse(raw_args, context).map(Self::EnableIf),
            disable_if::KEY => DisableIf::parse(raw_args, context).map(Self::DisableIf),
            read_only::KEY => Ok(Self::ReadOnly),
            label::KEY => Label::parse(raw_args).map(Self::Label),
            on_value_changed::KEY => OnValueChanged::parse(raw_args).map(Self::OnValueChanged),
            _ => Err("unknown attribute".to_string()),
        }
    }

    pub fn is_known_key(key: &str) -> bool {
        matches!(
            key,
            show_if::KEY
                | hide_if::KEY
                | enable_if::KEY
                | disable_if::KEY
                | read_only::KEY
                | label::KEY
                | on_value_changed::KEY
        )
    }

    pub fn get_key(&self) -> &'static str {
        match self {
            Self::ShowIf(_) => show_if::KEY,
            Self::HideIf(_) => hide_if::KEY,
            Self::EnableIf(_) => enable_if::KEY,
            Self::DisableIf(_) => disable_if::KEY,
            Self::ReadOnly => read_only::KEY,
            Self::Label(_) => label::KEY,
            Self::OnValueChanged(_) => on_value_changed::KEY,
        }
    }
}

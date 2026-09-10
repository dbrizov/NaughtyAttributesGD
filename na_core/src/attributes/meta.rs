pub mod show_if;

use godot::classes::Object;
use godot::prelude::*;

use crate::attributes::ParseContext;
use show_if::ShowIf;

pub enum MetaAttribute {
    ShowIf(ShowIf),
}

impl MetaAttribute {
    pub fn parse(key: &str, raw_args: &str, context: &ParseContext) -> Option<Self> {
        match key {
            show_if::KEY => ShowIf::parse(raw_args, context).map(Self::ShowIf),
            _ => None,
        }
    }

    pub fn is_known_key(key: &str) -> bool {
        matches!(key, show_if::KEY)
    }
}

pub fn is_visible(metas: &[MetaAttribute], object: &Gd<Object>) -> bool {
    metas.iter().all(|meta| match meta {
        MetaAttribute::ShowIf(show_if) => show_if.is_visible(object),
    })
}

pub mod info_box;

use crate::attributes::ParseContext;
use info_box::InfoBox;

pub enum DecoratorAttribute {
    InfoBox(InfoBox),
}

impl DecoratorAttribute {
    pub fn parse(key: &str, raw_args: &str, _context: &ParseContext) -> Result<Self, String> {
        match key {
            info_box::KEY => InfoBox::parse(raw_args).map(Self::InfoBox),
            _ => Err("unknown attribute".to_string()),
        }
    }

    pub fn is_known_key(key: &str) -> bool {
        matches!(key, info_box::KEY)
    }
}

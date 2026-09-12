pub mod min_max_slider;

use crate::attributes::ParseContext;
use min_max_slider::MinMaxSlider;

pub enum DrawerAttribute {
    MinMaxSlider(MinMaxSlider),
}

impl DrawerAttribute {
    pub fn parse(key: &str, raw_args: &str, context: &ParseContext) -> Result<Self, String> {
        match key {
            min_max_slider::KEY => MinMaxSlider::parse(raw_args, context).map(Self::MinMaxSlider),
            _ => Err("unknown attribute".to_string()),
        }
    }

    pub fn is_known_key(key: &str) -> bool {
        matches!(key, min_max_slider::KEY)
    }

    pub fn get_key(&self) -> &'static str {
        match self {
            Self::MinMaxSlider(_) => min_max_slider::KEY,
        }
    }
}

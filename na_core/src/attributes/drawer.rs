pub mod dropdown;
pub mod enum_flags;
pub mod min_max_slider;

use crate::attributes::parse_context::ParseContext;
use dropdown::Dropdown;
use enum_flags::EnumFlags;
use min_max_slider::MinMaxSlider;

pub enum DrawerAttribute {
    Dropdown(Dropdown),
    EnumFlags(EnumFlags),
    MinMaxSlider(MinMaxSlider),
}

impl DrawerAttribute {
    pub fn parse(key: &str, raw_args: &str, context: &ParseContext) -> Result<Self, String> {
        match key {
            dropdown::KEY => Dropdown::parse(raw_args, context).map(Self::Dropdown),
            enum_flags::KEY => EnumFlags::parse(raw_args, context).map(Self::EnumFlags),
            min_max_slider::KEY => MinMaxSlider::parse(raw_args, context).map(Self::MinMaxSlider),
            _ => Err("unknown attribute".to_string()),
        }
    }

    pub fn is_known_key(key: &str) -> bool {
        matches!(key, dropdown::KEY | enum_flags::KEY | min_max_slider::KEY)
    }

    pub fn get_key(&self) -> &'static str {
        match self {
            Self::Dropdown(_) => dropdown::KEY,
            Self::EnumFlags(_) => enum_flags::KEY,
            Self::MinMaxSlider(_) => min_max_slider::KEY,
        }
    }
}

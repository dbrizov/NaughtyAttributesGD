pub mod max_value;
pub mod min_value;
pub mod require;

use crate::attributes::parse_context::ParseContext;
use max_value::MaxValue;
use min_value::MinValue;
use require::Require;

pub enum ValidatorAttribute {
    MinValue(MinValue),
    MaxValue(MaxValue),
    Require(Require),
}

impl ValidatorAttribute {
    pub fn parse(key: &str, raw_args: &str, context: &ParseContext) -> Result<Self, String> {
        match key {
            min_value::KEY => MinValue::parse(raw_args, context).map(Self::MinValue),
            max_value::KEY => MaxValue::parse(raw_args, context).map(Self::MaxValue),
            require::KEY => Require::parse(raw_args, context).map(Self::Require),
            _ => Err("unknown attribute".to_string()),
        }
    }

    pub fn is_known_key(key: &str) -> bool {
        matches!(key, min_value::KEY | max_value::KEY | require::KEY)
    }

    pub fn get_key(&self) -> &'static str {
        match self {
            Self::MinValue(_) => min_value::KEY,
            Self::MaxValue(_) => max_value::KEY,
            Self::Require(_) => require::KEY,
        }
    }
}

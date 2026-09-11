pub mod min_value;

use crate::attributes::ParseContext;
use min_value::MinValue;

pub enum ValidatorAttribute {
    MinValue(MinValue),
}

impl ValidatorAttribute {
    pub fn parse(key: &str, raw_args: &str, context: &ParseContext) -> Result<Self, String> {
        match key {
            min_value::KEY => MinValue::parse(raw_args, context).map(Self::MinValue),
            _ => Err("unknown attribute".to_string()),
        }
    }

    pub fn is_known_key(key: &str) -> bool {
        matches!(key, min_value::KEY)
    }

    pub fn get_key(&self) -> &'static str {
        match self {
            Self::MinValue(_) => min_value::KEY,
        }
    }
}

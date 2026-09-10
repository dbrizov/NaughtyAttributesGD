use na_macros::naughty_attribute;

use crate::naughty_attribute::INaughtyAttribute;

#[naughty_attribute]
pub struct ShowIf {
    conditions: Vec<String>,
}

impl ShowIf {
    pub fn new(conditions: Vec<String>) -> Self {
        Self { conditions }
    }
}

impl INaughtyAttribute for ShowIf {
    fn parse(value: &str) -> Option<Self> {
        todo!()
    }
}

pub mod message_box;

use crate::attributes::ParseContext;
use message_box::{MessageBox, Severity};

pub enum DecoratorAttribute {
    MessageBox(MessageBox),
}

impl DecoratorAttribute {
    pub fn parse(key: &str, raw_args: &str, _context: &ParseContext) -> Result<Self, String> {
        match key {
            message_box::KEY_INFO_BOX => {
                MessageBox::parse(raw_args, Severity::Info).map(Self::MessageBox)
            }
            message_box::KEY_WARNING_BOX => {
                MessageBox::parse(raw_args, Severity::Warning).map(Self::MessageBox)
            }
            message_box::KEY_ERROR_BOX => {
                MessageBox::parse(raw_args, Severity::Error).map(Self::MessageBox)
            }
            _ => Err("unknown attribute".to_string()),
        }
    }

    pub fn is_known_key(key: &str) -> bool {
        matches!(
            key,
            message_box::KEY_INFO_BOX | message_box::KEY_WARNING_BOX | message_box::KEY_ERROR_BOX
        )
    }

    pub fn get_key(&self) -> &'static str {
        match self {
            Self::MessageBox(message_box) => message_box.get_key(),
        }
    }
}

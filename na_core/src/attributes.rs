pub mod meta;
pub mod validator;

use godot::prelude::*;

use meta::MetaAttribute;
use validator::ValidatorAttribute;

pub enum NaughtyAttribute {
    Meta(MetaAttribute),
    Validator(ValidatorAttribute),
}

impl NaughtyAttribute {
    pub fn parse(key: &str, raw_args: &str, context: &ParseContext) -> Option<Self> {
        if MetaAttribute::is_known_key(key) {
            MetaAttribute::parse(key, raw_args, context).map(Self::Meta)
        } else if ValidatorAttribute::is_known_key(key) {
            ValidatorAttribute::parse(key, raw_args, context).map(Self::Validator)
        } else {
            None
        }
    }
}

pub fn is_known_key(key: &str) -> bool {
    MetaAttribute::is_known_key(key) || ValidatorAttribute::is_known_key(key)
}

pub struct ParseContext<'a> {
    pub script_path: &'a str,
    pub property: &'a str,
    pub variant_type: VariantType,
    pub constants: &'a VarDictionary,
}

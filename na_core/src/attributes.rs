pub mod decorator;
pub mod drawer;
pub mod meta;
pub mod validator;

use godot::prelude::*;

use decorator::DecoratorAttribute;
use drawer::DrawerAttribute;
use meta::MetaAttribute;
use validator::ValidatorAttribute;

pub enum NaughtyAttribute {
    Decorator(DecoratorAttribute),
    Drawer(DrawerAttribute),
    Meta(MetaAttribute),
    Validator(ValidatorAttribute),
}

impl NaughtyAttribute {
    pub fn parse(key: &str, raw_args: &str, context: &ParseContext) -> Result<Self, String> {
        if DecoratorAttribute::is_known_key(key) {
            DecoratorAttribute::parse(key, raw_args, context).map(Self::Decorator)
        } else if DrawerAttribute::is_known_key(key) {
            DrawerAttribute::parse(key, raw_args, context).map(Self::Drawer)
        } else if MetaAttribute::is_known_key(key) {
            MetaAttribute::parse(key, raw_args, context).map(Self::Meta)
        } else if ValidatorAttribute::is_known_key(key) {
            ValidatorAttribute::parse(key, raw_args, context).map(Self::Validator)
        } else {
            Err("unknown attribute".to_string())
        }
    }
}

pub fn is_known_key(key: &str) -> bool {
    DecoratorAttribute::is_known_key(key)
        || DrawerAttribute::is_known_key(key)
        || MetaAttribute::is_known_key(key)
        || ValidatorAttribute::is_known_key(key)
}

/// What an attribute needs while parsing.
pub struct ParseContext<'a> {
    pub variant_type: VariantType,
    pub constants: &'a VarDictionary,
}

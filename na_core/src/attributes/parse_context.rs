use godot::prelude::*;

/// What an attribute needs while parsing.
pub struct ParseContext<'a> {
    pub variant_type: VariantType,
    pub constants: &'a VarDictionary,
}

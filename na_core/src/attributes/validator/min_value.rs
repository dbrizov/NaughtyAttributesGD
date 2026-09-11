use godot::prelude::*;

use crate::attributes::ParseContext;
use crate::expression::Expression;

pub const KEY: &str = "min_value";

pub const SUPPORTED_TYPES: &[VariantType] = &[
    VariantType::INT,
    VariantType::FLOAT,
    VariantType::VECTOR2,
    VariantType::VECTOR2I,
    VariantType::VECTOR3,
    VariantType::VECTOR3I,
    VariantType::VECTOR4,
    VariantType::VECTOR4I,
];

pub struct MinValue {
    pub min_value: Expression,
}

impl MinValue {
    pub fn parse(raw_args: &str, context: &ParseContext) -> Option<Self> {
        if !SUPPORTED_TYPES.contains(&context.variant_type) {
            context.warn(
                KEY,
                "can be used only on int, float, Vector2/3/4 and Vector2i/3i/4i properties",
            );
            return None;
        }

        let source = raw_args.trim();
        if source.is_empty() {
            context.warn(KEY, "expected a numeric expression");
            return None;
        }

        let min_value = Expression::compile(source, context.constants);
        if !min_value.is_valid() {
            context.warn(KEY, min_value.error());
        }

        Some(Self { min_value })
    }
}

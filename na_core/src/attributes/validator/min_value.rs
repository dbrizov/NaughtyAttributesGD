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
    pub fn parse(raw_args: &str, context: &ParseContext) -> Result<Self, String> {
        if !SUPPORTED_TYPES.contains(&context.variant_type) {
            return Err(
                "can be used only on int, float, Vector2/3/4 and Vector2i/3i/4i properties"
                    .to_string(),
            );
        }

        let source = raw_args.trim();
        if source.is_empty() {
            return Err("expected a numeric expression".to_string());
        }

        let min_value = Expression::compile(source, context.constants);
        if !min_value.is_valid() {
            return Err(min_value.error().to_string());
        }

        Ok(Self { min_value })
    }
}

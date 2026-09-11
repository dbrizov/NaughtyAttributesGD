use godot::prelude::*;

use na_logging::na_error;

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
            na_error!(
                "{}.{} - {KEY}: can be used only on int, float, Vector2/3/4 and Vector2i/3i/4i properties",
                context.script_path,
                context.property
            );
            return None;
        }

        let source = raw_args.trim();
        if source.is_empty() {
            na_error!(
                "{}.{} - {KEY}: expected a numeric expression",
                context.script_path,
                context.property
            );
            return None;
        }

        let min_value = Expression::compile(source, context.constants);
        if !min_value.is_valid() {
            na_error!(
                "{}.{} - {KEY}: {}",
                context.script_path,
                context.property,
                min_value.error()
            );
        }

        Some(Self { min_value })
    }
}

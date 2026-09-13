use godot::classes::Object;
use godot::prelude::*;

use crate::attributes::parse_context::ParseContext;
use crate::expressions::expression::Expression;

const NUMBER_TYPES: &[VariantType] = &[
    VariantType::INT,
    VariantType::FLOAT,
    VariantType::VECTOR2,
    VariantType::VECTOR2I,
    VariantType::VECTOR3,
    VariantType::VECTOR3I,
    VariantType::VECTOR4,
    VariantType::VECTOR4I,
];

#[derive(Clone)]
pub struct NumberExpression {
    expression: Expression,
}

impl NumberExpression {
    pub fn parse(raw_args: &str, context: &ParseContext) -> Result<Self, String> {
        if !NUMBER_TYPES.contains(&context.variant_type) {
            return Err(
                "can be used only on int, float, Vector2/3/4 and Vector2i/3i/4i properties"
                    .to_string(),
            );
        }

        let source = raw_args.trim();
        if source.is_empty() {
            return Err("expected a numeric expression".to_string());
        }

        let expression = Expression::compile(source, context.constants);
        if !expression.is_valid() {
            return Err(expression.get_error().to_string());
        }

        Ok(Self { expression })
    }

    pub fn evaluate(&self, object: &Gd<Object>) -> Result<f64, String> {
        self.expression.evaluate_number(object)
    }
}

use godot::classes::Object;
use godot::global::type_string;
use godot::prelude::*;

use crate::attributes::parse_context::ParseContext;
use crate::expressions::expression::Expression;

#[derive(Clone)]
pub struct DropdownOption {
    pub label: String,
    pub value: Variant,
}

#[derive(Clone)]
pub struct OptionsExpression {
    expression: Expression,
}

impl OptionsExpression {
    pub fn parse(raw_args: &str, context: &ParseContext) -> Result<Self, String> {
        let source = raw_args.trim();
        if source.is_empty() {
            return Err("expected an Array or a Dictionary expression".to_string());
        }

        let expression = Expression::compile(source, context.constants);
        if !expression.is_valid() {
            return Err(expression.get_error().to_string());
        }

        Ok(Self { expression })
    }

    pub fn evaluate(&self, object: &Gd<Object>) -> Result<Vec<DropdownOption>, String> {
        let value = self.expression.evaluate(object)?;
        let options: Vec<DropdownOption> = match value.get_type() {
            VariantType::ARRAY => value
                .to::<AnyArray>()
                .iter_shared()
                .map(|element| DropdownOption {
                    label: element.stringify().to_string(),
                    value: element,
                })
                .collect(),
            VariantType::DICTIONARY => value
                .to::<AnyDictionary>()
                .iter_shared()
                .map(|(key, value)| DropdownOption {
                    label: key.stringify().to_string(),
                    value,
                })
                .collect(),
            other => {
                return Err(format!(
                    "'{}' is {}, not an Array or a Dictionary",
                    self.expression.get_text(),
                    type_string(other.ord() as i64)
                ));
            }
        };

        if options.is_empty() {
            return Err(format!("'{}' is empty", self.expression.get_text()));
        }

        Ok(options)
    }
}

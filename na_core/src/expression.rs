use godot::classes::{Expression as GodotExpression, Object, Script};
use godot::global::type_string;
use godot::prelude::*;

pub struct Expression {
    expression_text: String,
    expression: Gd<GodotExpression>,
    inputs: VarArray,
    valid: bool,
    error: String,
}

impl Expression {
    pub fn compile(expression_text: &str, constants: &VarDictionary) -> Self {
        let mut input_names = PackedStringArray::new();
        let mut inputs = VarArray::new();

        for (key, value) in constants.iter_shared() {
            input_names.push(&key.stringify());
            inputs.push(&value);
        }

        let mut expression = GodotExpression::new_gd();
        let error = expression
            .parse_ex(&GString::from(&normalize_operators(expression_text)))
            .input_names(&input_names)
            .done();

        let valid = error == godot::global::Error::OK;
        let message = if valid {
            String::new()
        } else {
            expression.get_error_text().to_string()
        };

        Self {
            expression_text: expression_text.to_string(),
            expression,
            inputs,
            valid,
            error: message,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.valid
    }

    pub fn get_expression_text(&self) -> &str {
        &self.expression_text
    }

    pub fn get_error(&self) -> &str {
        &self.error
    }

    pub fn evaluate(&self, object: &Gd<Object>) -> Result<Variant, String> {
        if !self.valid {
            return Err(self.error.clone());
        }

        let mut expression = self.expression.clone();
        let result = expression
            .execute_ex()
            .inputs(&self.inputs)
            .base_instance(object)
            .show_error(false)
            .done();

        if expression.has_execute_failed() {
            return Err(format!(
                "'{}' - {}{}",
                self.expression_text,
                expression.get_error_text(),
                get_tool_hint(object, &self.expression_text)
            ));
        }

        Ok(result)
    }

    pub fn evaluate_bool(&self, object: &Gd<Object>) -> Result<bool, String> {
        self.evaluate(object).map(|value| value.booleanize())
    }

    pub fn evaluate_number(&self, object: &Gd<Object>) -> Result<f64, String> {
        let value = self.evaluate(object)?;

        match value.get_type() {
            VariantType::INT => Ok(value.to::<i64>() as f64),
            VariantType::FLOAT => Ok(value.to::<f64>()),
            other => Err(format!(
                "'{}' is {}, not a number",
                self.expression_text,
                type_string(other.ord() as i64)
            )),
        }
    }
}

fn get_tool_hint(object: &Gd<Object>, expression_text: &str) -> &'static str {
    if !expression_text.contains('(') {
        return "";
    }

    let is_tool = object
        .get("script")
        .try_to::<Gd<Script>>()
        .map(|script| script.is_tool())
        .unwrap_or(true);

    if is_tool {
        ""
    } else {
        " (calling a method needs @tool on the script)"
    }
}

pub fn normalize_operators(expression_text: &str) -> String {
    let mut out = String::with_capacity(expression_text.len());
    let mut chars = expression_text.chars().peekable();
    let mut quote: Option<char> = None;

    while let Some(character) = chars.next() {
        if let Some(active) = quote {
            out.push(character);
            if character == '\\' {
                if let Some(escaped) = chars.next() {
                    out.push(escaped);
                }
            } else if character == active {
                quote = None;
            }
            continue;
        }

        match character {
            '"' | '\'' => {
                quote = Some(character);
                out.push(character);
            }
            '&' if chars.peek() == Some(&'&') => {
                chars.next();
                out.push_str(" and ");
            }
            '|' if chars.peek() == Some(&'|') => {
                chars.next();
                out.push_str(" or ");
            }
            _ => out.push(character),
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::normalize_operators;

    #[test]
    fn inserts_spaces_around_replaced_operators() {
        assert_eq!(normalize_operators("a&&b"), "a and b");
        assert_eq!(normalize_operators("a||b"), "a or b");
        assert_eq!(
            normalize_operators("(level>5&&is_weapon)||kind==Weapon.MAGIC"),
            "(level>5 and is_weapon) or kind==Weapon.MAGIC"
        );
        assert_eq!(
            normalize_operators("(level > 5&&is_weapon)||kind == Weapon.MAGIC"),
            "(level > 5 and is_weapon) or kind == Weapon.MAGIC"
        );
    }

    #[test]
    fn leaves_spaced_operators_usable() {
        assert_eq!(normalize_operators("a && b"), "a  and  b");
        assert_eq!(normalize_operators("a || b"), "a  or  b");
    }

    #[test]
    fn leaves_keyword_forms_alone() {
        assert_eq!(normalize_operators("a and b"), "a and b");
        assert_eq!(normalize_operators("!a"), "!a");
    }

    #[test]
    fn does_not_touch_string_literals() {
        assert_eq!(
            normalize_operators(r#"has_item("a&&b") || x"#),
            r#"has_item("a&&b")  or  x"#
        );
    }

    #[test]
    fn leaves_single_ampersand_alone() {
        assert_eq!(normalize_operators("flags & 4 != 0"), "flags & 4 != 0");
    }
}

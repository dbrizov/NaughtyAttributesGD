use godot::classes::{Expression, Object};
use godot::prelude::*;

pub struct Condition {
    source: String,
    expression: Gd<Expression>,
    inputs: VarArray,
    valid: bool,
    error: String,
}

impl Condition {
    pub fn compile(source: &str, constants: &VarDictionary) -> Self {
        let mut input_names = PackedStringArray::new();
        let mut inputs = VarArray::new();

        for (key, value) in constants.iter_shared() {
            input_names.push(&key.stringify());
            inputs.push(&value);
        }

        let mut expression = Expression::new_gd();
        let error = expression
            .parse_ex(&GString::from(&normalize(source)))
            .input_names(&input_names)
            .done();

        let valid = error == godot::global::Error::OK;
        let message = if valid {
            String::new()
        } else {
            expression.get_error_text().to_string()
        };

        Self {
            source: source.to_string(),
            expression,
            inputs,
            valid,
            error: message,
        }
    }

    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn is_valid(&self) -> bool {
        self.valid
    }

    pub fn error(&self) -> &str {
        &self.error
    }

    pub fn evaluate(&self, object: &Gd<Object>) -> Result<bool, String> {
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
            return Err(expression.get_error_text().to_string());
        }

        Ok(result.booleanize())
    }
}

pub fn normalize(source: &str) -> String {
    let mut out = String::with_capacity(source.len());
    let mut chars = source.chars().peekable();
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
    use super::normalize;

    #[test]
    fn inserts_spaces_around_replaced_operators() {
        assert_eq!(normalize("a&&b"), "a and b");
        assert_eq!(normalize("a||b"), "a or b");
        assert_eq!(
            normalize("(level>5&&is_weapon)||kind==Weapon.MAGIC"),
            "(level>5 and is_weapon) or kind==Weapon.MAGIC"
        );
        assert_eq!(
            normalize("(level > 5&&is_weapon)||kind == Weapon.MAGIC"),
            "(level > 5 and is_weapon) or kind == Weapon.MAGIC"
        );
    }

    #[test]
    fn leaves_spaced_operators_usable() {
        assert_eq!(normalize("a && b"), "a  and  b");
        assert_eq!(normalize("a || b"), "a  or  b");
    }

    #[test]
    fn leaves_keyword_forms_alone() {
        assert_eq!(normalize("a and b"), "a and b");
        assert_eq!(normalize("!a"), "!a");
    }

    #[test]
    fn does_not_touch_string_literals() {
        assert_eq!(
            normalize(r#"has_item("a&&b") || x"#),
            r#"has_item("a&&b")  or  x"#
        );
    }

    #[test]
    fn leaves_single_ampersand_alone() {
        assert_eq!(normalize("flags & 4 != 0"), "flags & 4 != 0");
    }
}

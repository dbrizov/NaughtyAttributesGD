use crate::annotation::unescape;

pub const KEY: &str = "label";

pub struct Label {
    pub text: String,
}

impl Label {
    pub fn parse(raw_args: &str) -> Result<Self, String> {
        let text = unescape(raw_args.trim());
        if text.is_empty() {
            return Err("expected a non-empty text".to_string());
        }

        Ok(Self { text })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_a_bare_comma_in_the_label() {
        assert_eq!(Label::parse("Damage, base").unwrap().text, "Damage, base");
    }

    #[test]
    fn unescapes_an_escaped_semicolon() {
        assert_eq!(Label::parse(r"Damage\; base").unwrap().text, "Damage; base");
    }

    #[test]
    fn rejects_an_empty_label() {
        assert!(Label::parse("").is_err());
        assert!(Label::parse("   ").is_err());
    }
}

use crate::annotation::unescape;
use crate::severity::Severity;

pub const KEY_INFO_BOX: &str = "info_box";
pub const KEY_WARNING_BOX: &str = "warning_box";
pub const KEY_ERROR_BOX: &str = "error_box";

pub struct MessageBox {
    pub text: String,
    pub severity: Severity,
}

impl MessageBox {
    pub fn parse(raw_args: &str, severity: Severity) -> Result<Self, String> {
        let text = unescape(raw_args.trim());
        if text.is_empty() {
            return Err("expected a message".to_string());
        }

        Ok(Self { text, severity })
    }

    pub fn get_key(&self) -> &'static str {
        match self.severity {
            Severity::Info => KEY_INFO_BOX,
            Severity::Warning => KEY_WARNING_BOX,
            Severity::Error => KEY_ERROR_BOX,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_a_bare_comma_in_the_message() {
        let message_box = MessageBox::parse("Careful, really", Severity::Info).unwrap();
        assert_eq!(message_box.text, "Careful, really");
    }

    #[test]
    fn unescapes_an_escaped_semicolon() {
        let message_box = MessageBox::parse(r"Careful\; really", Severity::Info).unwrap();
        assert_eq!(message_box.text, "Careful; really");
    }

    #[test]
    fn rejects_an_empty_message() {
        assert!(MessageBox::parse("", Severity::Info).is_err());
        assert!(MessageBox::parse("   ", Severity::Warning).is_err());
    }

    #[test]
    fn reports_the_key_of_its_severity() {
        let info_box = MessageBox::parse("Msg", Severity::Info).unwrap();
        let warning_box = MessageBox::parse("Msg", Severity::Warning).unwrap();
        let error_box = MessageBox::parse("Msg", Severity::Error).unwrap();
        assert_eq!(info_box.get_key(), KEY_INFO_BOX);
        assert_eq!(warning_box.get_key(), KEY_WARNING_BOX);
        assert_eq!(error_box.get_key(), KEY_ERROR_BOX);
    }
}

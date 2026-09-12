use crate::annotation::split_args;

pub const KEY: &str = "info_box";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Info,
    Warning,
    Error,
}

impl Severity {
    fn parse(text: &str) -> Result<Self, String> {
        match text {
            "info" => Ok(Self::Info),
            "warning" => Ok(Self::Warning),
            "error" => Ok(Self::Error),
            _ => Err(format!(
                "expected a severity of info, warning or error, got '{text}'"
            )),
        }
    }
}

pub struct InfoBox {
    pub text: String,
    pub severity: Severity,
}

impl InfoBox {
    pub fn parse(raw_args: &str) -> Result<Self, String> {
        let args = split_args(raw_args);
        let (text, severity) = match args.as_slice() {
            [text] => (text, Severity::Info),
            [text, severity] => (text, Severity::parse(severity)?),
            _ => return Err("expected a message and an optional severity".to_string()),
        };

        if text.is_empty() {
            return Err("expected a message".to_string());
        }

        Ok(Self {
            text: text.to_string(),
            severity,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_to_the_info_severity() {
        let info_box = InfoBox::parse("Plain information").unwrap();
        assert_eq!(info_box.text, "Plain information");
        assert_eq!(info_box.severity, Severity::Info);
    }

    #[test]
    fn reads_every_severity() {
        assert_eq!(InfoBox::parse("A,info").unwrap().severity, Severity::Info);
        assert_eq!(
            InfoBox::parse("A,warning").unwrap().severity,
            Severity::Warning
        );
        assert_eq!(
            InfoBox::parse("A, error").unwrap().severity,
            Severity::Error
        );
    }

    #[test]
    fn keeps_an_escaped_comma_in_the_message() {
        let info_box = InfoBox::parse(r"Careful\, really").unwrap();
        assert_eq!(info_box.text, "Careful, really");
    }

    #[test]
    fn rejects_an_empty_message() {
        assert!(InfoBox::parse("").is_err());
        assert!(InfoBox::parse("   ").is_err());
        assert!(InfoBox::parse(",warning").is_err());
    }

    #[test]
    fn rejects_an_unknown_severity() {
        assert!(InfoBox::parse("A,fatal").is_err());
    }

    #[test]
    fn rejects_more_than_a_message_and_a_severity() {
        assert!(InfoBox::parse("A,warning,extra").is_err());
    }
}

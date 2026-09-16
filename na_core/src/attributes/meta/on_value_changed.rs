use godot::classes::Object;
use godot::prelude::*;

use crate::annotation::is_identifier;
use crate::expressions::expression::get_method_call_hint;

pub const KEY: &str = "on_value_changed";

pub struct OnValueChanged {
    pub method: String,
}

impl OnValueChanged {
    pub fn parse(raw_args: &str) -> Result<Self, String> {
        let name = raw_args.trim();
        if name.is_empty() {
            return Err("expected a method name".to_string());
        }

        if !is_identifier(name) {
            return Err(format!("'{name}' is not a method name"));
        }

        Ok(Self {
            method: name.to_string(),
        })
    }

    pub fn call(
        &self,
        object: &mut Gd<Object>,
        old_value: &Variant,
        new_value: &Variant,
    ) -> Result<(), String> {
        let method = StringName::from(self.method.as_str());
        if !object.has_method(&method) {
            return Err(format!(
                "'{}' is not a method of the script{}",
                self.method,
                get_method_call_hint(object)
            ));
        }

        object
            .try_call(&method, &[old_value.clone(), new_value.clone()])
            .map(|_| ())
            .map_err(|error| {
                format!("{error} (expected a callback with two arguments: old_value, new_value)")
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trims_the_method_name() {
        assert_eq!(
            OnValueChanged::parse("  on_hp_changed  ").unwrap().method,
            "on_hp_changed"
        );
    }

    #[test]
    fn rejects_an_empty_method_name() {
        assert!(OnValueChanged::parse("").is_err());
        assert!(OnValueChanged::parse("   ").is_err());
    }

    #[test]
    fn rejects_a_call_instead_of_a_name() {
        assert!(OnValueChanged::parse("on_hp_changed()").is_err());
    }

    #[test]
    fn rejects_a_name_starting_with_a_digit() {
        assert!(OnValueChanged::parse("1st_callback").is_err());
    }

    #[test]
    fn rejects_a_non_ascii_name() {
        assert!(OnValueChanged::parse("héllo").is_err());
    }
}

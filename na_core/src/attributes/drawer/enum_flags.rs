use godot::global::type_string;
use godot::prelude::*;

use crate::annotation::is_identifier;
use crate::attributes::parse_context::ParseContext;

pub const KEY: &str = "enum_flags";

pub struct EnumFlags {
    hint_text: GString,
}

impl EnumFlags {
    pub fn parse(raw_args: &str, context: &ParseContext) -> Result<Self, String> {
        let name = raw_args.trim();
        if name.is_empty() {
            return Err("expected an enum name".to_string());
        }

        if !is_identifier(name) {
            return Err(format!("'{name}' is not an enum name"));
        }

        if context.variant_type != VariantType::INT {
            return Err("can be used only on int properties".to_string());
        }

        let members = get_members(name, context.constants)?;

        Ok(Self {
            hint_text: GString::from(&build_hint_text(&members)),
        })
    }

    pub fn get_hint_text(&self) -> &GString {
        &self.hint_text
    }
}

fn get_members(name: &str, constants: &VarDictionary) -> Result<Vec<(String, i64)>, String> {
    let Some(constant) = constants.get(name) else {
        return Err(format!(
            "'{name}' is not declared in the script or its base scripts"
        ));
    };

    if constant.get_type() != VariantType::DICTIONARY {
        return Err(format!(
            "'{name}' is {}, not an enum",
            type_string(constant.get_type().ord() as i64)
        ));
    }

    let mut members = Vec::new();
    for (key, value) in constant.to::<AnyDictionary>().iter_shared() {
        if value.get_type() != VariantType::INT {
            return Err(format!(
                "'{name}.{}' is {}, not an int",
                key.stringify(),
                type_string(value.get_type().ord() as i64)
            ));
        }

        members.push((key.stringify().to_string(), value.to::<i64>()));
    }

    if members.is_empty() {
        return Err(format!("'{name}' has no members"));
    }

    Ok(members)
}

fn build_hint_text(members: &[(String, i64)]) -> String {
    members
        .iter()
        .map(|(name, value)| format!("{name}:{value}"))
        .collect::<Vec<_>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_a_flags_hint_in_declaration_order() {
        let members = vec![
            ("FIRE".to_string(), 1),
            ("ICE".to_string(), 2),
            ("WIND".to_string(), 4),
        ];

        assert_eq!(build_hint_text(&members), "FIRE:1,ICE:2,WIND:4");
    }

    #[test]
    fn keeps_the_values_the_enum_declares() {
        let members = vec![("NONE".to_string(), 0), ("ALL".to_string(), 3)];

        assert_eq!(build_hint_text(&members), "NONE:0,ALL:3");
    }
}

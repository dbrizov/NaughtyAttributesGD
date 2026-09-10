use godot::classes::{Object, Script};
use godot::prelude::*;
use godot::register::info::{PropertyHint, PropertyUsageFlags};

use crate::LOG_PREFIX;
use crate::parser::{self, ParsedHint};

pub struct PropertyDescriptor {
    pub name: StringName,
    pub variant_type: VariantType,
    pub hint: PropertyHint,
    pub hint_string: GString,
    pub usage: PropertyUsageFlags,
}

pub struct ClassDescriptor {
    pub script_path: String,
    pub properties: Vec<PropertyDescriptor>,
}

impl ClassDescriptor {
    pub fn is_naughty(&self) -> bool {
        !self.properties.is_empty()
    }

    pub fn find(&self, name: &StringName) -> Option<&PropertyDescriptor> {
        self.properties
            .iter()
            .find(|property| &property.name == name)
    }
}

pub fn parse_object(object: &Gd<Object>) -> ClassDescriptor {
    let script_path = script_path(object);
    let mut properties = Vec::new();

    for info in object.get_property_list().iter_shared() {
        let usage: PropertyUsageFlags = info.at("usage").to();
        if !is_script_variable(usage) {
            continue;
        }

        let hint: PropertyHint = info.at("hint").to();
        let hint_string = info.at("hint_string").to::<GString>().to_string();
        if hint != PropertyHint::NONE || hint_string.is_empty() {
            continue;
        }

        let name = info.at("name").to::<GString>().to_string();
        let parsed = parser::parse_hint_string(&hint_string, |_| false);

        for key in &parsed.unknown_keys {
            godot_warn!("{LOG_PREFIX} {script_path}.{name} - unknown attribute '{key}'");
        }

        if !parsed.is_claimed() {
            continue;
        }

        properties.push(PropertyDescriptor {
            name: StringName::from(name.as_str()),
            variant_type: info.at("type").to(),
            hint: builtin_hint(&parsed),
            hint_string: builtin_hint_string(&parsed),
            usage,
        });
    }

    ClassDescriptor {
        script_path,
        properties,
    }
}

fn builtin_hint(parsed: &ParsedHint) -> PropertyHint {
    parsed
        .builtin
        .as_ref()
        .and_then(|(key, _)| parser::builtin_hint(key))
        .unwrap_or(PropertyHint::NONE)
}

fn builtin_hint_string(parsed: &ParsedHint) -> GString {
    match &parsed.builtin {
        Some((_, raw_args)) => GString::from(raw_args.as_str()),
        None => GString::new(),
    }
}

fn is_script_variable(usage: PropertyUsageFlags) -> bool {
    let excluded =
        PropertyUsageFlags::GROUP | PropertyUsageFlags::SUBGROUP | PropertyUsageFlags::CATEGORY;

    usage.is_set(PropertyUsageFlags::SCRIPT_VARIABLE)
        && usage.is_set(PropertyUsageFlags::EDITOR)
        && (usage.ord() & excluded.ord()) == 0
}

fn script_path(object: &Gd<Object>) -> String {
    object
        .get("script")
        .try_to::<Gd<Script>>()
        .map(|script| script.get_path().to_string())
        .unwrap_or_default()
}

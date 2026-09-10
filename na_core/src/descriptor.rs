use godot::classes::{Object, Script};
use godot::prelude::*;
use godot::register::info::{PropertyHint, PropertyUsageFlags};

use crate::LOG_PREFIX;
use crate::attributes::meta::MetaAttribute;
use crate::attributes::{self, ParsedAttribute};
use crate::parse_context::ParseContext;
use crate::parser::{self, ParsedHint};

impl PropertyDescriptor {
    pub fn is_naughty(&self) -> bool {
        self.claimed
    }
}

pub struct PropertyDescriptor {
    pub name: StringName,
    pub variant_type: VariantType,
    pub hint: PropertyHint,
    pub hint_string: GString,
    pub usage: PropertyUsageFlags,
    pub claimed: bool,
    pub metas: Vec<MetaAttribute>,
}

pub struct ClassDescriptor {
    pub script_path: String,
    pub category: String,
    pub properties: Vec<PropertyDescriptor>,
}

impl ClassDescriptor {
    pub fn is_naughty(&self) -> bool {
        self.properties.iter().any(PropertyDescriptor::is_naughty)
    }

    pub fn find(&self, name: &StringName) -> Option<&PropertyDescriptor> {
        self.properties
            .iter()
            .find(|property| &property.name == name)
    }
}

pub fn parse_object(object: &Gd<Object>) -> ClassDescriptor {
    let script_path = script_path(object);
    let constants = merged_constants(object);
    let mut properties = Vec::new();

    for info in script_property_list(object) {
        let usage: PropertyUsageFlags = info.at("usage").to();
        if !is_editor_property(usage) {
            continue;
        }

        let hint: PropertyHint = info.at("hint").to();
        let hint_string = info.at("hint_string").to::<GString>().to_string();
        let name = info.at("name").to::<GString>().to_string();
        let variant_type: VariantType = info.at("type").to();

        if hint != PropertyHint::NONE || hint_string.is_empty() {
            properties.push(PropertyDescriptor {
                name: StringName::from(name.as_str()),
                variant_type,
                hint,
                hint_string: GString::from(hint_string.as_str()),
                usage,
                claimed: false,
                metas: Vec::new(),
            });
            continue;
        }

        let parsed = parser::parse_hint_string(&hint_string, attributes::is_known_key);

        for key in &parsed.unknown_keys {
            godot_warn!("{LOG_PREFIX} {script_path}.{name} - unknown attribute '{key}'");
        }

        if !parsed.is_claimed() {
            properties.push(PropertyDescriptor {
                name: StringName::from(name.as_str()),
                variant_type,
                hint: PropertyHint::NONE,
                hint_string: GString::new(),
                usage,
                claimed: false,
                metas: Vec::new(),
            });
            continue;
        }

        let context = ParseContext {
            script_path: &script_path,
            property: &name,
            constants: &constants,
        };

        let mut metas = Vec::new();
        for entry in &parsed.attributes {
            match ParsedAttribute::parse(&entry.key, &entry.raw_args, &context) {
                Some(ParsedAttribute::Meta(meta)) => metas.push(meta),
                None => {}
            }
        }

        properties.push(PropertyDescriptor {
            name: StringName::from(name.as_str()),
            variant_type,
            hint: builtin_hint(&parsed),
            hint_string: builtin_hint_string(&parsed),
            usage,
            claimed: true,
            metas,
        });
    }

    let category = script_path
        .rsplit('/')
        .next()
        .unwrap_or_default()
        .to_string();

    ClassDescriptor {
        script_path,
        category,
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

fn is_editor_property(usage: PropertyUsageFlags) -> bool {
    let excluded =
        PropertyUsageFlags::GROUP | PropertyUsageFlags::SUBGROUP | PropertyUsageFlags::CATEGORY;

    usage.is_set(PropertyUsageFlags::EDITOR) && (usage.ord() & excluded.ord()) == 0
}

fn script_property_list(object: &Gd<Object>) -> Vec<VarDictionary> {
    let Ok(script) = object.get("script").try_to::<Gd<Script>>() else {
        return Vec::new();
    };

    let mut chain = Vec::new();
    let mut current = Some(script);
    while let Some(script) = current {
        current = script.get_base_script();
        chain.push(script);
    }

    let mut properties = Vec::new();
    for script in chain.iter().rev() {
        for info in script.get_script_property_list().iter_shared() {
            properties.push(info);
        }
    }

    properties
}

fn merged_constants(object: &Gd<Object>) -> VarDictionary {
    let mut merged = VarDictionary::new();
    let mut chain = Vec::new();
    let mut current = object.get("script").try_to::<Gd<Script>>().ok();

    while let Some(script) = current {
        current = script.get_base_script();
        chain.push(script);
    }

    for script in chain.iter().rev() {
        for (key, value) in script.get_script_constant_map().iter_shared() {
            merged.set(&key, &value);
        }
    }

    merged
}

fn script_path(object: &Gd<Object>) -> String {
    object
        .get("script")
        .try_to::<Gd<Script>>()
        .map(|script| script.get_path().to_string())
        .unwrap_or_default()
}

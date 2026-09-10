use godot::classes::{Object, Script};
use godot::prelude::*;
use godot::register::info::{PropertyHint, PropertyUsageFlags};

use crate::LOG_PREFIX;
use crate::annotation::PropertyAnnotation;
use crate::attributes::meta::MetaAttribute;
use crate::attributes::{self, NaughtyAttribute, ParseContext};

impl PropertyDescriptor {
    fn plain(info: &PropertyInfo) -> Self {
        Self {
            name: StringName::from(info.name.as_str()),
            variant_type: info.variant_type,
            hint: info.hint,
            hint_string: GString::from(info.hint_string.as_str()),
            usage: info.usage,
            claimed: false,
            metas: Vec::new(),
        }
    }

    fn unclaimed(info: &PropertyInfo) -> Self {
        Self {
            hint: PropertyHint::NONE,
            hint_string: GString::new(),
            ..Self::plain(info)
        }
    }

    fn claimed(
        info: &PropertyInfo,
        annotation: &PropertyAnnotation,
        metas: Vec<MetaAttribute>,
    ) -> Self {
        Self {
            hint: annotation.builtin_hint(),
            hint_string: annotation.builtin_hint_string(),
            claimed: true,
            metas,
            ..Self::plain(info)
        }
    }

    pub fn is_naughty(&self) -> bool {
        self.claimed
    }
}

struct PropertyInfo {
    name: String,
    variant_type: VariantType,
    hint: PropertyHint,
    hint_string: String,
    usage: PropertyUsageFlags,
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
    pub fn from_object(object: &Gd<Object>) -> ClassDescriptor {
        let script_path = script_path(object);
        let constants = merged_constants(object);
        let mut properties = Vec::new();

        for info in script_property_list(object) {
            let usage: PropertyUsageFlags = info.at("usage").to();
            if !is_editor_property(usage) {
                continue;
            }

            let property = PropertyInfo {
                name: info.at("name").to::<GString>().to_string(),
                variant_type: info.at("type").to(),
                hint: info.at("hint").to(),
                hint_string: info.at("hint_string").to::<GString>().to_string(),
                usage,
            };

            if property.hint != PropertyHint::NONE || property.hint_string.is_empty() {
                properties.push(PropertyDescriptor::plain(&property));
                continue;
            }

            let annotation =
                PropertyAnnotation::parse(&property.hint_string, attributes::is_known_key);

            for key in &annotation.unknown_keys {
                godot_warn!(
                    "{LOG_PREFIX} {script_path}.{} - unknown attribute '{key}'",
                    property.name
                );
            }

            if !annotation.is_claimed() {
                properties.push(PropertyDescriptor::unclaimed(&property));
                continue;
            }

            let context = ParseContext {
                script_path: &script_path,
                property: &property.name,
                constants: &constants,
            };

            let mut metas = Vec::new();
            for entry in &annotation.attributes {
                if let Some(NaughtyAttribute::Meta(meta)) =
                    NaughtyAttribute::parse(&entry.key, &entry.raw_args, &context)
                {
                    metas.push(meta);
                }
            }

            properties.push(PropertyDescriptor::claimed(&property, &annotation, metas));
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

    pub fn is_naughty(&self) -> bool {
        self.properties.iter().any(PropertyDescriptor::is_naughty)
    }

    pub fn find(&self, name: &StringName) -> Option<&PropertyDescriptor> {
        self.properties
            .iter()
            .find(|property| &property.name == name)
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

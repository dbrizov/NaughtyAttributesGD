use godot::classes::{Object, Script};
use godot::prelude::*;
use godot::register::info::{PropertyHint, PropertyUsageFlags};

use na_logging::na_error;

use crate::annotation::PropertyAnnotation;
use crate::attributes::meta::MetaAttribute;
use crate::attributes::validator::ValidatorAttribute;
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
            validators: Vec::new(),
        }
    }

    fn unclaimed(info: &PropertyInfo) -> Self {
        Self {
            hint: PropertyHint::NONE,
            hint_string: GString::new(),
            ..Self::plain(info)
        }
    }

    fn claimed(info: &PropertyInfo, annotation: &PropertyAnnotation) -> Self {
        Self {
            hint: annotation.builtin_hint(),
            hint_string: annotation.builtin_hint_string(),
            claimed: true,
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
    pub validators: Vec<ValidatorAttribute>,
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

            let property_info = PropertyInfo {
                name: info.at("name").to::<GString>().to_string(),
                variant_type: info.at("type").to(),
                hint: info.at("hint").to(),
                hint_string: info.at("hint_string").to::<GString>().to_string(),
                usage,
            };

            if property_info.hint != PropertyHint::NONE || property_info.hint_string.is_empty() {
                properties.push(PropertyDescriptor::plain(&property_info));
                continue;
            }

            let annotation =
                PropertyAnnotation::parse(&property_info.hint_string, attributes::is_known_key);

            for key in &annotation.unknown_keys {
                na_error!(
                    "{script_path}.{} - unknown attribute '{key}'",
                    property_info.name
                );
            }

            if !annotation.is_claimed() {
                properties.push(PropertyDescriptor::unclaimed(&property_info));
                continue;
            }

            let context = ParseContext {
                script_path: &script_path,
                property: &property_info.name,
                variant_type: property_info.variant_type,
                constants: &constants,
            };

            let mut property = PropertyDescriptor::claimed(&property_info, &annotation);
            for entry in &annotation.attributes {
                match NaughtyAttribute::parse(&entry.key, &entry.raw_args, &context) {
                    Some(NaughtyAttribute::Meta(meta)) => property.metas.push(meta),
                    Some(NaughtyAttribute::Validator(validator)) => {
                        property.validators.push(validator)
                    }
                    None => {}
                }
            }

            properties.push(property);
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

pub fn script_path(object: &Gd<Object>) -> String {
    object
        .get("script")
        .try_to::<Gd<Script>>()
        .map(|script| script.get_path().to_string())
        .unwrap_or_default()
}

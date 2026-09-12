use godot::classes::{Object, Script};
use godot::prelude::*;
use godot::register::info::{PropertyHint, PropertyUsageFlags};

use na_logging::na_error;

use crate::annotation::PropertyAnnotation;
use crate::attributes::decorator::DecoratorAttribute;
use crate::attributes::drawer::DrawerAttribute;
use crate::attributes::meta::MetaAttribute;
use crate::attributes::validator::ValidatorAttribute;
use crate::attributes::{self, NaughtyAttribute, ParseContext};

impl PropertyDescriptor {
    fn plain(info: &PropertyInfo) -> Self {
        Self {
            name: StringName::from(info.name.as_str()),
            variant_type: info.variant_type,
            hint: info.hint,
            hint_text: GString::from(info.hint_text.as_str()),
            usage: info.usage,
            claimed: false,
            decorators: Vec::new(),
            drawer: None,
            metas: Vec::new(),
            validators: Vec::new(),
        }
    }

    fn unclaimed(info: &PropertyInfo) -> Self {
        Self {
            hint: PropertyHint::NONE,
            hint_text: GString::new(),
            ..Self::plain(info)
        }
    }

    fn claimed(info: &PropertyInfo, annotation: &PropertyAnnotation) -> Self {
        Self {
            hint: annotation.get_builtin_hint(),
            hint_text: annotation.get_builtin_hint_text(),
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
    hint_text: String,
    usage: PropertyUsageFlags,
}

pub struct PropertyDescriptor {
    pub name: StringName,
    pub variant_type: VariantType,
    pub hint: PropertyHint,
    pub hint_text: GString,
    pub usage: PropertyUsageFlags,
    pub claimed: bool,
    pub decorators: Vec<DecoratorAttribute>,
    pub drawer: Option<DrawerAttribute>,
    pub metas: Vec<MetaAttribute>,
    pub validators: Vec<ValidatorAttribute>,
}

#[derive(PartialEq)]
struct ScriptSnapshot {
    property_list: Vec<VarDictionary>,
    constants: VarDictionary,
    default_values: VarDictionary,
}

impl ScriptSnapshot {
    fn from_object(object: &Gd<Object>) -> Self {
        Self {
            property_list: get_script_property_list(object),
            constants: get_constants(object),
            default_values: get_default_values(object),
        }
    }
}

pub struct ClassDescriptor {
    pub script_path: String,
    pub script_name: String,
    pub properties: Vec<PropertyDescriptor>,
    script_snapshot: ScriptSnapshot,
}

impl ClassDescriptor {
    pub fn from_object(object: &Gd<Object>) -> ClassDescriptor {
        let script_snapshot = ScriptSnapshot::from_object(object);
        let script_path = get_script_path(object);
        let mut properties = Vec::new();

        for info in &script_snapshot.property_list {
            let usage: PropertyUsageFlags = info.at("usage").to();
            if !is_editor_property(usage) {
                continue;
            }

            let property_info = PropertyInfo {
                name: info.at("name").to::<GString>().to_string(),
                variant_type: info.at("type").to(),
                hint: info.at("hint").to(),
                hint_text: info.at("hint_string").to::<GString>().to_string(),
                usage,
            };

            if property_info.hint != PropertyHint::NONE || property_info.hint_text.is_empty() {
                properties.push(PropertyDescriptor::plain(&property_info));
                continue;
            }

            let annotation =
                PropertyAnnotation::parse(&property_info.hint_text, attributes::is_known_key);

            for key in &annotation.unknown_keys {
                na_error!(
                    "{script_path}.{} - {key}: unknown attribute",
                    property_info.name
                );
            }

            if !annotation.is_claimed() {
                properties.push(PropertyDescriptor::unclaimed(&property_info));
                continue;
            }

            let context = ParseContext {
                variant_type: property_info.variant_type,
                constants: &script_snapshot.constants,
            };

            let mut property = PropertyDescriptor::claimed(&property_info, &annotation);
            for entry in &annotation.attributes {
                match NaughtyAttribute::parse(&entry.key, &entry.raw_args, &context) {
                    Ok(NaughtyAttribute::Decorator(decorator)) => {
                        property.decorators.push(decorator)
                    }
                    Ok(NaughtyAttribute::Drawer(drawer)) => {
                        if property.drawer.is_some() {
                            na_error!(
                                "{script_path}.{} - {}: a property can have only one drawer, keeping the last one",
                                property_info.name,
                                entry.key
                            );
                        }

                        property.drawer = Some(drawer);
                    }
                    Ok(NaughtyAttribute::Meta(meta)) => property.metas.push(meta),
                    Ok(NaughtyAttribute::Validator(validator)) => {
                        property.validators.push(validator)
                    }
                    Err(error) => {
                        na_error!(
                            "{script_path}.{} - {}: {error}",
                            property_info.name,
                            entry.key
                        );
                    }
                }
            }

            properties.push(property);
        }

        let script_name = script_path
            .rsplit('/')
            .next()
            .unwrap_or_default()
            .to_string();

        ClassDescriptor {
            script_path,
            script_name,
            properties,
            script_snapshot,
        }
    }

    pub fn is_naughty(&self) -> bool {
        self.properties.iter().any(PropertyDescriptor::is_naughty)
    }

    pub fn is_stale(&self, object: &Gd<Object>) -> bool {
        ScriptSnapshot::from_object(object) != self.script_snapshot
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

pub fn get_script_path(object: &Gd<Object>) -> String {
    get_script(object)
        .map(|script| script.get_path().to_string())
        .unwrap_or_default()
}

fn get_script(object: &Gd<Object>) -> Option<Gd<Script>> {
    object.get("script").try_to::<Gd<Script>>().ok()
}

/// Returns the script and its bases, base-first.
fn get_script_chain(object: &Gd<Object>) -> Vec<Gd<Script>> {
    let mut chain = Vec::new();
    let mut current = get_script(object);

    while let Some(script) = current {
        current = script.get_base_script();
        chain.push(script);
    }

    chain.reverse();
    chain
}

fn get_script_property_list(object: &Gd<Object>) -> Vec<VarDictionary> {
    get_script(object)
        .map(|script| script.get_script_property_list().iter_shared().collect())
        .unwrap_or_default()
}

fn get_constants(object: &Gd<Object>) -> VarDictionary {
    let mut constants = VarDictionary::new();

    for script in get_script_chain(object) {
        for (key, value) in script.get_script_constant_map().iter_shared() {
            constants.set(&key, &value);
        }
    }

    constants
}

fn get_default_values(object: &Gd<Object>) -> VarDictionary {
    let mut default_values = VarDictionary::new();
    let Some(script) = get_script(object) else {
        return default_values;
    };

    for info in script.get_script_property_list().iter_shared() {
        let name = info.at("name");
        let text = name.to::<GString>().to_string();
        default_values.set(&name, &script.get_property_default_value(text.as_str()));
    }

    default_values
}

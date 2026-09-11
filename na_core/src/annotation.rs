use godot::prelude::GString;
use godot::register::info::PropertyHint;

pub const BUILTIN_HINTS: &[(&str, PropertyHint)] = &[
    ("range", PropertyHint::RANGE),
    ("enum", PropertyHint::ENUM),
    ("enum_suggestion", PropertyHint::ENUM_SUGGESTION),
    ("exp_easing", PropertyHint::EXP_EASING),
    ("link", PropertyHint::LINK),
    ("flags", PropertyHint::FLAGS),
    ("flags_2d_render", PropertyHint::LAYERS_2D_RENDER),
    ("flags_2d_physics", PropertyHint::LAYERS_2D_PHYSICS),
    ("flags_2d_navigation", PropertyHint::LAYERS_2D_NAVIGATION),
    ("flags_3d_render", PropertyHint::LAYERS_3D_RENDER),
    ("flags_3d_physics", PropertyHint::LAYERS_3D_PHYSICS),
    ("flags_3d_navigation", PropertyHint::LAYERS_3D_NAVIGATION),
    ("flags_avoidance", PropertyHint::LAYERS_AVOIDANCE),
    ("file", PropertyHint::FILE),
    ("dir", PropertyHint::DIR),
    ("global_file", PropertyHint::GLOBAL_FILE),
    ("global_dir", PropertyHint::GLOBAL_DIR),
    ("resource_type", PropertyHint::RESOURCE_TYPE),
    ("multiline", PropertyHint::MULTILINE_TEXT),
    ("expression", PropertyHint::EXPRESSION),
    ("placeholder", PropertyHint::PLACEHOLDER_TEXT),
    ("color_no_alpha", PropertyHint::COLOR_NO_ALPHA),
    ("type_string", PropertyHint::TYPE_STRING),
    ("node_path", PropertyHint::NODE_PATH_VALID_TYPES),
    ("save_file", PropertyHint::SAVE_FILE),
    ("global_save_file", PropertyHint::GLOBAL_SAVE_FILE),
    ("node_type", PropertyHint::NODE_TYPE),
    ("hide_quaternion_edit", PropertyHint::HIDE_QUATERNION_EDIT),
    ("password", PropertyHint::PASSWORD),
    ("locale_id", PropertyHint::LOCALE_ID),
    ("localizable_string", PropertyHint::LOCALIZABLE_STRING),
];

pub fn builtin_hint_from_key(key: &str) -> Option<PropertyHint> {
    BUILTIN_HINTS
        .iter()
        .find(|(name, _)| *name == key)
        .map(|(_, hint)| *hint)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttributeEntry {
    pub key: String,
    pub raw_args: String,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PropertyAnnotation {
    pub builtin: Option<(String, String)>,
    pub attributes: Vec<AttributeEntry>,
    pub unknown_keys: Vec<String>,
}

impl PropertyAnnotation {
    pub fn parse(hint_string: &str, is_known_key: impl Fn(&str) -> bool) -> Self {
        let mut annotation = PropertyAnnotation::default();

        for entry in split_unescaped(hint_string, ';') {
            let entry = entry.trim();
            if entry.is_empty() {
                continue;
            }

            let (key, raw_args) = match first_unescaped(entry, ':') {
                Some(index) => (entry[..index].trim(), &entry[index + 1..]),
                None => (entry, ""),
            };

            if builtin_hint_from_key(key).is_some() {
                annotation.builtin = Some((key.to_string(), trim_builtin_args(raw_args)));
            } else if is_known_key(key) {
                annotation.attributes.push(AttributeEntry {
                    key: key.to_string(),
                    raw_args: raw_args.trim().to_string(),
                });
            } else {
                annotation.unknown_keys.push(key.to_string());
            }
        }

        annotation
    }

    pub fn get_builtin_hint(&self) -> PropertyHint {
        self.builtin
            .as_ref()
            .and_then(|(key, _)| builtin_hint_from_key(key))
            .unwrap_or(PropertyHint::NONE)
    }

    pub fn get_builtin_hint_text(&self) -> GString {
        match &self.builtin {
            Some((_, raw_args)) => GString::from(raw_args.as_str()),
            None => GString::new(),
        }
    }

    pub fn is_claimed(&self) -> bool {
        self.builtin.is_some() || !self.attributes.is_empty()
    }
}

pub fn trim_builtin_args(raw_args: &str) -> String {
    raw_args
        .split(',')
        .map(str::trim)
        .collect::<Vec<_>>()
        .join(",")
}

pub fn split_args(raw_args: &str) -> Vec<String> {
    split_unescaped(raw_args, ',')
        .into_iter()
        .map(|arg| unescape(arg.trim()))
        .collect()
}

pub fn split_unescaped(text: &str, delimiter: char) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut start = 0;
    let mut chars = text.char_indices();

    while let Some((index, character)) = chars.next() {
        if character == '\\' {
            chars.next();
        } else if character == delimiter {
            parts.push(&text[start..index]);
            start = index + character.len_utf8();
        }
    }

    parts.push(&text[start..]);
    parts
}

pub fn first_unescaped(text: &str, needle: char) -> Option<usize> {
    let mut chars = text.char_indices();

    while let Some((index, character)) = chars.next() {
        if character == '\\' {
            chars.next();
        } else if character == needle {
            return Some(index);
        }
    }

    None
}

pub fn unescape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars();

    while let Some(character) = chars.next() {
        if character == '\\' {
            if let Some(escaped) = chars.next() {
                out.push(escaped);
            }
        } else {
            out.push(character);
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn known(key: &str) -> bool {
        matches!(key, "show_if" | "min_value" | "info_box")
    }

    #[test]
    fn splits_entries_on_semicolons() {
        let annotation = PropertyAnnotation::parse("show_if:a;min_value:3", known);
        assert_eq!(annotation.attributes.len(), 2);
        assert_eq!(annotation.attributes[0].key, "show_if");
        assert_eq!(annotation.attributes[0].raw_args, "a");
        assert_eq!(annotation.attributes[1].key, "min_value");
    }

    #[test]
    fn splits_key_on_first_colon_only() {
        let annotation = PropertyAnnotation::parse("show_if:kind == Weapon.MELEE", known);
        assert_eq!(annotation.attributes[0].raw_args, "kind == Weapon.MELEE");

        let annotation = PropertyAnnotation::parse("show_if:a ? b : c", known);
        assert_eq!(annotation.attributes[0].raw_args, "a ? b : c");
    }

    #[test]
    fn keeps_commas_inside_unsplit_args() {
        let annotation = PropertyAnnotation::parse(r#"show_if:has_item("sword", 2)"#, known);
        assert_eq!(annotation.attributes[0].raw_args, r#"has_item("sword", 2)"#);
    }

    #[test]
    fn recognises_builtin_hints() {
        let annotation = PropertyAnnotation::parse("range:0,10,0.1", known);
        assert_eq!(
            annotation.builtin,
            Some(("range".to_string(), "0,10,0.1".to_string()))
        );
        assert!(annotation.attributes.is_empty());
        assert!(annotation.is_claimed());
    }

    #[test]
    fn builtin_args_are_taken_verbatim() {
        let annotation = PropertyAnnotation::parse("enum:One,Two,Three;show_if:a", known);
        assert_eq!(annotation.builtin.unwrap().1, "One,Two,Three");
        assert_eq!(annotation.attributes.len(), 1);
    }

    #[test]
    fn empty_builtin_args_are_allowed() {
        let annotation = PropertyAnnotation::parse("multiline:", known);
        assert_eq!(
            annotation.builtin,
            Some(("multiline".to_string(), String::new()))
        );
        assert!(annotation.is_claimed());
    }

    #[test]
    fn unknown_keys_are_collected_not_claimed() {
        let annotation = PropertyAnnotation::parse("shwo_if:a", known);
        assert_eq!(annotation.unknown_keys, vec!["shwo_if".to_string()]);
        assert!(!annotation.is_claimed());
    }

    #[test]
    fn plain_hint_strings_are_not_claimed() {
        assert!(!PropertyAnnotation::parse("", known).is_claimed());
        assert!(!PropertyAnnotation::parse("2:", known).is_claimed());
    }

    #[test]
    fn splits_args_on_unescaped_commas() {
        assert_eq!(split_args("Careful, really"), vec!["Careful", "really"]);
        assert_eq!(
            split_args(r"Careful\, really"),
            vec!["Careful, really".to_string()]
        );
    }

    #[test]
    fn escaped_delimiters_survive_entry_splitting() {
        let annotation = PropertyAnnotation::parse(r"info_box:one\;two", known);
        assert_eq!(annotation.attributes.len(), 1);
        assert_eq!(
            split_args(&annotation.attributes[0].raw_args),
            vec!["one;two"]
        );
    }

    #[test]
    fn unescape_resolves_escapes() {
        assert_eq!(unescape(r"a\,b"), "a,b");
        assert_eq!(unescape(r"a\;b"), "a;b");
        assert_eq!(unescape(r"a\\b"), r"a\b");
    }

    #[test]
    fn tolerates_whitespace_around_delimiters() {
        let annotation = PropertyAnnotation::parse(
            "show_if : (level>5&&is_weapon)||kind==Weapon.MAGIC ;    range  : 0 ,   10,   0.1",
            known,
        );

        assert_eq!(annotation.attributes.len(), 1);
        assert_eq!(annotation.attributes[0].key, "show_if");
        assert_eq!(
            annotation.attributes[0].raw_args,
            "(level>5&&is_weapon)||kind==Weapon.MAGIC"
        );
        assert_eq!(
            annotation.builtin,
            Some(("range".to_string(), "0,10,0.1".to_string()))
        );
    }

    #[test]
    fn builtin_args_keep_internal_spaces() {
        let annotation = PropertyAnnotation::parse("enum: One , Two Three , Four", known);
        assert_eq!(annotation.builtin.unwrap().1, "One,Two Three,Four");
    }

    #[test]
    fn every_builtin_key_resolves() {
        for (key, _) in BUILTIN_HINTS {
            assert!(builtin_hint_from_key(key).is_some());
        }
        assert!(builtin_hint_from_key("definitely_not_a_hint").is_none());
    }
}

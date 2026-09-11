use godot::classes::Object;
use godot::prelude::*;

use crate::attributes::ParseContext;
use crate::member;

pub const KEY: &str = "min_value";

pub const SUPPORTED_TYPES: &[VariantType] = &[
    VariantType::INT,
    VariantType::FLOAT,
    VariantType::VECTOR2,
    VariantType::VECTOR2I,
    VariantType::VECTOR3,
    VariantType::VECTOR3I,
    VariantType::VECTOR4,
    VariantType::VECTOR4I,
];

pub struct MinValue {
    pub bound: Bound,
}

impl MinValue {
    pub fn parse(raw_args: &str, context: &ParseContext) -> Option<Self> {
        if !SUPPORTED_TYPES.contains(&context.variant_type) {
            context.warn(
                KEY,
                "can be used only on int, float, Vector2/3/4 and Vector2i/3i/4i properties",
            );
            return None;
        }

        let Some(bound) = Bound::parse(raw_args) else {
            context.warn(
                KEY,
                &format!(
                    "expected a number or a member name, got '{}'",
                    raw_args.trim()
                ),
            );
            return None;
        };

        Some(Self { bound })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Bound {
    Value(f64),
    Member(String),
}

impl Bound {
    pub fn parse(text: &str) -> Option<Self> {
        let text = text.trim();
        let first = text.chars().next()?;

        if first.is_ascii_digit() || matches!(first, '-' | '+' | '.') {
            return text
                .parse::<f64>()
                .ok()
                .filter(|value| value.is_finite())
                .map(Self::Value);
        }

        is_identifier(text).then(|| Self::Member(text.to_string()))
    }

    pub fn resolve(&self, object: &Gd<Object>) -> Result<f64, String> {
        match self {
            Self::Value(value) => Ok(*value),
            Self::Member(name) => member::resolve_number(object, name),
        }
    }
}

fn is_identifier(text: &str) -> bool {
    let mut chars = text.chars();

    chars
        .next()
        .is_some_and(|first| first == '_' || first.is_alphabetic())
        && chars.all(|character| character == '_' || character.is_alphanumeric())
}

#[cfg(test)]
mod tests {
    use super::Bound;

    fn member(name: &str) -> Option<Bound> {
        Some(Bound::Member(name.to_string()))
    }

    #[test]
    fn parses_numeric_literals() {
        assert_eq!(Bound::parse("0"), Some(Bound::Value(0.0)));
        assert_eq!(Bound::parse("2.5"), Some(Bound::Value(2.5)));
        assert_eq!(Bound::parse("-10"), Some(Bound::Value(-10.0)));
        assert_eq!(Bound::parse(" .5 "), Some(Bound::Value(0.5)));
        assert_eq!(Bound::parse("1e3"), Some(Bound::Value(1000.0)));
    }

    #[test]
    fn parses_member_names() {
        assert_eq!(Bound::parse("floor_hp"), member("floor_hp"));
        assert_eq!(Bound::parse(" _floor "), member("_floor"));
        assert_eq!(Bound::parse("get_floor2"), member("get_floor2"));
    }

    #[test]
    fn reads_float_keywords_as_member_names() {
        assert_eq!(Bound::parse("inf"), member("inf"));
        assert_eq!(Bound::parse("nan"), member("nan"));
    }

    #[test]
    fn rejects_malformed_bounds() {
        for text in [
            "",
            "   ",
            "-",
            "-inf",
            "1abc",
            "1_000",
            "0,10",
            "floor hp",
            "floor_hp()",
            "self.floor_hp",
        ] {
            assert_eq!(Bound::parse(text), None, "{text:?}");
        }
    }
}

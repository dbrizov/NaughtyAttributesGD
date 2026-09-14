use godot::prelude::*;

pub const KEY: &str = "horizontal_line";

const COLOR_ERROR: &str = "expected a colour like #5a5a5a or #5a5a5a80";

pub struct HorizontalLine {
    pub color: Option<Color>,
}

impl HorizontalLine {
    pub fn parse(raw_args: &str) -> Result<Self, String> {
        let source = raw_args.trim();
        if source.is_empty() {
            return Ok(Self { color: None });
        }

        parse_color(source).map(|color| Self { color: Some(color) })
    }
}

fn parse_color(source: &str) -> Result<Color, String> {
    let digits = source.strip_prefix('#').unwrap_or(source);
    if digits.len() != 6 && digits.len() != 8 {
        return Err(COLOR_ERROR.to_string());
    }

    let red = parse_channel(digits, 0)?;
    let green = parse_channel(digits, 1)?;
    let blue = parse_channel(digits, 2)?;
    let alpha = if digits.len() == 8 {
        parse_channel(digits, 3)?
    } else {
        u8::MAX
    };

    Ok(Color::from_rgba8(red, green, blue, alpha))
}

fn parse_channel(digits: &str, index: usize) -> Result<u8, String> {
    digits
        .get(index * 2..index * 2 + 2)
        .and_then(|channel| u8::from_str_radix(channel, 16).ok())
        .ok_or_else(|| COLOR_ERROR.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leaves_the_colour_to_the_theme_without_args() {
        assert!(HorizontalLine::parse("").unwrap().color.is_none());
        assert!(HorizontalLine::parse("   ").unwrap().color.is_none());
    }

    #[test]
    fn parses_a_six_digit_hex_as_opaque() {
        let line = HorizontalLine::parse("#5a5a5a").unwrap();
        assert_eq!(line.color, Some(Color::from_rgba8(0x5a, 0x5a, 0x5a, 0xff)));
    }

    #[test]
    fn parses_an_eight_digit_hex_with_alpha() {
        let line = HorizontalLine::parse("1a2b3c80").unwrap();
        assert_eq!(line.color, Some(Color::from_rgba8(0x1a, 0x2b, 0x3c, 0x80)));
    }

    #[test]
    fn rejects_a_malformed_colour() {
        assert!(HorizontalLine::parse("#5a5a5").is_err());
        assert!(HorizontalLine::parse("#gggggg").is_err());
        assert!(HorizontalLine::parse("dim_gray").is_err());
        assert!(HorizontalLine::parse("#5a5a5\u{00e9}").is_err());
    }
}

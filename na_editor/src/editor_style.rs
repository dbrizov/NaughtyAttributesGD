use godot::classes::{EditorInterface, Texture2D};
use godot::prelude::*;

pub const LABEL_WIDTH: f32 = 160.0;
pub const HORIZONTAL_SEPARATION: i32 = 6;
pub const VERTICAL_SEPARATION: i32 = 2;

pub fn get_icon(name: &str) -> Option<Gd<Texture2D>> {
    let theme = EditorInterface::singleton().get_editor_theme()?;
    if !theme.has_icon(name, "EditorIcons") {
        return None;
    }

    theme.get_icon(name, "EditorIcons")
}

pub fn get_color(name: &str) -> Option<Color> {
    let theme = EditorInterface::singleton().get_editor_theme()?;
    if !theme.has_color(name, "Editor") {
        return None;
    }

    Some(theme.get_color(name, "Editor"))
}

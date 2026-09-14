use godot::builtin::Side;
use godot::classes::{HSeparator, StyleBoxFlat, VBoxContainer};
use godot::prelude::*;

use na_core::attributes::decorator::horizontal_line::HorizontalLine;

use crate::decorators::IDecorator;
use crate::editor_style;

const CATEGORY_STYLEBOX: &str = "bg";
const CATEGORY_THEME_TYPE: &str = "EditorInspectorCategory";
const FALLBACK_COLOR: Color = Color::from_rgba(0.35, 0.35, 0.35, 1.0);
const CORNER_RADIUS: i32 = 2;
const MARGIN_TOP: f32 = 2.0;
const MARGIN_BOTTOM: f32 = 2.0;

impl IDecorator for HorizontalLine {
    fn decorate(&self, container: &mut Gd<VBoxContainer>, _object: &Gd<Object>) {
        let mut separator = HSeparator::new_alloc();
        separator.add_theme_stylebox_override("separator", &create_line_style());
        container.add_child(&separator);
    }
}

fn create_line_style() -> Gd<StyleBoxFlat> {
    let mut style = StyleBoxFlat::new_gd();
    style.set_bg_color(get_category_color());
    style.set_corner_radius_all(CORNER_RADIUS);
    style.set_content_margin(Side::TOP, MARGIN_TOP);
    style.set_content_margin(Side::BOTTOM, MARGIN_BOTTOM);
    style
}

fn get_category_color() -> Color {
    editor_style::get_stylebox(CATEGORY_STYLEBOX, CATEGORY_THEME_TYPE)
        .and_then(|stylebox| stylebox.try_cast::<StyleBoxFlat>().ok())
        .map(|stylebox| stylebox.get_bg_color())
        .unwrap_or(FALLBACK_COLOR)
}

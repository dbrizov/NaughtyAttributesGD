use godot::builtin::Side;
use godot::classes::control::SizeFlags;
use godot::classes::text_server::AutowrapMode;
use godot::classes::texture_rect::StretchMode;
use godot::classes::{HBoxContainer, Label, PanelContainer, StyleBoxFlat, TextureRect};
use godot::global::VerticalAlignment;
use godot::prelude::*;

use na_core::severity::Severity;

use crate::editor_style;

const ROW_SEPARATION: i32 = 8;
const MARGIN_LEFT: f32 = 8.0;
const MARGIN_RIGHT: f32 = 8.0;
const MARGIN_TOP: f32 = 1.0;
const MARGIN_BOTTOM: f32 = 1.0;

#[derive(Clone)]
pub struct MessageBubble {
    panel: Gd<PanelContainer>,
    label: Gd<Label>,
}

impl MessageBubble {
    pub fn new(severity: Severity) -> Self {
        let mut icon = TextureRect::new_alloc();
        if let Some(texture) = editor_style::get_icon(get_icon_name(severity)) {
            icon.set_texture(&texture);
        }
        icon.set_stretch_mode(StretchMode::KEEP_CENTERED);
        icon.set_v_size_flags(SizeFlags::SHRINK_CENTER);

        let mut label = Label::new_alloc();
        label.set_autowrap_mode(AutowrapMode::WORD_SMART);
        label.set_vertical_alignment(VerticalAlignment::CENTER);
        label.set_h_size_flags(SizeFlags::EXPAND_FILL);

        let mut row = HBoxContainer::new_alloc();
        row.add_theme_constant_override("separation", ROW_SEPARATION);
        row.add_child(&icon);
        row.add_child(&label);

        let mut panel = PanelContainer::new_alloc();
        panel.add_theme_stylebox_override("panel", &create_bubble_style(severity));
        panel.add_child(&row);

        Self { panel, label }
    }

    pub fn get_panel(&self) -> &Gd<PanelContainer> {
        &self.panel
    }

    pub fn set_text(&mut self, text: &str) {
        if self.label.is_instance_valid() {
            self.label.set_text(text);
        }
    }

    pub fn set_visible(&mut self, visible: bool) {
        if self.panel.is_instance_valid() && self.panel.is_visible() != visible {
            self.panel.set_visible(visible);
        }
    }
}

fn create_bubble_style(severity: Severity) -> Gd<StyleBoxFlat> {
    let color = editor_style::get_color(get_color_name(severity))
        .unwrap_or_else(|| Color::from_rgb(0.6, 0.6, 0.6));

    let mut style = StyleBoxFlat::new_gd();
    style.set_bg_color(Color { a: 0.12, ..color });
    style.set_border_color(Color { a: 0.45, ..color });
    style.set_border_width_all(1);
    style.set_corner_radius_all(6);
    style.set_content_margin(Side::LEFT, MARGIN_LEFT);
    style.set_content_margin(Side::RIGHT, MARGIN_RIGHT);
    style.set_content_margin(Side::TOP, MARGIN_TOP);
    style.set_content_margin(Side::BOTTOM, MARGIN_BOTTOM);
    style
}

fn get_icon_name(severity: Severity) -> &'static str {
    match severity {
        Severity::Info => "NodeInfo",
        Severity::Warning => "StatusWarning",
        Severity::Error => "StatusError",
    }
}

fn get_color_name(severity: Severity) -> &'static str {
    match severity {
        Severity::Info => "accent_color",
        Severity::Warning => "warning_color",
        Severity::Error => "error_color",
    }
}

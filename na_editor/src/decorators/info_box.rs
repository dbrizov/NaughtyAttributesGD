use godot::builtin::Side;
use godot::classes::control::SizeFlags;
use godot::classes::text_server::AutowrapMode;
use godot::classes::texture_rect::StretchMode;
use godot::classes::{
    Control, EditorInterface, HBoxContainer, Label, PanelContainer, StyleBoxFlat, Texture2D,
    TextureRect,
};
use godot::global::VerticalAlignment;
use godot::prelude::*;

use na_core::attributes::decorator::info_box::{InfoBox, Severity};

use crate::decorators::IDecorator;

impl IDecorator for InfoBox {
    fn decorate(&self, container: &mut Gd<Control>, _object: &Gd<Object>) {
        let mut icon = TextureRect::new_alloc();
        if let Some(texture) = get_icon(self.severity) {
            icon.set_texture(&texture);
        }
        icon.set_stretch_mode(StretchMode::KEEP_CENTERED);
        icon.set_v_size_flags(SizeFlags::SHRINK_CENTER);

        let mut label = Label::new_alloc();
        label.set_text(self.text.as_str());
        label.set_autowrap_mode(AutowrapMode::WORD_SMART);
        label.set_vertical_alignment(VerticalAlignment::CENTER);
        label.set_h_size_flags(SizeFlags::EXPAND_FILL);

        let mut row = HBoxContainer::new_alloc();
        row.add_theme_constant_override("separation", 8);
        row.add_child(&icon);
        row.add_child(&label);

        let mut bubble = PanelContainer::new_alloc();
        bubble.add_theme_stylebox_override("panel", &create_bubble_style(self.severity));
        bubble.add_child(&row);
        container.add_child(&bubble);
    }
}

fn create_bubble_style(severity: Severity) -> Gd<StyleBoxFlat> {
    let color = get_color(severity);

    let mut style = StyleBoxFlat::new_gd();
    style.set_bg_color(Color { a: 0.12, ..color });
    style.set_border_color(Color { a: 0.45, ..color });
    style.set_border_width_all(1);
    style.set_corner_radius_all(6);
    style.set_content_margin(Side::LEFT, 8.0);
    style.set_content_margin(Side::RIGHT, 8.0);
    style.set_content_margin(Side::TOP, 2.0);
    style.set_content_margin(Side::BOTTOM, 2.0);
    style
}

fn get_icon(severity: Severity) -> Option<Gd<Texture2D>> {
    let name = match severity {
        Severity::Info => "NodeInfo",
        Severity::Warning => "StatusWarning",
        Severity::Error => "StatusError",
    };

    let theme = EditorInterface::singleton().get_editor_theme()?;
    if !theme.has_icon(name, "EditorIcons") {
        return None;
    }

    theme.get_icon(name, "EditorIcons")
}

fn get_color(severity: Severity) -> Color {
    let name = match severity {
        Severity::Info => "accent_color",
        Severity::Warning => "warning_color",
        Severity::Error => "error_color",
    };

    EditorInterface::singleton()
        .get_editor_theme()
        .filter(|theme| theme.has_color(name, "Editor"))
        .map(|theme| theme.get_color(name, "Editor"))
        .unwrap_or_else(|| Color::from_rgb(0.6, 0.6, 0.6))
}

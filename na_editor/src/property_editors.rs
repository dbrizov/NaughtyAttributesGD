use std::collections::HashMap;

use godot::classes::control::SizeFlags;
use godot::classes::{
    Button, Control, EditorInspector, EditorProperty, HBoxContainer, Label, VBoxContainer,
};
use godot::global::VerticalAlignment;
use godot::prelude::*;

use na_core::descriptor::{ClassDescriptor, PropertyDescriptor};
use na_logging::na_error;

use crate::attribute_registry;
use crate::editor_style;
use crate::property_edit_action::PropertyEditAction;
use crate::property_utils;
use crate::variant_utils;

pub struct PropertyEditor {
    pub container: Gd<Control>,
    pub editor: Gd<EditorProperty>,
    pub revert_button: Gd<Button>,
}

impl PropertyEditor {
    pub fn refresh_revert_button(&self, object: &Gd<Object>, property: &PropertyDescriptor) {
        if !self.revert_button.is_instance_valid() {
            return;
        }

        let value = object.get(&property.name);
        let revertable = !variant_utils::is_equal_approx(&value, &property.default_value);
        set_revert_button_enabled(&mut self.revert_button.clone(), revertable);
    }
}

/// Returns `None` if Godot has no editor for the property.
fn create_property_editor(
    parent: &mut Gd<Control>,
    edit_action: &mut PropertyEditAction,
    object: &Gd<Object>,
    property: &PropertyDescriptor,
) -> Option<PropertyEditor> {
    let visible = property_utils::is_visible(object, property);
    if visible {
        property_utils::validate_property(edit_action, object, property);
    }

    let editor = property_utils::create_drawer_editor(object, property)
        .or_else(|| create_default_editor(object, property));

    let Some(mut editor) = editor else {
        na_error!("No property editor for '{}'", property.name);
        return None;
    };

    let mut container: Gd<Control> = VBoxContainer::new_alloc().upcast();
    container.add_theme_constant_override("separation", editor_style::VERTICAL_SEPARATION);
    for decorator in &property.decorators {
        attribute_registry::get_decorator(decorator).decorate(&mut container, object);
    }

    editor.set_label("");
    editor.set_name_split_ratio(0.0);
    editor.set_h_size_flags(SizeFlags::EXPAND_FILL);

    let mut label = Label::new_alloc();
    label.set_text(&property_utils::capitalize_name(&property.name));
    label.set_vertical_alignment(VerticalAlignment::CENTER);
    label.set_custom_minimum_size(Vector2::new(editor_style::LABEL_WIDTH, 0.0));

    let revert_button = create_revert_button();
    let mut row = HBoxContainer::new_alloc();
    row.add_theme_constant_override("separation", editor_style::HORIZONTAL_SEPARATION);
    row.add_child(&label);
    row.add_child(&revert_button);
    row.add_child(&editor);

    container.add_child(&row);
    container.set_visible(visible);
    parent.add_child(&container);
    editor.set_object_and_property(object, &property.name);
    editor.update_property();

    let property_editor = PropertyEditor {
        container,
        editor,
        revert_button,
    };

    property_editor.refresh_revert_button(object, property);
    Some(property_editor)
}

fn create_revert_button() -> Gd<Button> {
    let mut button = Button::new_alloc();
    if let Some(texture) = editor_style::get_icon("ReloadSmall") {
        button.set_button_icon(&texture);
    }

    button.set_flat(true);
    button.set_tooltip_text("Revert to the value the script declares");
    button.set_v_size_flags(SizeFlags::SHRINK_CENTER);
    set_revert_button_enabled(&mut button, false);
    button
}

fn set_revert_button_enabled(button: &mut Gd<Button>, enabled: bool) {
    let alpha = if enabled { 1.0 } else { 0.0 };
    button.set_modulate(Color::from_rgba(1.0, 1.0, 1.0, alpha));
    button.set_disabled(!enabled);
}

fn create_default_editor(
    object: &Gd<Object>,
    property: &PropertyDescriptor,
) -> Option<Gd<EditorProperty>> {
    EditorInspector::instantiate_property_editor(
        object,
        property.variant_type,
        &GString::from(&property.name),
        property.hint,
        &property.hint_text,
        property.usage.ord() as u32,
    )
}

/// Returns the created editors keyed by property name. Properties Godot has no editor for are left out.
pub fn create_property_editors(
    parent: &mut Gd<Control>,
    object: &Gd<Object>,
    class: &ClassDescriptor,
) -> HashMap<StringName, PropertyEditor> {
    let mut edit_action = PropertyEditAction::new(object);
    let mut editors = HashMap::new();

    for property in &class.properties {
        if let Some(editor) = create_property_editor(parent, &mut edit_action, object, property) {
            editors.insert(property.name.clone(), editor);
        }
    }

    edit_action.commit(&format!("Validate {}", class.category));
    editors
}

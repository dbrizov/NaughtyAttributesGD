use godot::classes::{Control, EditorInspector, EditorProperty, VBoxContainer};
use godot::obj::Inherits;
use godot::prelude::*;

use na_core::descriptor::PropertyDescriptor;
use na_logging::na_error;

use crate::attribute_registry;
use crate::property_undo_redo::PropertyEditAction;
use crate::property_utils;

#[derive(Clone)]
pub struct PropertyBlock {
    pub editor: Gd<EditorProperty>,
    pub decorations_container: Option<Gd<VBoxContainer>>,
}

impl PropertyBlock {
    pub fn set_label(&self, label: &str) {
        let mut editor = Gd::clone(&self.editor);
        if editor.is_instance_valid() {
            editor.set_label(label);
            editor.queue_redraw();
        }
    }

    pub fn set_visible(&self, visible: bool) {
        set_control_visible(&self.editor, visible);
        if let Some(container) = &self.decorations_container {
            set_control_visible(container, visible);
        }
    }

    pub fn set_enabled(&self, enabled: bool) {
        let mut editor = Gd::clone(&self.editor);
        let read_only = !enabled;
        if editor.is_instance_valid() && editor.is_read_only() != read_only {
            editor.set_read_only(read_only);
            editor.queue_redraw();
        }
    }
}

fn set_control_visible<T: Inherits<Control>>(control: &Gd<T>, visible: bool) {
    let mut control: Gd<Control> = Gd::clone(control).upcast();
    if control.is_instance_valid() && control.is_visible() != visible {
        control.set_visible(visible);
    }
}

pub fn create_property_block(
    edit_action: &mut PropertyEditAction,
    object: &Gd<Object>,
    property: &PropertyDescriptor,
    wide: bool,
) -> Option<PropertyBlock> {
    let visible = property_utils::is_visible(object, property);
    if visible {
        property_utils::validate_property(edit_action, object, property);
    }

    let editor = property_utils::create_drawer_editor(object, property)
        .or_else(|| create_default_editor(object, property, wide));

    let Some(editor) = editor else {
        na_error!("No property editor for '{}'", property.name);
        return None;
    };

    let property_block = PropertyBlock {
        editor,
        decorations_container: create_decorations_container(object, property),
    };

    property_block.set_visible(visible);
    Some(property_block)
}

fn create_decorations_container(
    object: &Gd<Object>,
    property: &PropertyDescriptor,
) -> Option<Gd<VBoxContainer>> {
    if property.decorators.is_empty() {
        return None;
    }

    let mut container = VBoxContainer::new_alloc();
    for attribute in &property.decorators {
        let decorator = attribute_registry::get_decorator(attribute);
        decorator.decorate(&mut container, object);
    }

    Some(container)
}

fn create_default_editor(
    object: &Gd<Object>,
    property: &PropertyDescriptor,
    wide: bool,
) -> Option<Gd<EditorProperty>> {
    EditorInspector::instantiate_property_editor_ex(
        object,
        property.variant_type,
        &GString::from(&property.name),
        property.hint,
        &property.hint_text,
        property.usage.ord() as u32,
    )
    .wide(wide)
    .done()
}

use godot::classes::{Control, EditorInspector, EditorProperty, VBoxContainer};
use godot::obj::Inherits;
use godot::prelude::*;

use na_core::descriptor::{self, PropertyDescriptor};
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
    pub fn set_label(&mut self, label: &str) {
        if self.editor.is_instance_valid() {
            self.editor.set_label(label);
            self.editor.queue_redraw();
        }
    }

    pub fn set_visible(&mut self, visible: bool) {
        set_control_visible(&mut self.editor, visible);
        if let Some(container) = &mut self.decorations_container {
            set_control_visible(container, visible);
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        let read_only = !enabled;
        if self.editor.is_instance_valid() && self.editor.is_read_only() != read_only {
            self.editor.set_read_only(read_only);
            self.editor.queue_redraw();
        }
    }
}

fn set_control_visible<T: Inherits<Control>>(control: &mut Gd<T>, visible: bool) {
    if !control.is_instance_valid() {
        return;
    }

    let control = control.upcast_mut::<Control>();
    if control.is_visible() != visible {
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
        na_error!(
            "{}.{}: no property editor",
            descriptor::get_script_path(object),
            property.name
        );
        return None;
    };

    let mut property_block = PropertyBlock {
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

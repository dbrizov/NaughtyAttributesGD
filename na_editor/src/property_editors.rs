use godot::classes::{Control, EditorInspector, EditorProperty, VBoxContainer};
use godot::prelude::*;

use na_core::descriptor::PropertyDescriptor;
use na_logging::na_error;

use crate::attribute_registry;
use crate::property_undo_redo::PropertyEditAction;
use crate::property_utils;

#[derive(Clone)]
pub struct PropertyEditor {
    pub editor: Gd<EditorProperty>,
    pub decorations: Option<Gd<Control>>,
}

impl PropertyEditor {
    pub fn set_visible(&self, visible: bool) {
        let editor: Gd<Control> = Gd::clone(&self.editor).upcast();
        set_control_visible(&editor, visible);
        if let Some(decorations) = &self.decorations {
            set_control_visible(decorations, visible);
        }
    }
}

fn set_control_visible(control: &Gd<Control>, visible: bool) {
    if control.is_instance_valid() && control.is_visible() != visible {
        Gd::clone(control).set_visible(visible);
    }
}

pub fn create_property_editor(
    edit_action: &mut PropertyEditAction,
    object: &Gd<Object>,
    property: &PropertyDescriptor,
    wide: bool,
) -> Option<PropertyEditor> {
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

    let property_editor = PropertyEditor {
        editor,
        decorations: create_decorations(object, property),
    };

    property_editor.set_visible(visible);
    Some(property_editor)
}

fn create_decorations(object: &Gd<Object>, property: &PropertyDescriptor) -> Option<Gd<Control>> {
    if property.decorators.is_empty() {
        return None;
    }

    let mut container: Gd<Control> = VBoxContainer::new_alloc().upcast();
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

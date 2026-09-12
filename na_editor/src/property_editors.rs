use std::collections::HashMap;

use godot::classes::{Control, EditorInspector, EditorProperty, VBoxContainer};
use godot::prelude::*;

use na_core::descriptor::{ClassDescriptor, PropertyDescriptor};
use na_logging::na_error;

use crate::attribute_registry;
use crate::property_edit_action::PropertyEditAction;
use crate::property_utils;

/// A property's decorators and its widget, in one container that is shown and hidden as a whole.
pub struct PropertyEditor {
    pub container: Gd<Control>,
    pub editor: Gd<EditorProperty>,
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

    let editor = EditorInspector::instantiate_property_editor(
        object,
        property.variant_type,
        &GString::from(&property.name),
        property.hint,
        &property.hint_text,
        property.usage.ord() as u32,
    );

    let Some(mut editor) = editor else {
        na_error!("No property editor for '{}'", property.name);
        return None;
    };

    let mut container: Gd<Control> = VBoxContainer::new_alloc().upcast();
    for decorator in &property.decorators {
        attribute_registry::get_decorator(decorator).decorate(&mut container, object);
    }

    editor.set_label(&property_utils::capitalize_name(&property.name));
    container.add_child(&editor);
    container.set_visible(visible);
    parent.add_child(&container);
    editor.set_object_and_property(object, &property.name);
    editor.update_property();

    Some(PropertyEditor { container, editor })
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

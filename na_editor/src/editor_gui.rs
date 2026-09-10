use godot::classes::{Control, EditorInspector, EditorProperty};
use godot::prelude::*;
use na_core::LOG_PREFIX;
use na_core::attributes::meta;
use na_core::descriptor::PropertyDescriptor;

pub fn draw_property(
    container: &mut Gd<Control>,
    object: &Gd<Object>,
    descriptor: &PropertyDescriptor,
) -> Option<Gd<EditorProperty>> {
    if !meta::is_visible(&descriptor.metas, object) {
        return None;
    }

    let editor = EditorInspector::instantiate_property_editor(
        object,
        descriptor.variant_type,
        &GString::from(&descriptor.name),
        descriptor.hint,
        &descriptor.hint_string,
        descriptor.usage.ord() as u32,
    );

    let Some(mut editor) = editor else {
        godot_warn!("{LOG_PREFIX} no property editor for '{}'", descriptor.name);
        return None;
    };

    editor.set_label(&label_for(&descriptor.name));
    container.add_child(&editor);
    editor.set_object_and_property(object, &descriptor.name);
    editor.update_property();

    Some(editor)
}

fn label_for(name: &StringName) -> GString {
    GString::from(name).capitalize()
}

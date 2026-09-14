use godot::classes::{EditorInspector, EditorProperty};
use godot::prelude::*;
use godot::register::info::PropertyHint;

use na_core::attributes::drawer::enum_flags::EnumFlags;
use na_core::descriptor::PropertyDescriptor;

use crate::drawers::IDrawer;

impl IDrawer for EnumFlags {
    fn create_editor(
        &self,
        object: &Gd<Object>,
        property: &PropertyDescriptor,
    ) -> Result<Gd<EditorProperty>, String> {
        EditorInspector::instantiate_property_editor_ex(
            object,
            property.variant_type,
            &GString::from(&property.name),
            PropertyHint::FLAGS,
            self.get_hint_text(),
            property.usage.ord() as u32,
        )
        .done()
        .ok_or_else(|| "no flags editor".to_string())
    }
}

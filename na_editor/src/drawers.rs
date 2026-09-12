pub mod min_max_slider;

use godot::classes::EditorProperty;
use godot::prelude::*;

use na_core::descriptor::PropertyDescriptor;

pub trait IDrawer {
    /// Returns the editor that replaces the default editor.
    fn create_editor(
        &self,
        object: &Gd<Object>,
        property: &PropertyDescriptor,
    ) -> Result<Gd<EditorProperty>, String>;
}

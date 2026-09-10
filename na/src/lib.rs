use godot::prelude::*;

pub use na_core::*;
pub use na_editor::*;

struct NaughtyAttributesExtensionLibrary;

#[gdextension]
unsafe impl ExtensionLibrary for NaughtyAttributesExtensionLibrary {}

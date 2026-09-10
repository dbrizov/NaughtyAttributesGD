use godot::prelude::*;

pub use na_core::*;
pub use na_editor::*;

struct NaughtyAttributes;

#[gdextension]
unsafe impl ExtensionLibrary for NaughtyAttributes {}

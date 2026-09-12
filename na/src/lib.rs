use godot::prelude::*;

pub use na_core::*;
pub use na_editor::*;

/// The GDExtension entry point; gdext registers every class through it.
struct NaughtyAttributesExtensionLibrary;

#[gdextension]
unsafe impl ExtensionLibrary for NaughtyAttributesExtensionLibrary {}

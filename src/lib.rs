use godot::prelude::*;

mod meta_attributes;
mod naughty_attribute;
mod naughty_editor_inspector_plugin;
mod naughty_editor_plugin;

struct NaughtyAttributes;

#[gdextension]
unsafe impl ExtensionLibrary for NaughtyAttributes {}

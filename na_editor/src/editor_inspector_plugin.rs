use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use godot::classes::{
    Control, EditorInspectorPlugin, EditorInterface, EditorProperty, IEditorInspectorPlugin,
    VBoxContainer,
};
use godot::obj::InstanceId;
use godot::prelude::*;
use godot::register::info::PropertyHint;
use na_core::descriptor::{self, ClassDescriptor};

use crate::editor_gui;

#[derive(GodotClass)]
#[class(tool, init, base = EditorInspectorPlugin)]
pub struct NaughtyEditorInspectorPlugin {
    cache: RefCell<HashMap<InstanceId, Rc<ClassDescriptor>>>,
    editors: HashMap<StringName, Gd<EditorProperty>>,
    edited: Option<Gd<Object>>,
    applying: bool,
    base: Base<EditorInspectorPlugin>,
}

#[godot_api]
impl IEditorInspectorPlugin for NaughtyEditorInspectorPlugin {
    fn can_handle(&self, object: Option<Gd<Object>>) -> bool {
        object
            .and_then(|object| self.descriptor_for(&object))
            .is_some()
    }

    fn parse_begin(&mut self, object: Option<Gd<Object>>) {
        let Some(object) = object else {
            return;
        };

        let Some(descriptor) = self.descriptor_for(&object) else {
            return;
        };

        self.editors.clear();
        self.edited = Some(object.clone());

        let mut container: Gd<Control> = VBoxContainer::new_alloc().upcast();

        for property in &descriptor.properties {
            let Some(editor) = editor_gui::draw_property(&mut container, &object, property) else {
                continue;
            };

            self.connect_property_changed(&editor);
            self.editors.insert(property.name.clone(), editor);
        }

        self.base_mut().add_custom_control(&container);
    }

    fn parse_property(
        &mut self,
        object: Option<Gd<Object>>,
        _variant_type: VariantType,
        name: GString,
        _hint: PropertyHint,
        _hint_string: GString,
        _usage: godot::register::info::PropertyUsageFlags,
        _wide: bool,
    ) -> bool {
        object
            .and_then(|object| self.descriptor_for(&object))
            .is_some_and(|descriptor| descriptor.find(&StringName::from(&name)).is_some())
    }
}

impl NaughtyEditorInspectorPlugin {
    fn descriptor_for(&self, object: &Gd<Object>) -> Option<Rc<ClassDescriptor>> {
        let script = object
            .get("script")
            .try_to::<Gd<godot::classes::Script>>()
            .ok()?;
        let key = script.instance_id();

        let cached = self.cache.borrow().get(&key).cloned();
        let descriptor = match cached {
            Some(descriptor) => descriptor,
            None => {
                let parsed = Rc::new(descriptor::parse_object(object));
                self.cache.borrow_mut().insert(key, parsed.clone());
                parsed
            }
        };

        descriptor.is_naughty().then_some(descriptor)
    }

    fn connect_property_changed(&self, editor: &Gd<EditorProperty>) {
        let callable = self.base().callable("on_property_changed");
        editor.clone().connect("property_changed", &callable);
    }

    pub fn invalidate_cache(&self) {
        self.cache.borrow_mut().clear();
    }

    pub fn refresh_conditions(&mut self) {
        if self.applying {
            return;
        }
    }
}

#[godot_api]
impl NaughtyEditorInspectorPlugin {
    #[func]
    fn on_property_changed(
        &mut self,
        name: StringName,
        value: Variant,
        _field: StringName,
        _changing: bool,
    ) {
        if self.applying {
            return;
        }

        let Some(object) = self.edited.clone() else {
            return;
        };

        let Some(mut undo_redo) = EditorInterface::singleton().get_editor_undo_redo() else {
            return;
        };

        self.applying = true;

        let previous = object.get(&name);
        undo_redo.create_action(&GString::from(&format!("Set {name}")));
        undo_redo.add_do_property(&object, &name, &value);
        undo_redo.add_undo_property(&object, &name, &previous);
        undo_redo.commit_action();

        self.applying = false;

        self.refresh_conditions();
    }
}

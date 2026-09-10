use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use godot::classes::{
    Control, EditorInspectorPlugin, EditorInterface, EditorProperty, IEditorInspectorPlugin,
    Script, VBoxContainer,
};
use godot::obj::InstanceId;
use godot::prelude::*;
use godot::register::info::{PropertyHint, PropertyUsageFlags};
use na_core::attributes::meta;
use na_core::descriptor::{self, ClassDescriptor};

use crate::editor_gui;

#[derive(Default)]
struct InspectorState {
    descriptor: Option<Rc<ClassDescriptor>>,
    object: Option<Gd<Object>>,
    editors: HashMap<StringName, Gd<EditorProperty>>,
    pending: Option<Gd<Control>>,
}

#[derive(GodotClass)]
#[class(tool, init, base = EditorInspectorPlugin)]
pub struct NaughtyEditorInspectorPlugin {
    cache: RefCell<HashMap<InstanceId, Rc<ClassDescriptor>>>,
    state: Rc<RefCell<InspectorState>>,
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

        let container: Gd<Control> = VBoxContainer::new_alloc().upcast();

        {
            let mut state = self.state.borrow_mut();
            state.descriptor = Some(descriptor.clone());
            state.object = Some(object.clone());
            state.editors.clear();
            state.pending = Some(container.clone());
        }

        let state = self.state.clone();
        let plugin_id = self.base().instance_id();
        let mut container = container;

        Callable::from_fn("naughty_populate", move |_args| {
            populate(&mut container, &object, &descriptor, &state, plugin_id);
            Variant::nil()
        })
        .call_deferred(&[]);
    }

    fn parse_end(&mut self, _object: Option<Gd<Object>>) {
        self.attach_pending();
    }

    fn parse_category(&mut self, _object: Option<Gd<Object>>, category: GString) {
        let matches = {
            let state = self.state.borrow();
            state
                .descriptor
                .as_ref()
                .is_some_and(|descriptor| descriptor.category == category.to_string())
        };

        if matches {
            self.attach_pending();
        }
    }

    fn parse_property(
        &mut self,
        object: Option<Gd<Object>>,
        _variant_type: VariantType,
        name: GString,
        _hint: PropertyHint,
        _hint_string: GString,
        _usage: PropertyUsageFlags,
        _wide: bool,
    ) -> bool {
        object
            .and_then(|object| self.descriptor_for(&object))
            .is_some_and(|descriptor| descriptor.find(&StringName::from(&name)).is_some())
    }
}

#[godot_api]
impl NaughtyEditorInspectorPlugin {
    #[func]
    fn refresh_conditions(&self) {
        let state = self.state.borrow();
        let (Some(object), Some(descriptor)) = (state.object.as_ref(), state.descriptor.as_ref())
        else {
            return;
        };

        for property in &descriptor.properties {
            let Some(editor) = state.editors.get(&property.name) else {
                continue;
            };

            if !editor.is_instance_valid() {
                continue;
            }

            let visible = meta::is_visible(&property.metas, object);
            if editor.is_visible() != visible {
                editor.clone().set_visible(visible);
            }
        }
    }
}

impl NaughtyEditorInspectorPlugin {
    fn descriptor_for(&self, object: &Gd<Object>) -> Option<Rc<ClassDescriptor>> {
        let script = object.get("script").try_to::<Gd<Script>>().ok()?;
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

    fn attach_pending(&mut self) {
        let Some(container) = self.state.borrow_mut().pending.take() else {
            return;
        };
        self.base_mut().add_custom_control(&container);
    }

    pub fn invalidate_cache(&self) {
        self.cache.borrow_mut().clear();
        let mut state = self.state.borrow_mut();
        state.descriptor = None;
        state.editors.clear();
    }
}

fn populate(
    container: &mut Gd<Control>,
    object: &Gd<Object>,
    descriptor: &Rc<ClassDescriptor>,
    state: &Rc<RefCell<InspectorState>>,
    plugin_id: InstanceId,
) {
    if !container.is_instance_valid() {
        return;
    }

    for property in &descriptor.properties {
        let Some(mut editor) = editor_gui::draw_property(container, object, property) else {
            continue;
        };

        connect_changed(&mut editor, object, plugin_id);
        state
            .borrow_mut()
            .editors
            .insert(property.name.clone(), editor);
    }
}

fn connect_changed(editor: &mut Gd<EditorProperty>, object: &Gd<Object>, plugin_id: InstanceId) {
    let object = object.clone();
    let callable = Callable::from_linked_fn("naughty_property_changed", editor, move |args| {
        apply_change(&object, args, plugin_id);
        Variant::nil()
    });

    editor.connect("property_changed", &callable);
}

fn apply_change(object: &Gd<Object>, args: &[&Variant], plugin_id: InstanceId) {
    let (Some(name), Some(value)) = (args.first(), args.get(1)) else {
        return;
    };

    let name = name.to::<StringName>();
    let Some(mut undo_redo) = EditorInterface::singleton().get_editor_undo_redo() else {
        return;
    };

    let previous = object.get(&name);
    if previous == **value {
        return;
    }

    undo_redo.create_action(&GString::from(&format!("Set {name}")));
    undo_redo.add_do_property(object, &name, value);
    undo_redo.add_undo_property(object, &name, &previous);
    undo_redo.commit_action();

    if let Ok(mut plugin) = Gd::<NaughtyEditorInspectorPlugin>::try_from_instance_id(plugin_id) {
        plugin.call_deferred("refresh_conditions", &[]);
    }
}

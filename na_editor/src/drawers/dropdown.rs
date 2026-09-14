use godot::classes::control::SizeFlags;
use godot::classes::{EditorProperty, IEditorProperty, OptionButton};
use godot::prelude::*;

use na_core::attributes::drawer::dropdown::{self, Dropdown};
use na_core::descriptor::{self, PropertyDescriptor};
use na_core::expressions::options_expression::DropdownOption;
use na_logging::na_error;

use crate::drawers::IDrawer;

const STALE_VALUE_SUFFIX: &str = " (not an option)";

impl IDrawer for Dropdown {
    fn create_editor(
        &self,
        object: &Gd<Object>,
        property: &PropertyDescriptor,
    ) -> Result<Gd<EditorProperty>, String> {
        let options = self.evaluate_options(object)?;
        let value = object.get(&property.name);
        let mut editor = NaughtyDropdown::new_alloc();
        editor.bind_mut().setup(self.clone(), options, &value);

        Ok(editor.upcast())
    }
}

/// The `dropdown` widget (not the attribute).
#[derive(GodotClass)]
#[class(tool, init, base = EditorProperty)]
pub struct NaughtyDropdown {
    attribute: Option<Dropdown>,
    option_button: Option<Gd<OptionButton>>,
    options: Vec<DropdownOption>,
    base: Base<EditorProperty>,
}

#[godot_api]
impl IEditorProperty for NaughtyDropdown {
    fn update_property(&mut self) {
        let Some(object) = self.base().get_edited_object() else {
            return;
        };

        let property = self.base().get_edited_property();
        let Some(options) = self.evaluate_options(&object, &property) else {
            return;
        };

        self.set_options(options, &object.get(&property));
    }

    fn set_read_only(&mut self, read_only: bool) {
        if let Some(button) = self.option_button.as_mut() {
            button.set_disabled(read_only);
        }
    }
}

#[godot_api]
impl NaughtyDropdown {
    #[func]
    fn on_item_selected(&mut self, index: i32) {
        let Some(value) = self
            .options
            .get(index as usize)
            .map(|option| option.value.clone())
        else {
            return;
        };

        let Some(object) = self.base().get_edited_object() else {
            return;
        };

        let property = self.base().get_edited_property();
        if object.get(&property) == value {
            return;
        }

        self.base_mut().emit_changed(&property, &value);
    }
}

impl NaughtyDropdown {
    fn setup(&mut self, attribute: Dropdown, options: Vec<DropdownOption>, value: &Variant) {
        self.attribute = Some(attribute);
        let mut button = OptionButton::new_alloc();
        button.set_h_size_flags(SizeFlags::EXPAND_FILL);
        button.set_clip_text(true);
        button.set_fit_to_longest_item(false);

        let object = self.to_gd();
        button.connect("item_selected", &object.callable("on_item_selected"));

        let mut base = self.base_mut();
        base.add_child(&button);
        base.add_focusable(&button);
        drop(base);

        self.option_button = Some(button);
        self.set_options(options, value);
    }

    fn evaluate_options(
        &self,
        object: &Gd<Object>,
        property: &StringName,
    ) -> Option<Vec<DropdownOption>> {
        let attribute = self.attribute.as_ref()?;
        match attribute.evaluate_options(object) {
            Ok(options) => Some(options),
            Err(error) => {
                na_error!(
                    "{}.{property} - {}: {error}",
                    descriptor::get_script_path(object),
                    dropdown::KEY
                );
                None
            }
        }
    }

    fn set_options(&mut self, options: Vec<DropdownOption>, value: &Variant) {
        let mut items = Vec::with_capacity(options.len() + 1);
        if !options.iter().any(|option| option.value == *value) {
            items.push(DropdownOption {
                label: format!("{}{STALE_VALUE_SUFFIX}", value.stringify()),
                value: value.clone(),
            });
        }

        items.extend(options);
        let selected = items.iter().position(|option| option.value == *value);

        if let Some(button) = self.option_button.as_mut() {
            button.set_block_signals(true);
            button.clear();
            for option in &items {
                button.add_item(&GString::from(&option.label));
            }

            button.select(selected.map_or(-1, |index| index as i32));
            button.set_block_signals(false);
        }

        self.options = items;
    }
}

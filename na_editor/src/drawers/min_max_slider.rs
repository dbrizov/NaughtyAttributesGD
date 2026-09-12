use godot::classes::control::SizeFlags;
use godot::classes::{Control, EditorProperty, EditorSpinSlider, HBoxContainer, IEditorProperty};
use godot::prelude::*;

use na_core::attributes::drawer::min_max_slider::MinMaxSlider;
use na_core::descriptor::PropertyDescriptor;

use crate::drawers::IDrawer;

impl IDrawer for MinMaxSlider {
    fn create_editor(
        &self,
        object: &Gd<Object>,
        property: &PropertyDescriptor,
    ) -> Result<Gd<EditorProperty>, String> {
        let min = self.min_value.evaluate_number(object)?;
        let max = self.max_value.evaluate_number(object)?;
        if min > max {
            return Err(format!(
                "the minimum {min} is greater than the maximum {max}"
            ));
        }

        let mut editor = NaughtyMinMaxSlider::new_alloc();
        editor
            .bind_mut()
            .setup(min, max, property.variant_type == VariantType::VECTOR2I);

        Ok(editor.upcast())
    }
}

#[derive(GodotClass)]
#[class(tool, init, base = EditorProperty)]
pub struct NaughtyMinMaxSlider {
    min_slider: Option<Gd<EditorSpinSlider>>,
    max_slider: Option<Gd<EditorSpinSlider>>,
    is_integer: bool,
    base: Base<EditorProperty>,
}

#[godot_api]
impl IEditorProperty for NaughtyMinMaxSlider {
    fn update_property(&mut self) {
        let Some(object) = self.base().get_edited_object() else {
            return;
        };

        let value = object.get(&self.base().get_edited_property());
        let (min, max) = if self.is_integer {
            let value = value.try_to::<Vector2i>().unwrap_or_default();
            (value.x as f64, value.y as f64)
        } else {
            let value = value.try_to::<Vector2>().unwrap_or_default();
            (value.x as f64, value.y as f64)
        };

        if let Some(slider) = self.min_slider.as_mut() {
            slider.set_value_no_signal(min);
        }

        if let Some(slider) = self.max_slider.as_mut() {
            slider.set_value_no_signal(max);
        }
    }
}

#[godot_api]
impl NaughtyMinMaxSlider {
    #[func]
    fn on_min_changed(&mut self, value: f64) {
        let Some(max) = self.max_slider.as_ref().map(|slider| slider.get_value()) else {
            return;
        };

        if value > max
            && let Some(slider) = self.min_slider.as_mut()
        {
            slider.set_value_no_signal(max);
        }

        self.emit_value();
    }

    #[func]
    fn on_max_changed(&mut self, value: f64) {
        let Some(min) = self.min_slider.as_ref().map(|slider| slider.get_value()) else {
            return;
        };

        if value < min
            && let Some(slider) = self.max_slider.as_mut()
        {
            slider.set_value_no_signal(min);
        }

        self.emit_value();
    }
}

impl NaughtyMinMaxSlider {
    fn setup(&mut self, min: f64, max: f64, is_integer: bool) {
        let mut min_slider = create_slider(min, max, is_integer);
        let mut max_slider = create_slider(min, max, is_integer);
        min_slider.set_label("Min");
        max_slider.set_label("Max");

        let object = self.to_gd();
        min_slider.connect("value_changed", &object.callable("on_min_changed"));
        max_slider.connect("value_changed", &object.callable("on_max_changed"));

        let mut row = HBoxContainer::new_alloc();
        row.set_h_size_flags(SizeFlags::EXPAND_FILL);
        row.add_child(&min_slider);
        row.add_child(&max_slider);

        let mut base = self.base_mut();
        base.add_child(&row);
        base.add_focusable(&min_slider.clone().upcast::<Control>());
        base.add_focusable(&max_slider.clone().upcast::<Control>());
        drop(base);

        self.min_slider = Some(min_slider);
        self.max_slider = Some(max_slider);
        self.is_integer = is_integer;
    }

    fn emit_value(&mut self) {
        let (Some(min_slider), Some(max_slider)) =
            (self.min_slider.as_ref(), self.max_slider.as_ref())
        else {
            return;
        };

        let min = min_slider.get_value();
        let max = max_slider.get_value();
        let value = if self.is_integer {
            Vector2i::new(min as i32, max as i32).to_variant()
        } else {
            Vector2::new(min as f32, max as f32).to_variant()
        };

        let property = self.base().get_edited_property();
        self.base_mut().emit_changed(&property, &value);
    }
}

fn create_slider(min: f64, max: f64, is_integer: bool) -> Gd<EditorSpinSlider> {
    let mut slider = EditorSpinSlider::new_alloc();
    slider.set_min(min);
    slider.set_max(max);
    slider.set_step(if is_integer { 1.0 } else { 0.1 });
    slider.set_h_size_flags(SizeFlags::EXPAND_FILL);
    slider
}

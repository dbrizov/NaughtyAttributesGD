use na_core::attributes::decorator::DecoratorAttribute;
use na_core::attributes::drawer::DrawerAttribute;
use na_core::attributes::validator::ValidatorAttribute;

use crate::decorators::IDecorator;
use crate::drawers::IDrawer;
use crate::validators::IValidator;

pub fn get_decorator(attribute: &DecoratorAttribute) -> &dyn IDecorator {
    match attribute {
        DecoratorAttribute::HorizontalLine(horizontal_line) => horizontal_line,
        DecoratorAttribute::MessageBox(message_box) => message_box,
    }
}

pub fn get_drawer(attribute: &DrawerAttribute) -> &dyn IDrawer {
    match attribute {
        DrawerAttribute::Dropdown(dropdown) => dropdown,
        DrawerAttribute::MinMaxSlider(min_max_slider) => min_max_slider,
    }
}

pub fn get_validator(attribute: &ValidatorAttribute) -> &dyn IValidator {
    match attribute {
        ValidatorAttribute::MinValue(min_value) => min_value,
        ValidatorAttribute::MaxValue(max_value) => max_value,
        ValidatorAttribute::Require(require) => require,
    }
}

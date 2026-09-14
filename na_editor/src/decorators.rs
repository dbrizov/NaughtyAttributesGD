mod horizontal_line;
mod message_box;

use godot::classes::VBoxContainer;
use godot::prelude::*;

pub trait IDecorator {
    fn decorate(&self, container: &mut Gd<VBoxContainer>, object: &Gd<Object>);
}

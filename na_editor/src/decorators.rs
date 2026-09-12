mod info_box;

use godot::classes::Control;
use godot::prelude::*;

pub trait IDecorator {
    fn decorate(&self, container: &mut Gd<Control>, object: &Gd<Object>);
}

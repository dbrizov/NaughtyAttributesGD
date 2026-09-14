use godot::classes::VBoxContainer;
use godot::prelude::*;

use na_core::attributes::decorator::message_box::MessageBox;

use crate::decorators::IDecorator;
use crate::message_bubble::MessageBubble;

impl IDecorator for MessageBox {
    fn decorate(&self, container: &mut Gd<VBoxContainer>, _object: &Gd<Object>) {
        let mut bubble = MessageBubble::new(self.severity);
        bubble.set_text(&self.text);
        container.add_child(bubble.get_panel());
    }
}

use godot::classes::EditorInterface;
use godot::classes::undo_redo::MergeMode;
use godot::prelude::*;

struct PropertyEdit {
    name: StringName,
    old_value: Variant,
    new_value: Variant,
}

pub struct PropertyEditAction {
    object: Gd<Object>,
    edits: Vec<PropertyEdit>,
}

impl PropertyEditAction {
    pub fn new(object: &Gd<Object>) -> Self {
        Self {
            object: object.clone(),
            edits: Vec::new(),
        }
    }

    pub fn set_property_value(&mut self, name: &StringName, value: &Variant) {
        let current_value = self.object.get(name);
        if current_value == *value {
            return;
        }

        self.object.set(name, value);
        let new_value = self.object.get(name);

        match self.edits.iter_mut().find(|edit| &edit.name == name) {
            Some(edit) => edit.new_value = new_value,
            None => self.edits.push(PropertyEdit {
                name: name.clone(),
                old_value: current_value,
                new_value,
            }),
        }
    }

    /// Returns the names of the changed properties, or `None` if nothing changed.
    pub fn commit(self, action_name: &str) -> Option<Vec<StringName>> {
        let edits: Vec<PropertyEdit> = self
            .edits
            .into_iter()
            .filter(|edit| edit.old_value != edit.new_value)
            .collect();

        if edits.is_empty() {
            return None;
        }

        let changed_properties = Some(edits.iter().map(|edit| edit.name.clone()).collect());

        let Some(mut undo_redo) = EditorInterface::singleton().get_editor_undo_redo() else {
            return changed_properties;
        };

        undo_redo
            .create_action_ex(action_name)
            .merge_mode(MergeMode::ALL)
            .backward_undo_ops(true)
            .done();

        for edit in &edits {
            undo_redo.add_do_property(&self.object, &edit.name, &edit.new_value);
            undo_redo.add_undo_property(&self.object, &edit.name, &edit.old_value);
        }
        undo_redo.commit_action_ex().execute(false).done();

        changed_properties
    }
}

use std::collections::HashMap;
use std::time::{Duration, Instant};

use godot::classes::undo_redo::MergeMode;
use godot::classes::{EditorInterface, EditorUndoRedoManager, UndoRedo};
use godot::obj::InstanceId;
use godot::prelude::*;

use na_core::descriptor::ClassDescriptor;

const MERGE_WINDOW: Duration = Duration::from_millis(800);

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
            object: Gd::clone(object),
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
        let object = Gd::clone(&self.object);
        let edits = self.get_changes();
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
            undo_redo.add_do_property(&object, &edit.name, &edit.new_value);
            undo_redo.add_undo_property(&object, &edit.name, &edit.old_value);
        }
        undo_redo.commit_action_ex().execute(false).done();

        changed_properties
    }

    pub fn add_to(
        self,
        undo_redo: &mut Gd<EditorUndoRedoManager>,
        session: &EditSession,
        requested_value: &Variant,
    ) {
        let object = Gd::clone(&self.object);
        let mut history = get_history(undo_redo, &object);
        let edits = self.revert();

        for edit in &edits {
            if edit.name == session.name {
                let is_corrected = edit.new_value != *requested_value;
                if is_corrected {
                    undo_redo.add_do_property(&object, &edit.name, &edit.new_value);
                }

                continue;
            }

            if edit.old_value == edit.new_value {
                continue;
            }

            undo_redo.add_do_property(&object, &edit.name, &edit.new_value);

            let origin = session.origins.get(&edit.name).unwrap_or(&edit.old_value);
            if let Some(history) = history.as_mut() {
                history.start_force_keep_in_merge_ends();
            }

            undo_redo.add_undo_property(&object, &edit.name, origin);
            if let Some(history) = history.as_mut() {
                history.end_force_keep_in_merge_ends();
            }
        }
    }

    fn revert(self) -> Vec<PropertyEdit> {
        let mut object = Gd::clone(&self.object);
        for edit in self.edits.iter().rev() {
            object.set(&edit.name, &edit.old_value);
        }

        self.edits
    }

    fn get_changes(self) -> Vec<PropertyEdit> {
        self.edits
            .into_iter()
            .filter(|edit| edit.old_value != edit.new_value)
            .collect()
    }
}

/// The claimed properties' values at the start of one merged inspector action.
pub struct EditSession {
    object: InstanceId,
    name: StringName,
    origins: HashMap<StringName, Variant>,
    last_edit: Instant,
    next_version: u64,
}

impl EditSession {
    pub fn resume<'a>(
        slot: &'a mut Option<Self>,
        undo_redo: &Gd<EditorUndoRedoManager>,
        object: &Gd<Object>,
        class: &ClassDescriptor,
        name: &StringName,
    ) -> &'a Self {
        let version = get_history(undo_redo, object)
            .map(|history| history.get_version())
            .unwrap_or_default();

        if let Some(session) = slot.as_mut()
            && session.continues(object, name, version)
        {
            session.advance(version);
        } else {
            *slot = None;
        }

        slot.get_or_insert_with(|| Self::begin(object, class, name, version))
    }

    fn begin(
        object: &Gd<Object>,
        class: &ClassDescriptor,
        name: &StringName,
        version: u64,
    ) -> Self {
        let origins = class
            .properties
            .iter()
            .filter(|property| property.claimed)
            .map(|property| (property.name.clone(), object.get(&property.name)))
            .collect();

        Self {
            object: object.instance_id(),
            name: name.clone(),
            origins,
            last_edit: Instant::now(),
            next_version: version + 1,
        }
    }

    fn continues(&self, object: &Gd<Object>, name: &StringName, version: u64) -> bool {
        self.object == object.instance_id()
            && &self.name == name
            && self.last_edit.elapsed() < MERGE_WINDOW
            && self.next_version == version
    }

    fn advance(&mut self, version: u64) {
        self.last_edit = Instant::now();
        self.next_version = version;
    }
}

fn get_history(undo_redo: &Gd<EditorUndoRedoManager>, object: &Gd<Object>) -> Option<Gd<UndoRedo>> {
    let history_id = undo_redo.get_object_history_id(object);
    undo_redo.get_history_undo_redo(history_id)
}

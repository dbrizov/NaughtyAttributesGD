use std::collections::HashMap;
use std::time::{Duration, Instant};

use godot::classes::undo_redo::MergeMode;
use godot::classes::{EditorInterface, EditorUndoRedoManager, UndoRedo};
use godot::obj::InstanceId;
use godot::prelude::*;

use na_core::descriptor::ClassDescriptor;

/// Godot's own merge timeout in `UndoRedo::create_action`. Must match the engine.
const GODOT_MERGE_WINDOW: Duration = Duration::from_millis(800);

struct PropertyChange {
    name: StringName,
    old_value: Variant,
    new_value: Variant,
}

/// A batch of property changes, applied at once and then committed as one undo action.
pub struct PropertyEditAction {
    object: Gd<Object>,
    changes: Vec<PropertyChange>,
}

impl PropertyEditAction {
    pub fn new(object: &Gd<Object>) -> Self {
        Self {
            object: Gd::clone(object),
            changes: Vec::new(),
        }
    }

    pub fn set_property_value(&mut self, name: &StringName, value: &Variant) {
        let current_value = self.object.get(name);
        if current_value == *value {
            return;
        }

        self.object.set(name, value);
        let new_value = self.object.get(name);

        match self.changes.iter_mut().find(|change| &change.name == name) {
            Some(change) => change.new_value = new_value,
            None => self.changes.push(PropertyChange {
                name: name.clone(),
                old_value: current_value,
                new_value,
            }),
        }
    }

    /// Returns the names of the changed properties, or `None` if nothing changed.
    pub fn commit(self, action_name: &str) -> Option<Vec<StringName>> {
        let object = Gd::clone(&self.object);
        let changes = self.get_changes();
        if changes.is_empty() {
            return None;
        }

        let changed_properties = Some(changes.iter().map(|change| change.name.clone()).collect());

        let Some(mut undo_redo) = EditorInterface::singleton().get_editor_undo_redo() else {
            return changed_properties;
        };

        undo_redo
            .create_action_ex(action_name)
            .merge_mode(MergeMode::ALL)
            .backward_undo_ops(true)
            .done();

        for change in &changes {
            undo_redo.add_do_property(&object, &change.name, &change.new_value);
            undo_redo.add_undo_property(&object, &change.name, &change.old_value);
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
        let changes = self.revert();

        for change in &changes {
            if change.name == session.name {
                let is_corrected = change.new_value != *requested_value;
                if is_corrected {
                    undo_redo.add_do_property(&object, &change.name, &change.new_value);
                }

                continue;
            }

            if change.old_value == change.new_value {
                continue;
            }

            undo_redo.add_do_property(&object, &change.name, &change.new_value);

            let origin = session
                .origins
                .get(&change.name)
                .unwrap_or(&change.old_value);
            if let Some(history) = history.as_mut() {
                history.start_force_keep_in_merge_ends();
            }

            undo_redo.add_undo_property(&object, &change.name, origin);
            if let Some(history) = history.as_mut() {
                history.end_force_keep_in_merge_ends();
            }
        }
    }

    fn revert(self) -> Vec<PropertyChange> {
        let mut object = Gd::clone(&self.object);
        for change in self.changes.iter().rev() {
            object.set(&change.name, &change.old_value);
        }

        self.changes
    }

    fn get_changes(self) -> Vec<PropertyChange> {
        self.changes
            .into_iter()
            .filter(|change| change.old_value != change.new_value)
            .collect()
    }
}

/// The claimed properties' values at the start of one merged inspector action.
pub struct EditSession {
    object: InstanceId,
    name: StringName,
    origins: HashMap<StringName, Variant>,
    last_edit_time: Instant,
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
            last_edit_time: Instant::now(),
            next_version: version + 1,
        }
    }

    fn continues(&self, object: &Gd<Object>, name: &StringName, version: u64) -> bool {
        self.object == object.instance_id()
            && &self.name == name
            && self.last_edit_time.elapsed() < GODOT_MERGE_WINDOW
            && self.next_version == version
    }

    fn advance(&mut self, version: u64) {
        self.last_edit_time = Instant::now();
        self.next_version = version;
    }
}

fn get_history(undo_redo: &Gd<EditorUndoRedoManager>, object: &Gd<Object>) -> Option<Gd<UndoRedo>> {
    let history_id = undo_redo.get_object_history_id(object);
    undo_redo.get_history_undo_redo(history_id)
}

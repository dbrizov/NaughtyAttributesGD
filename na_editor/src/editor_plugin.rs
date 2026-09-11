use godot::classes::notify::NodeNotification;
use godot::classes::{EditorInspectorPlugin, EditorInterface, EditorPlugin, IEditorPlugin};
use godot::prelude::*;
use godot::signal::ConnectHandle;

use na_logging::na_print;

use crate::editor_inspector_plugin::NaughtyEditorInspectorPlugin;

#[derive(GodotClass)]
#[class(tool, init, base = EditorPlugin)]
pub struct NaughtyEditorPlugin {
    inspector_plugin: Option<Gd<NaughtyEditorInspectorPlugin>>,
    property_edited_handle: Option<ConnectHandle>,
    version_changed_handle: Option<ConnectHandle>,
    is_refreshing: bool,
    base: Base<EditorPlugin>,
}

#[godot_api]
impl IEditorPlugin for NaughtyEditorPlugin {
    fn enter_tree(&mut self) {
        na_print!("Editor plugin ready");

        let plugin = NaughtyEditorInspectorPlugin::new_gd();
        self.base_mut()
            .add_inspector_plugin(&plugin.clone().upcast::<EditorInspectorPlugin>());
        self.inspector_plugin = Some(plugin);

        self.connect_property_edited();
        self.connect_version_changed();
    }

    fn exit_tree(&mut self) {
        self.disconnect_version_changed();
        self.disconnect_property_edited();

        if let Some(plugin) = self.inspector_plugin.take() {
            self.base_mut()
                .remove_inspector_plugin(&plugin.upcast::<EditorInspectorPlugin>());
        }
    }

    fn on_notification(&mut self, what: NodeNotification) {
        if what == NodeNotification::EXTENSION_RELOADED {
            self.property_edited_handle = None;
            self.version_changed_handle = None;
            self.connect_property_edited();
            self.connect_version_changed();
        }
    }
}

impl NaughtyEditorPlugin {
    fn connect_property_edited(&mut self) {
        if self.property_edited_handle.is_some() {
            return;
        }

        let Some(inspector) = EditorInterface::singleton().get_inspector() else {
            return;
        };

        let handle = inspector
            .signals()
            .property_edited()
            .connect_other(&*self, Self::on_property_edited);

        self.property_edited_handle = Some(handle);
    }

    fn disconnect_property_edited(&mut self) {
        if let Some(handle) = self.property_edited_handle.take()
            && handle.is_connected()
        {
            handle.disconnect();
        }
    }

    fn on_property_edited(&mut self, _property: GString) {
        if self.is_refreshing {
            return;
        }

        let Some(mut plugin) = self.inspector_plugin.clone() else {
            return;
        };

        self.is_refreshing = true;
        plugin.call_deferred("refresh_conditions", &[]);
        self.is_refreshing = false;
    }

    fn connect_version_changed(&mut self) {
        if self.version_changed_handle.is_some() {
            return;
        }

        let Some(undo_redo) = EditorInterface::singleton().get_editor_undo_redo() else {
            return;
        };

        let handle = undo_redo
            .clone()
            .signals()
            .version_changed()
            .connect_other(&*self, Self::on_version_changed);

        self.version_changed_handle = Some(handle);
    }

    fn disconnect_version_changed(&mut self) {
        if let Some(handle) = self.version_changed_handle.take()
            && handle.is_connected()
        {
            handle.disconnect();
        }
    }

    fn on_version_changed(&mut self) {
        let Some(mut plugin) = self.inspector_plugin.clone() else {
            return;
        };

        plugin.call_deferred("sync_property_editors", &[]);
    }
}

use godot::classes::notify::NodeNotification;
use godot::classes::{EditorInspectorPlugin, EditorInterface, EditorPlugin, IEditorPlugin, Script};
use godot::prelude::*;
use godot::signal::ConnectHandle;

use crate::editor_inspector_plugin::NaughtyEditorInspectorPlugin;

#[derive(GodotClass)]
#[class(tool, init, base = EditorPlugin)]
pub struct NaughtyEditorPlugin {
    inspector_plugin: Option<Gd<NaughtyEditorInspectorPlugin>>,
    refresh_handle: Option<ConnectHandle>,
    saved_handle: Option<ConnectHandle>,
    undo_handle: Option<ConnectHandle>,
    refreshing: bool,
    base: Base<EditorPlugin>,
}

#[godot_api]
impl IEditorPlugin for NaughtyEditorPlugin {
    fn enter_tree(&mut self) {
        godot_print!("{} Editor plugin ready", na_core::LOG_PREFIX);

        let plugin = NaughtyEditorInspectorPlugin::new_gd();
        self.base_mut()
            .add_inspector_plugin(&plugin.clone().upcast::<EditorInspectorPlugin>());
        self.inspector_plugin = Some(plugin);

        self.connect_refresh();
        self.connect_saved();
        self.connect_undo();
    }

    fn exit_tree(&mut self) {
        self.disconnect_undo();
        self.disconnect_saved();
        self.disconnect_refresh();

        if let Some(plugin) = self.inspector_plugin.take() {
            self.base_mut()
                .remove_inspector_plugin(&plugin.upcast::<EditorInspectorPlugin>());
        }
    }

    fn on_notification(&mut self, what: NodeNotification) {
        if what == NodeNotification::EXTENSION_RELOADED {
            self.refresh_handle = None;
            self.saved_handle = None;
            self.undo_handle = None;
            self.connect_refresh();
            self.connect_saved();
            self.connect_undo();
        }
    }
}

impl NaughtyEditorPlugin {
    fn connect_refresh(&mut self) {
        if self.refresh_handle.is_some() {
            return;
        }

        let Some(inspector) = EditorInterface::singleton().get_inspector() else {
            return;
        };

        let handle = inspector
            .signals()
            .property_edited()
            .connect_other(&*self, Self::on_property_edited);

        self.refresh_handle = Some(handle);
    }

    fn disconnect_refresh(&mut self) {
        if let Some(handle) = self.refresh_handle.take()
            && handle.is_connected()
        {
            handle.disconnect();
        }
    }

    fn connect_saved(&mut self) {
        if self.saved_handle.is_some() {
            return;
        }

        let handle = self
            .base()
            .signals()
            .resource_saved()
            .connect_other(&*self, Self::on_resource_saved);

        self.saved_handle = Some(handle);
    }

    fn disconnect_saved(&mut self) {
        if let Some(handle) = self.saved_handle.take()
            && handle.is_connected()
        {
            handle.disconnect();
        }
    }

    fn on_resource_saved(&mut self, resource: Gd<Resource>) {
        if resource.try_cast::<Script>().is_err() {
            return;
        }

        if let Some(plugin) = self.inspector_plugin.clone() {
            plugin.bind().invalidate_cache();
        }
    }

    fn connect_undo(&mut self) {
        if self.undo_handle.is_some() {
            return;
        }

        let Some(undo_redo) = EditorInterface::singleton().get_editor_undo_redo() else {
            return;
        };

        let handle = undo_redo
            .clone()
            .signals()
            .version_changed()
            .connect_other(&*self, Self::on_undo_version_changed);

        self.undo_handle = Some(handle);
    }

    fn disconnect_undo(&mut self) {
        if let Some(handle) = self.undo_handle.take()
            && handle.is_connected()
        {
            handle.disconnect();
        }
    }

    fn on_undo_version_changed(&mut self) {
        let Some(mut plugin) = self.inspector_plugin.clone() else {
            return;
        };

        plugin.call_deferred("sync_property_editors", &[]);
    }

    fn on_property_edited(&mut self, _property: GString) {
        if self.refreshing {
            return;
        }

        let Some(mut plugin) = self.inspector_plugin.clone() else {
            return;
        };

        self.refreshing = true;
        plugin.call_deferred("refresh_conditions", &[]);
        self.refreshing = false;
    }
}

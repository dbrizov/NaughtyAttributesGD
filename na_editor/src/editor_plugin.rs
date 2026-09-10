use godot::classes::notify::NodeNotification;
use godot::classes::{EditorInspectorPlugin, EditorInterface, EditorPlugin, IEditorPlugin, Script};
use godot::prelude::*;
use godot::signal::ConnectHandle;

use crate::editor_inspector_plugin::NaughtyEditorInspectorPlugin;

#[derive(GodotClass)]
#[class(tool, init, base = EditorPlugin)]
pub struct NaughtyEditorPlugin {
    inspector_plugin: Option<Gd<NaughtyEditorInspectorPlugin>>,
    property_edited_handle: Option<ConnectHandle>,
    resource_saved_handle: Option<ConnectHandle>,
    version_changed_handle: Option<ConnectHandle>,
    is_refreshing: bool,
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

        self.connect_property_edited();
        self.connect_resource_saved();
        self.connect_version_changed();
    }

    fn exit_tree(&mut self) {
        self.disconnect_version_changed();
        self.disconnect_resource_saved();
        self.disconnect_property_edited();

        if let Some(plugin) = self.inspector_plugin.take() {
            self.base_mut()
                .remove_inspector_plugin(&plugin.upcast::<EditorInspectorPlugin>());
        }
    }

    fn on_notification(&mut self, what: NodeNotification) {
        if what == NodeNotification::EXTENSION_RELOADED {
            self.property_edited_handle = None;
            self.resource_saved_handle = None;
            self.version_changed_handle = None;
            self.connect_property_edited();
            self.connect_resource_saved();
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

    fn connect_resource_saved(&mut self) {
        if self.resource_saved_handle.is_some() {
            return;
        }

        let handle = self
            .base()
            .signals()
            .resource_saved()
            .connect_other(&*self, Self::on_resource_saved);

        self.resource_saved_handle = Some(handle);
    }

    fn disconnect_resource_saved(&mut self) {
        if let Some(handle) = self.resource_saved_handle.take()
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

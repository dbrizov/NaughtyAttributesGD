use godot::classes::notify::NodeNotification;
use godot::classes::{EditorInspectorPlugin, EditorInterface, EditorPlugin, IEditorPlugin};
use godot::prelude::*;
use godot::signal::ConnectHandle;

use crate::naughty_editor_inspector_plugin::NaughtyEditorInspectorPlugin;

#[derive(GodotClass)]
#[class(tool, init, base = EditorPlugin)]
struct NaughtyEditorPlugin {
    inspector_plugin: Option<Gd<NaughtyEditorInspectorPlugin>>,
    refresh_handle: Option<ConnectHandle>,
    refreshing: bool,
    base: Base<EditorPlugin>,
}

#[godot_api]
impl IEditorPlugin for NaughtyEditorPlugin {
    fn enter_tree(&mut self) {
        let plugin = NaughtyEditorInspectorPlugin::new_gd();
        self.base_mut()
            .add_inspector_plugin(&plugin.clone().upcast::<EditorInspectorPlugin>());
        self.inspector_plugin = Some(plugin);

        self.connect_refresh();
    }

    fn exit_tree(&mut self) {
        self.disconnect_refresh();

        if let Some(plugin) = self.inspector_plugin.take() {
            self.base_mut()
                .remove_inspector_plugin(&plugin.upcast::<EditorInspectorPlugin>());
        }
    }

    fn on_notification(&mut self, what: NodeNotification) {
        if what == NodeNotification::EXTENSION_RELOADED {
            self.refresh_handle = None;
            self.connect_refresh();
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

    fn on_property_edited(&mut self, _property: GString) {
        if self.refreshing {
            return;
        }

        let Some(inspector) = EditorInterface::singleton().get_inspector() else {
            return;
        };

        let Some(mut edited) = inspector.get_edited_object() else {
            return;
        };

        self.refreshing = true;
        edited.notify_property_list_changed();
        self.refreshing = false;
    }
}

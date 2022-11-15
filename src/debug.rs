use bevy::prelude::*;
//use bevy_inspector_egui::{WorldInspectorPlugin, RegisterInspectable};

use crate::prelude::*;
pub struct DebugPlugin;

impl Plugin for DebugPlugin{

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }

    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions){
            //app.add_plugin(WorldInspectorPlugin::new())//todo not working in bevy 0_9 needed update
            //.register_inspectable::<Player>();
        }
    }
}
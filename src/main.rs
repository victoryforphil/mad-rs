//! Shows how to render simple primitive shapes with a single color.

use bevy::{prelude::*};
use bevy_egui::{EguiPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;


mod components;
mod bundles;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup)
        .add_system(
            systems::ui::ui_create_unit::sys_ui_setup,
        )
        .add_system(
            systems::ui::ui_selected_unit::sys_ui_setup,)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());

  
    
}
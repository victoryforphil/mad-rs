//! Shows how to render simple primitive shapes with a single color.

use bevy::{prelude::*};
use bevy_egui::{EguiPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::prelude::*;
use components::{selectable::SelectableComponent, inventory::InventoryComponent};
use systems::init_system;
use units::{silo::SiloUnit, UnitSpawners};



mod components;
mod bundles;
mod systems;
mod events;
mod units;
fn main() {
    let mut app = App::new();
    app
        .add_event::<events::spawn::SpawnUnitEvent>()
        .add_event::<events::spawn::SpawnUnitRequstEvent>()
        .add_event::<events::input::SelectorUpdateEvent>()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugins(DefaultPickingPlugins)
        .add_startup_system(setup)
        .add_system(
            systems::ui::ui_create_unit::sys_ui_setup,
        )
        .add_system(
            systems::ui::ui_selected_unit::sys_ui_setup,);
    init_system(&mut app);
    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());
    //commands.spawn(RaycastPickCamera::default());
    //commands.spawn(SpritePick)

    commands.spawn(OnPointer::<Click>::target_component_mut::<SelectableComponent>(|click, selectable| {
        selectable.active = true;
    
    }));
    let mut inv = InventoryComponent::new(0);
    inv.add_unit(UnitSpawners::SiloUnit(SiloUnit {  }), 1);
    commands.spawn(inv);
  
    
}



use bevy::{prelude::{Commands, AssetServer, Res, Query, EventWriter}};
use bevy_egui::{egui, EguiContexts};
use strum::IntoEnumIterator;

use crate::{ components::{ inventory::InventoryComponent}, units::{  UnitSpawners, spawnable::{Spawnable, SpawnContext}}, events::spawn::SpawnUnitRequstEvent};

pub fn sys_ui_setup(mut contexts: EguiContexts, mut commands: Commands, assets: Res<AssetServer>, query: Query<(&InventoryComponent)>, mut spawn_event: EventWriter<SpawnUnitRequstEvent>){
    egui::Window::new("Create Unit").show(contexts.ctx_mut(), |ui| {
        ui.label("Available Units:");
        
        // Iterate and display all options in Units enum
        for unit in UnitSpawners::iter() {
           let unit = UnitSpawners::from(unit);
           if ui.button(unit.get_build_info().name).clicked() {
                spawn_event.send(SpawnUnitRequstEvent::new(0, UnitSpawners::SiloUnit(crate::units::silo::SiloUnit {  }), (5.0, 5.0)));
           }
          
        }
    });
}
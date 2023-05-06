
use bevy::prelude::{Commands, AssetServer, Res, Query};
use bevy_egui::{egui, EguiContexts};

use crate::components::{selectable::SelectableComponent, unit_info::NameComponent};


// Query for all selected components, with optional Name and Health components
pub fn sys_ui_setup(mut contexts: EguiContexts, mut commands: Commands, assets: Res<AssetServer>, query: Query<(&SelectableComponent, Option<&NameComponent>)>){
    egui::Window::new("Selected Unit").show(contexts.ctx_mut(), |ui| {
        ui.label("Selected Unit:");

        for (selected, name) in query.iter() {
            
            ui.label(format!("Client: {}", selected.client));
            ui.label(format!("On the go: {}", selected.active));

            if let Some(name) = name {
                ui.label(format!("Name: {}", name.name));

            }


        }
       
    });
}
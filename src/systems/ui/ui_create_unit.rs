
use bevy::prelude::{Commands, AssetServer, Res};
use bevy_egui::{egui, EguiContexts};

use crate::{bundles::silo_bundle::SiloBundle, components::selected::SelectecComponent};

pub fn sys_ui_setup(mut contexts: EguiContexts, mut commands: Commands, assets: Res<AssetServer>){
    egui::Window::new("Create Unit").show(contexts.ctx_mut(), |ui| {
        ui.label("Available Units:");

        if ui.button("Create Unit").clicked() {
            let mut entity = commands.spawn(SiloBundle::new(assets));
            
            entity.insert(SelectecComponent::new(0, false));
        }
    });
}
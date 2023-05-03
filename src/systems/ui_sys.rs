
use bevy_egui::{egui, EguiContexts};

pub fn sys_ui_setup(mut contexts: EguiContexts){
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.label("world");
    });
}
use bevy::prelude::*;


pub mod ui;
pub mod spawn_sys;


pub fn init_system(app: &mut App){
   app.add_system( spawn_sys::sys_check_spawn_request);
}
// Trait to define spawn behavior for a give unit
/*  Such info defined:
    - Name / Desc
    - Cost
    - Bevy Bundle / Spawn logic
*/

use std::{sync::Arc, cell::RefCell};

use bevy::prelude::*;
use enum_dispatch::enum_dispatch;

pub struct UnitSpawnInfo{
    pub name: String,
    pub desc: String,
    pub cost: u32
}
// Bevy commands and asset server context for spawning
pub struct SpawnContext<'a> {
    pub commands: &'a mut Commands<'a, 'a>,
    pub asset_server: Arc<AssetServer>,
}


#[enum_dispatch]
pub trait Spawnable{

    fn get_build_info(&self) -> UnitSpawnInfo;
    fn spawn(&self, context: &mut SpawnContext){
        
    }
}
// Path: src/units/silo.irs
// Functions that define spawn related functionality

use std::{cell::RefCell, sync::Arc};

use bevy::prelude::*;

use crate::{events::spawn::{SpawnUnitRequstEvent, SpawnUnitEvent}, units::{spawnable::{Spawnable, SpawnContext}, UnitSpawners, silo::SiloUnit}};


pub fn sys_check_spawn_request(
    mut req_event: EventReader<SpawnUnitRequstEvent>, 
    mut spawn_event: EventWriter<SpawnUnitEvent>
){
   

    for req in req_event.iter() {
        info!("[SpawnSys] Spawn request recived for {:?}", req);
        info!("[SpawnSys] Spawn request approved for {:?}", req);

        spawn_event.send(req.get_approved());
    }
}

pub fn sys_spawn_bundle(mut commands: Commands, mut event: EventReader<SpawnUnitEvent>, asset_server: Res<AssetServer>){
    
    for spawn in event.iter() {
      
        info!("[SpawnSys] Spawning bundles {:?}", spawn);
        
        let spawner =  UnitSpawners::SiloUnit(SiloUnit{});
        spawner.spawn(&mut SpawnContext {
            commands: &mut commands,
            asset_server: Arc::new(asset_server.clone()),
        });
    }
}
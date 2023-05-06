
// Silo Unit

use std::sync::Arc;

use crate::components::{unit_info::NameComponent, owner::Owner, unit_silo::UnitSilo, map_pos::MapPos, selectable::SelectableComponent};

use super::spawnable::{Spawnable, UnitSpawnInfo, SpawnContext};
use bevy::prelude::*;
use bevy_mod_picking::{PickableBundle, prelude::RaycastPickTarget};
#[derive(Debug, PartialEq, Default, Clone)]
pub struct SiloUnit{

}

impl Spawnable for SiloUnit{
    fn get_build_info(&self) -> super::spawnable::UnitSpawnInfo {
        UnitSpawnInfo{
            name: "Silo".to_string(),
            desc: "Silo description".to_string(),
            cost: 1_000_000
        }
    }

    fn spawn(&self,  context: &mut SpawnContext) {
      
        let asset_server = Arc::clone(&context.asset_server);

        
        let info = self.get_build_info();
        let bundle =(
            NameComponent{
                name: info.name,
                desc: info.desc,
            },
            Owner::default(),
            UnitSilo{
                state: crate::components::unit_silo::UnitSiloState::Idle
            },
            MapPos::new(0,0),
            SpriteBundle {
                texture: asset_server.load("sprites/silo-default.png"),
                ..default()
            },
            PickableBundle::default(),
            RaycastPickTarget::default(),
            SelectableComponent { client: 0, active: false, moving: false },
        );

        context.commands.spawn(bundle);
    }
}
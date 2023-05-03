use bevy::{prelude::*};
use crate::components::{owner::Owner, unit_silo::UnitSilo, map_pos::MapPos, unit_info::NameComponent};

#[derive(Bundle)]
pub struct SiloBundle{
    pub name: NameComponent,
    pub owner: Owner,
    pub unit_silo: UnitSilo,
    pub location: MapPos,
    pub sprite: SpriteBundle,
}


impl SiloBundle{
    pub fn new(asset_server: Res<AssetServer>) -> Self{
        SiloBundle{
            name: NameComponent{
                name: "Silo",
                desc: "A silo to store units",
            },
            owner: Owner::default(),
            unit_silo: UnitSilo{
                state: crate::components::unit_silo::UnitSiloState::Idle
            },
            location: MapPos::new(0,0),
            sprite: SpriteBundle {
                texture: asset_server.load("sprites/units/silo-default.png"),
                ..default()
            }
        }
    }
}
    


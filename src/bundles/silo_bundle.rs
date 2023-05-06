use bevy::{prelude::*};
use bevy_mod_picking::{PickableBundle, prelude::RaycastPickTarget};
use crate::components::{owner::Owner, unit_silo::UnitSilo, map_pos::MapPos, unit_info::NameComponent, selectable::SelectableComponent};

#[derive(Bundle)]
pub struct SiloBundle{
    pub name: NameComponent,
    pub owner: Owner,
    pub unit_silo: UnitSilo,
    pub location: MapPos,
    pub sprite: SpriteBundle,
    pub pick_bundle: PickableBundle,
    pub pick_target: RaycastPickTarget,
    pub selectable: SelectableComponent,
}


impl SiloBundle{
    pub fn new(asset_server: Res<AssetServer>) -> Self{
        SiloBundle{
            name: NameComponent{
                name: "Silo".to_string(),
                desc: "A silo to store units".to_string(),
            },
            owner: Owner::default(),
            unit_silo: UnitSilo{
                state: crate::components::unit_silo::UnitSiloState::Idle
            },
            location: MapPos::new(0,0),
            sprite: SpriteBundle {
                texture: asset_server.load("sprites/units/silo-default.png"),
                ..default()
            },
            pick_bundle: PickableBundle::default(),
            pick_target: RaycastPickTarget::default(),
            selectable: SelectableComponent { client: 0, active: false, moving: false },
        }
    }
}
    


use bevy::{prelude::*};
use crate::components::{owner::Owner, unit_silo::UnitSilo};

#[derive(Bundle)]
pub struct SiloBundle{
    pub owner: Owner,
    pub unit_silo: UnitSilo,
    pub transform: TransformBundle,
    pub sprite: SpriteBundle,
}


impl Default for SiloBundle{
    fn default() -> Self{
        SiloBundle{
            owner: Owner::default(),
            unit_silo: UnitSilo{
                state: crate::components::unit_silo::UnitSiloState::Idle
            },
            transform: TransformBundle::default(),
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.25, 0.25, 0.75),
                    custom_size: Some(Vec2::new(50.0, 100.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(-50., 0., 0.)),
                ..default()
            }
        }
    }
}
    


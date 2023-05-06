
use enum_dispatch::enum_dispatch;
use strum_macros::EnumIter;
use bevy::prelude::*;

pub mod silo;

pub mod spawnable;

use spawnable::*;
use silo::SiloUnit;

#[enum_dispatch(Spawnable)]
#[derive(EnumIter, Debug, Clone)]
pub enum UnitSpawners{
    SiloUnit
}

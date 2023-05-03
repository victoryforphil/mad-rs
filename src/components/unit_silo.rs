// Component to represner a ICBM silo
//

use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum UnitSiloState{
    Idle,
    Launching,
    LaunchingComplete,
    Offline
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct UnitSilo{
    pub state : UnitSiloState
}
use bevy::prelude::*;

// Component to store who a given entity belongs to across a team and multiplayer context
#[derive(Component, Debug, Clone, Copy, PartialEq)] 
pub struct Owner{
    pub team: u8,
    pub client: u16
}

impl Owner{
    pub fn new(team: u8, client: u16) -> Self{
        Owner{
            team,
            client
        }
    }
}

impl Default for Owner{
    fn default() -> Self{
        Owner{
            team: 0,
            client: 0
        }
    }
}

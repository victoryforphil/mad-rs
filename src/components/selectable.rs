// On-the-go component defines a selected component that is being moved
// such as after creation for placement or after selection for movement
//
// Stores a client id to determine who's current selection tool (cursor/gamepad/etc)
// to use for movement

use bevy::prelude::Component;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct SelectableComponent{
    pub client: u16,
    pub active: bool,
    pub moving: bool
}

impl SelectableComponent{
    pub fn new(client: u16) -> Self{
        SelectableComponent{
            client,
            active: false,
            moving: false
        }
    }
}
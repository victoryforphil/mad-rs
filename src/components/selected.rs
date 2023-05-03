// On-the-go component defines a selected component that is being moved
// such as after creation for placement or after selection for movement
//
// Stores a client id to determine who's current selection tool (cursor/gamepad/etc)
// to use for movement

use bevy::prelude::Component;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct SelectecComponent{
    pub client: u16,
    pub on_the_go: bool
}

impl SelectecComponent{
    pub fn new(client: u16, on_go: bool) -> Self{
        SelectecComponent{
            client,
            on_the_go: on_go
        }
    }
}
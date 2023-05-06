// Component to store availables units for a player

use bevy::prelude::Component;

use crate::units::{ UnitSpawners};

#[derive(Component, Debug, Clone)]
pub struct InventoryComponent{
    pub client: u16,
    pub units: Vec<(UnitSpawners, u8)>
}

impl InventoryComponent{
    pub fn new(client: u16) -> Self{
        InventoryComponent{
            client,
            units: Vec::new()
        }
    }

    pub fn add_unit(&mut self, unit: UnitSpawners, count: u8){
        self.units.push((unit, count));
    }
}
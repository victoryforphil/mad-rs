use crate::units::UnitSpawners;

#[derive(Debug, Clone)]
pub struct SpawnUnitRequstEvent {
    pub client: u16,
    pub unit: UnitSpawners,
    pub position: (f32, f32),
}


#[derive(Debug, Clone)]
pub struct SpawnUnitEvent {
    pub client: u16,
    pub unit: UnitSpawners,
    pub position: (f32, f32),
}

impl SpawnUnitEvent {
    pub fn new(client: u16, unit: UnitSpawners, position: (f32, f32)) -> Self {
        Self {
            client,
            unit,
            position,
        }
    }
}

impl SpawnUnitRequstEvent {
    pub fn new(client: u16, unit: UnitSpawners, position: (f32, f32)) -> Self {
        SpawnUnitRequstEvent {
            client,
            unit,
            position,
        }
    }

    pub fn get_approved(&self) -> SpawnUnitEvent {
        SpawnUnitEvent {
            client: self.client.clone(),
            unit: self.unit.clone(),
            position: self.position.clone(),
        }
    }
}

// Unit tests
#[cfg(test)]
mod tests {
    use crate::units::silo::SiloUnit;

    use super::*;

    #[test]
    fn test_spawn_unit_request_event() {
        let event = SpawnUnitRequstEvent::new(1, UnitSpawners::SiloUnit(SiloUnit {}), (0.0, 0.0));
        assert_eq!(event.client, 1);

        assert_eq!(event.position, (0.0, 0.0));
    }

    #[test]
    fn test_spawn_unit_event() {
        let event = SpawnUnitEvent::new(1, UnitSpawners::SiloUnit(SiloUnit {  }), (0.0, 0.0));
        assert_eq!(event.client, 1);

        assert_eq!(event.position, (0.0, 0.0));
    }

    #[test]
    fn test_spawn_unit_request_event_get_approved() {
        let event = SpawnUnitRequstEvent::new(1, UnitSpawners::SiloUnit(SiloUnit {}), (0.0, 0.0));
        let approved = event.get_approved();
        assert_eq!(approved.client, 1);
        assert_eq!(approved.position, (0.0, 0.0));
    }
}

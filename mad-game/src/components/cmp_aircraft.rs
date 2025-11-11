use mad_core::geo::{FullGridLocationOwned, GridLocation};
#[derive(Debug, Clone, PartialEq)]
pub struct CmpAircraft {
    pub aircraft_num: u32,
    pub aircraft_type: String,
    pub aircraft_owner: String,
    pub aircraft_callsign: String,
    pub aircraft_waypoints: Vec<FullGridLocationOwned>,
    pub aircraft_waypoint_index: u32,
    pub aircraft_speed: f64,
    pub aircraft_heading: f64,
    pub aircraft_course: f64,
    pub aircraft_vspeed: f64,
    pub aircraft_altitude: f64,
    pub aircraft_fuel: f64,
    pub aircraft_fuel_consumption: f64,
    pub aircraft_fuel_remaining: f64,
}

impl CmpAircraft {
    pub fn new(
        aircraft_num: u32,
        aircraft_type: String,
        aircraft_owner: String,
        aircraft_callsign: String,
    ) -> Self {
        Self {
            aircraft_num,
            aircraft_type,
            aircraft_owner,
            aircraft_callsign,
            aircraft_waypoints: Vec::new(),
            aircraft_waypoint_index: 0,
            aircraft_speed: 0.0,
            aircraft_heading: 0.0,
            aircraft_course: 0.0,
            aircraft_vspeed: 0.0,
            aircraft_altitude: 0.0,
            aircraft_fuel: 0.0,
            aircraft_fuel_consumption: 0.0,
            aircraft_fuel_remaining: 0.0,
        }
    }
}

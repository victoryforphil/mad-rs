use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Component)]
pub struct MapPos {
    x: i32,
    y: i32,
    lat: f64,
    lon: f64,
    alt: f64,
}


impl MapPos {
    pub fn new(x: i32, y: i32) -> Self {
        MapPos {
            x,
            y,
            lat: 0.0,
            lon: 0.0,
            alt: 0.0,
        }
    }

    pub fn set_lat_lon_alt(&mut self, lat: f64, lon: f64, alt: f64) {
        self.lat = lat;
        self.lon = lon;
        self.alt = alt;
    }

    pub fn get_lat_lon_alt(&self) -> (f64, f64, f64) {
        (self.lat, self.lon, self.alt)
    }

    pub fn get_x_y(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn set_x_y(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}
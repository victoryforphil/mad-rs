use crate::geo::{GridZone, GridRegion, GridCell, GridLocation};

pub struct FullGridLocationOwned{
    pub zone: GridZone,
    pub region: GridRegion,
    pub cell: GridCell,
    pub location: GridLocation,
}

impl FullGridLocationOwned{
    pub fn new(zone: GridZone, region: GridRegion, cell: GridCell, location: GridLocation) -> Self{
        Self{ zone, region, cell, location }
    }
    
}

pub struct FullGridLocationRef<'a>{
    pub zone: &'a GridZone,
    pub region: &'a GridRegion,
    pub cell: &'a GridCell,
    pub location: &'a GridLocation,
}

impl<'a> FullGridLocationRef<'a>{
    pub fn new(zone: &'a GridZone, region: &'a GridRegion, cell: &'a GridCell, location: &'a GridLocation) -> Self{
        Self{ zone, region, cell, location }
    }
}
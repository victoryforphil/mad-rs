pub enum OSMPBDownloads {
    USPacific,
}

impl OSMPBDownloads {
    pub fn url(&self) -> &str {
        match self {
            OSMPBDownloads::USPacific => {
                "https://download.geofabrik.de/north-america/us-pacific-latest.osm.pbf"
            }
        }
    }

    pub fn filename(&self) -> &str {
        match self {
            OSMPBDownloads::USPacific => "us-pacific-latest.osm.pbf",
        }
    }
}

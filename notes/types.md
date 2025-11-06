# Grid Cell
- Represents a indexable MGRS grid cell index within a shared zone.
- Similar to a full MGRS coordinate but minus the zone / hemisphere and decimal precision.
- MGRS 1km resolution (3)

# Grid Zone
- Largest chunk unit
- Represented as UTM Zome (horizontally, n=60)

# Grid Region 
- Represents a 100km x 100km region within a grid zone
- Used to store a group of cells, such as in a file 
- MGRS Grid Identifier but represented as a single integer index countring from 0 to 1199
- Columns: 8 letters per zone (A-H, J-R, or S-Z)
- Rows: 20 letters (A to V, omitting I and O)

# GridLocation
- Represents a location within a grid zone with all needed information
- f64 position within the 1km GridCell

# FullGridLocation
- Combined version of GridCell, GridZone, and GridLocation
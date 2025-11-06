

# Storage Layers
## Layer 0: Zone
- Sharded based on 


# Core Storage Types

## Layers
- Stored per UTM grid cell 
- Consists of a list of GEOJSON objects / shapes
- Static type requested over TCP GET/SET interface

# Protocol

## Static API
- Consists of a Get/Set model used over reliable but slower connections such as TCP
- Used to get static infomration or initial data such as layer information.
- percision down to 1km square UTM cells

### PacketLayout
- write: bool
  - `true`: set
  - `false`: get
- key: string
- value: bytes (optional)
- index: [GridCell](./types.md#utm-cell)
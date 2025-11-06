use crate::LayerRow;

#[derive(Debug)]
pub struct LayerTable{
    layer_type: String,
    rows: Vec<LayerRow>,
}

impl LayerTable{
    pub fn new(layer_type: String) -> Self{
        Self{ layer_type, rows: Vec::new() }
    }
    pub fn type_name(&self) -> &String{
        &self.layer_type
    }
    pub fn rows(&self) -> &Vec<LayerRow>{
        &self.rows
    }
}
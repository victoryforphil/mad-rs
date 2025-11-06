use crate::EntityRow;



#[derive(Debug)]
pub struct EntityTable{
    pub component_type: String,
    pub rows: Vec<EntityRow>,
}

impl EntityTable{
    pub fn new(component_type: String) -> Self{
        Self{ component_type, rows: Vec::new() }
    }
    pub fn type_name(&self) -> &String{
        &self.component_type
    }
    pub fn rows(&self) -> &Vec<EntityRow>{
        &self.rows
    }
}
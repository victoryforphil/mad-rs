use mad_core::geo::GridCell;



pub enum EntityQuery{
    All,
    ListInCell(GridCell),
    ListInCells(Vec<GridCell>),
    GetByEntityIndex(u32),
}

impl EntityQuery{
    pub fn matches(&self, cell: &GridCell, entity_index: u32) -> bool{
        match self{
            EntityQuery::All => true,
            EntityQuery::ListInCell(query_cell) => query_cell == cell,
            EntityQuery::ListInCells(query_cells) => query_cells.contains(cell),
            EntityQuery::GetByEntityIndex(query_entity_index) => *query_entity_index == entity_index,
        }
    }   
}

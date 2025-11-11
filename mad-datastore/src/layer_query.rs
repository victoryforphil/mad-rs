use mad_core::geo::GridCell;

pub enum LayerQuery {
    All,
    ListInCell(GridCell),
    ListInCells(Vec<GridCell>),
}

impl LayerQuery {
    pub fn matches(&self, cell: &GridCell) -> bool {
        match self {
            LayerQuery::All => true,
            LayerQuery::ListInCell(query_cell) => query_cell == cell,
            LayerQuery::ListInCells(query_cells) => query_cells.contains(cell),
        }
    }
}

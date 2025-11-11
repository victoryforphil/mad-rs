use crate::{LayerQuery, LayerRow};

#[derive(Debug)]
pub struct LayerTable {
    layer_type: String,
    rows: Vec<LayerRow>,
}

impl LayerTable {
    pub fn new(layer_type: String) -> Self {
        Self {
            layer_type,
            rows: Vec::new(),
        }
    }
    pub fn type_name(&self) -> &String {
        &self.layer_type
    }
    pub fn rows(&self) -> &Vec<LayerRow> {
        &self.rows
    }

    pub fn append_row(&mut self, mut row: LayerRow) {
        row.index = self.rows.len() as u32;
        self.rows.push(row);
    }

    pub fn get_rows(&self) -> Vec<&LayerRow> {
        self.rows.iter().collect()
    }

    pub fn query(&self, query: LayerQuery) -> Vec<&LayerRow> {
        self.rows
            .iter()
            .filter(|row| query.matches(&row.cell))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use mad_core::geo::GridCell;

    use super::*;

    #[test]
    fn test_new() {
        let layer_table = LayerTable::new("test_layer".to_string());
        assert_eq!(layer_table.type_name(), "test_layer");
        assert_eq!(layer_table.rows().len(), 0);
    }
    #[test]
    fn test_append_row() {
        let mut layer_table = LayerTable::new("test_layer".to_string());
        let row = LayerRow::new(GridCell::new(0, 0));
        layer_table.append_row(row);
        assert_eq!(layer_table.rows().len(), 1);
    }

    #[test]
    fn test_get_rows() {
        let layer_table = LayerTable::new("test_layer".to_string());
        let rows = layer_table.get_rows();
        assert_eq!(rows.len(), 0);
    }

    #[test]
    fn test_query_all() {
        let mut layer_table = LayerTable::new("test_layer".to_string());
        layer_table.append_row(LayerRow::new(GridCell::new(0, 0)));
        layer_table.append_row(LayerRow::new(GridCell::new(1, 1)));
        let rows = layer_table.query(LayerQuery::All);
        assert_eq!(rows.len(), 2);
    }

    #[test]
    fn test_query_list_in_cell() {
        let mut layer_table = LayerTable::new("test_layer".to_string());
        layer_table.append_row(LayerRow::new(GridCell::new(0, 0)));
        layer_table.append_row(LayerRow::new(GridCell::new(1, 1)));
        let rows = layer_table.query(LayerQuery::ListInCell(GridCell::new(0, 0)));
        assert_eq!(rows.len(), 1);
    }

    #[test]
    fn test_query_list_in_cells() {
        let mut layer_table = LayerTable::new("test_layer".to_string());
        layer_table.append_row(LayerRow::new(GridCell::new(0, 0)));
        layer_table.append_row(LayerRow::new(GridCell::new(1, 1)));
        layer_table.append_row(LayerRow::new(GridCell::new(2, 2)));
        let rows = layer_table.query(LayerQuery::ListInCells(vec![
            GridCell::new(0, 0),
            GridCell::new(1, 1),
        ]));
        assert_eq!(rows.len(), 2);
    }
}

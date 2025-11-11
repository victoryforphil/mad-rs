use crate::{EntityQuery, EntityRow};

#[derive(Debug)]
pub struct EntityTable {
    pub component_type: String,
    pub rows: Vec<EntityRow>,
}

impl EntityTable {
    pub fn new(component_type: String) -> Self {
        Self {
            component_type,
            rows: Vec::new(),
        }
    }
    pub fn type_name(&self) -> &String {
        &self.component_type
    }
    pub fn rows(&self) -> &Vec<EntityRow> {
        &self.rows
    }

    pub fn append_row(&mut self, mut row: EntityRow) {
        row.index = self.rows.len() as u32;
        self.rows.push(row);
    }

    pub fn get_rows(&self) -> Vec<&EntityRow> {
        self.rows.iter().collect()
    }

    pub fn query(&self, query: EntityQuery) -> Vec<&EntityRow> {
        let rows = self
            .rows
            .iter()
            .filter(|row| query.matches(&row.cell, row.entity_index))
            .collect();
        rows
    }
}

#[cfg(test)]
mod tests {
    use mad_core::geo::GridCell;

    use super::*;

    #[test]
    fn test_new() {
        let entity_table = EntityTable::new("test_entity".to_string());
        assert_eq!(entity_table.type_name(), "test_entity");
        assert_eq!(entity_table.rows().len(), 0);
    }

    #[test]
    fn test_append_row() {
        let mut entity_table = EntityTable::new("test_entity".to_string());
        let row = EntityRow::new(GridCell::new(0, 0), 0);
        entity_table.append_row(row);
        assert_eq!(entity_table.rows().len(), 1);
    }

    #[test]
    fn test_get_rows() {
        let entity_table = EntityTable::new("test_entity".to_string());
        let rows = entity_table.get_rows();
        assert_eq!(rows.len(), 0);
    }

    #[test]
    fn test_query() {
        let mut entity_table = EntityTable::new("test_entity".to_string());

        let row = EntityRow::new(GridCell::new(0, 0), 0);
        entity_table.append_row(row);
        let rows = entity_table.query(EntityQuery::All);
        assert_eq!(rows.len(), 1);
    }

    #[test]
    fn test_query_list_in_cell() {
        let mut entity_table = EntityTable::new("test_entity".to_string());
        let row = EntityRow::new(GridCell::new(0, 0), 0);
        entity_table.append_row(row);
        let rows = entity_table.query(EntityQuery::ListInCell(GridCell::new(0, 0)));
        assert_eq!(rows.len(), 1);
    }

    #[test]
    fn test_query_list_in_cells() {
        let mut entity_table = EntityTable::new("test_entity".to_string());
        let row = EntityRow::new(GridCell::new(0, 0), 0);
        entity_table.append_row(row);
        let row = EntityRow::new(GridCell::new(1, 1), 1);
        entity_table.append_row(row);
        let rows = entity_table.query(EntityQuery::ListInCells(vec![
            GridCell::new(0, 0),
            GridCell::new(1, 1),
        ]));
        assert_eq!(rows.len(), 2);
    }

    #[test]
    fn test_query_get_by_entity_index() {
        let mut entity_table = EntityTable::new("test_entity".to_string());
        let row = EntityRow::new(GridCell::new(0, 0), 0);
        entity_table.append_row(row);
        let rows = entity_table.query(EntityQuery::GetByEntityIndex(0));
        assert_eq!(rows.len(), 1);
    }
}

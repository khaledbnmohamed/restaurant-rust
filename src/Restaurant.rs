use std::sync::{Arc, Mutex};
use crate::table::Table;

type TablePtr = Arc<Mutex<Table>>;

#[derive(Clone)]
pub struct Restaurant {
    tables: Vec<TablePtr>,
}

impl Restaurant {
    pub fn new(table_size: usize) -> Self {
        let tables: Vec<TablePtr> = (0..table_size as u32)
            .map(|tid| Arc::new(Mutex::new(Table::new(tid))))
            .collect();

        Self { tables }
    }

    pub fn get_table(&self, table_number: u32) -> TablePtr {
        Arc::clone(&self.tables[table_number as usize])
    }
}


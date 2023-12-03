#[derive(Debug, PartialEq)]
pub struct Item {
    pub(crate) item_id: u32,
    pub(crate) table_number: u32,
    pub(crate) preparation_time: u32,
}

impl Item {
    pub fn new(item_id: u32, table_number: u32, preparation_time: u32) -> Item {
        Item {
            item_id,
            table_number,
            preparation_time,
        }
    }

    #[cfg(test)]
    pub fn id(&self) -> u32 {
        self.item_id
    }

    pub fn print(&self) -> String {
        format!(
            "{{\"item_id\": {}, \"table_number\": {}, \"preparation_time\": {}}}",
            self.item_id, self.table_number, self.preparation_time
        )
    }
}

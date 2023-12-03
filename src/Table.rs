use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::collections::HashMap;
use crate::item::Item;

pub struct Table {
    table_number: u32,
    pub(crate) items: HashMap<u32, Item>,
    rng: StdRng,
}

impl Table {
    pub fn new(table_number: u32) -> Table {
        Table {
            table_number,
            items: HashMap::new(),
            rng: StdRng::from_entropy(),
        }
    }

    #[cfg(test)]
    pub fn id(&self) -> u32 {
        self.table_number
    }

    #[cfg(test)]
    pub fn items_size(&self) -> usize {
        self.items.len()
    }

    pub fn add_item(&mut self, item_id: u32) {
        let item = Item::new(item_id, self.table_number, self.rng.gen_range(5..15));
        self.items.insert(item_id, item);
    }

    pub fn check_item(&self, item_id: u32) -> Option<&Item> {
        self.items.get(&item_id)
    }

    pub fn remove_item(&mut self, item_id: u32) -> Option<Item> {
        self.items.remove(&item_id)
    }

    pub fn print_item(&self, item_id: u32) -> String {
        self.check_item(item_id)
            .map_or_else(|| "{\"msg\": \"not found\"}".to_owned(), |item| item.print())
    }

    pub fn print_items(&self) -> String {
        let items_str: Vec<String> = self.items.values().map(Item::print).collect();
        format!("[{}]", items_str.join(", "))
    }
}

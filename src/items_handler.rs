use crate::restaurant::Restaurant;

pub fn add_item(table_id: u32, item_data: &str, restaurant: Restaurant) -> String {
    if let Some(item_id) = item_data.split(',').next().and_then(|id| id.parse::<u32>().ok()) {
        let table = restaurant.get_table(table_id);
        table.lock().unwrap().add_item(item_id);
        "{\"msg\": \"success\"}".to_owned()
    } else {
        "{\"msg\": \"invalid input\"}".to_owned()
    }
}

pub fn remove_item(table_id: u32, item_id: u32, restaurant: Restaurant) -> String {
    let table = restaurant.get_table(table_id);
    let mut table_guard = table.lock().unwrap();
    if let Some(_) = table_guard.remove_item(item_id) {
        "{\"msg\": \"success\"}".to_owned()
    } else {
        "{\"msg\": \"cannot remove, item does not exist\"}".to_owned()
    }
}

pub fn get_all(table_id: u32, restaurant: Restaurant) -> String {
    let table = restaurant.get_table(table_id);
    let result = table.lock().unwrap().print_items();
    result
}

pub fn get_item(table_id: u32, item_id: u32, restaurant: Restaurant) -> String {
    let table = restaurant.get_table(table_id);
    let result = table.lock().unwrap().print_item(item_id);
    result
}

use crate::restaurant::Restaurant;

pub fn add_item(table_number: u32, item_data: &str, restaurant: Restaurant) -> String {
    if let Some(item_id) = item_data.split(',').next().and_then(|id| id.parse::<u32>().ok()) {
        let table = restaurant.get_table(table_number);
        table.lock().unwrap().add_item(item_id);
        "{\"msg\": \"success\"}".to_owned()
    } else {
        "{\"msg\": \"can't add item\"}".to_owned()
    }
}

pub fn delete_item(table_number: u32, item_id: u32, restaurant: Restaurant) -> String {
    let table = restaurant.get_table(table_number);
    let mut table_guard = table.lock().unwrap();
    if let Some(_) = table_guard.delete_item(item_id) {
        "{\"msg\": \"success\"}".to_owned()
    } else {
        "{\"msg\": \"error deleting item\"}".to_owned()
    }
}

pub fn get_all(table_number: u32, restaurant: Restaurant) -> String {
    let table = restaurant.get_table(table_number);
    let result = table.lock().unwrap().serializes();
    result
}

pub fn get_item(table_number: u32, item_id: u32, restaurant: Restaurant) -> String {
    let table = restaurant.get_table(table_number);
    let result = table.lock().unwrap().serialize(item_id);
    result
}

#[cfg(test)]
mod tests {
    use crate::items_handler::{add_item, get_all, get_item, delete_item};
    use crate::restaurant::Restaurant;

    fn create_restaurant(table_n: usize, item_n: usize) -> Restaurant {
        let r = Restaurant::new(table_n);
        let t = r.get_table(0);
        for i in 0..item_n {
            t.lock().unwrap().add_item(i as u32);
        }
        r
    }

    #[test]
    fn test_api_get_all() {
        let r = create_restaurant(1, 2);
        let output = get_all(0, r);
        assert!(output.contains("\"item_id\": 0"));
        assert!(output.contains("\"item_id\": 1"));
    }

    #[test]
    fn test_api_get_item() {
        let r = create_restaurant(1, 2);
        let r2 = r.clone();

        let output = get_item(0, 1, r);
        assert!(output.contains("\"item_id\": 1"));

        let output2 = get_item(0, 3, r2);
        assert!(output2.contains("{\"msg\": \"not found\"}"));
    }

    #[test]
    fn test_api_delete_item() {
        let item_amount = 5;
        let item_id = 1;

        let r = create_restaurant(1, item_amount);
        let r2 = r.clone();
        let r3 = r.clone();

        let output = delete_item(0, item_id, r);
        assert!(output.contains("success"));

        assert_eq!(
            r2.get_table(0).lock().unwrap().items_size(),
            item_amount - 1
        );

        let output2 = delete_item(0, item_id, r3);
        assert!(output2.contains("error deleting item"));
    }

    #[test]
    fn test_api_add_item() {
        let item_amount = 5;

        let r = create_restaurant(1, item_amount);

        add_item(0, "999,", r.clone());

        assert_eq!(
            r.clone().get_table(0).lock().unwrap().items_size(),
            item_amount + 1
        );

        add_item(0, "777", r.clone());

        assert_eq!(
            r.clone().get_table(0).lock().unwrap().items_size(),
            item_amount + 2
        );
    }
}

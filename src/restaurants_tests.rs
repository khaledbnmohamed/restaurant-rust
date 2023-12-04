#[cfg(test)]
mod tests {
    use std::thread;
    use crate::restaurant::Restaurant;

    #[test]
    fn test_restaurant_get_table() {
        let r = Restaurant::new(10);
        for test_id in 0..4 {
            let r2 = r.clone();
            thread::spawn(move || {
                let t = r2.get_table(test_id);
                let id = t.lock().unwrap().id();
                assert_eq!(id, test_id)
            });
        }
    }

    #[test]
    fn test_add_item() {
        let r = Restaurant::new(10);

        let desire_table_number = 0;

        let add_amount: usize = 1000;

        let handles: Vec<_> = (0..add_amount as u32)
            .map(|test_val| {
                let r2 = r.clone();
                thread::spawn(move || {
                    let t = r2.get_table(desire_table_number);
                    t.lock().unwrap().add_item(test_val);
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }

        let t = r.get_table(desire_table_number);
        let len = t.lock().unwrap().items_size();
        assert_eq!(len, add_amount);
    }
}

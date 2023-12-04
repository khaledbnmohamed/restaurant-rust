#[cfg(test)]
mod tests {
    use std::thread;
    use crate::handler::{parse_handler, parse_method, request_parser};
    use crate::model::{RequestHandler, RequestMethod};
    use crate::restaurant::Restaurant;

    #[test]
    fn test_parse_method() -> Result<(), String> {
        assert_eq!(parse_method("GET"), RequestMethod::Get);
        assert_eq!(parse_method("POST"), RequestMethod::Post);
        assert_eq!(parse_method("DELETE"), RequestMethod::Delete);
        assert_eq!(parse_method("PUT"), RequestMethod::Put);
        assert_eq!(parse_method("ABC"), RequestMethod::Unknown);
        Ok(())
    }

    #[test]
    fn test_parse_handler() -> Result<(), String> {
        assert_eq!(parse_handler("/add/xxx"), (RequestHandler::Add, vec!["xxx"]));
        assert_eq!(parse_handler("/get/xxx"), (RequestHandler::Get, vec!["xxx"]));
        assert_eq!(parse_handler("/delete/xxx"), (RequestHandler::Remove, vec!["xxx"]));
        assert_eq!(
            parse_handler("/add/xxx/yyy"),
            (RequestHandler::Add, vec!["xxx", "yyy"])
        );
        assert_eq!(
            parse_handler("/add/xxx/yyy/"),
            (RequestHandler::Add, vec!["xxx", "yyy", ""])
        );
        assert_eq!(parse_handler("add"), (RequestHandler::Unknown, vec![]));
        assert_eq!(parse_handler("add/xxx"), (RequestHandler::Unknown, vec![]));
        assert_eq!(parse_handler("/"), (RequestHandler::Unknown, vec![]));
        assert_eq!(parse_handler(""), (RequestHandler::Unknown, vec![]));
        Ok(())
    }

    fn get_restaurant_ready(desired_table_number: u32, add_amount: usize) -> Restaurant {
        let restaurant = Restaurant::new(200);

        let mut handles = vec![];

        for test_id in 0..add_amount {
            let restaurant = restaurant.clone();

            let req = format!("POST /add/{}/{}", desired_table_number, test_id);
            let mut bytes: Vec<u8> = req.into_bytes();

            let handle = thread::spawn(move || {
                let _res = request_parser(&mut bytes, restaurant.clone());
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        restaurant
    }

    #[test]
    fn integration_test_add_item() -> Result<(), String> {
        let desired_table_number = 0;
        let add_amount = 100;
        let restaurant = get_restaurant_ready(desired_table_number, add_amount);

        let t = restaurant.get_table(desired_table_number);
        let len = t.lock().unwrap().items_size();
        assert_eq!(len, add_amount);

        Ok(())
    }

    #[test]
    fn integration_test_delete_item() -> Result<(), String> {
        let desired_table_number = 0;
        let add_amount = 100;
        let delete_amount = 76;
        let restaurant = get_restaurant_ready(desired_table_number, add_amount);

        let mut handles = vec![];

        for test_id in 0..delete_amount {
            let restaurant = restaurant.clone();

            let req = format!("DELETE /delete/{}/{}", desired_table_number, test_id);
            let mut bytes: Vec<u8> = req.into_bytes();

            let handle = thread::spawn(move || {
                let _res = request_parser(&mut bytes, restaurant.clone());
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let t = restaurant.get_table(desired_table_number);
        let len = t.lock().unwrap().items_size();
        assert_eq!(len, add_amount - delete_amount);

        Ok(())
    }

    #[test]
    fn integration_test_check_item() -> Result<(), String> {
        let desired_table_number = 0;
        let add_amount = 20;
        let restaurant = get_restaurant_ready(desired_table_number, add_amount);

        let mut handles = vec![];

        for test_id in 0..add_amount {
            let restaurant = restaurant.clone();

            let req = format!("GET /get/{}/{}", desired_table_number, test_id);
            let mut bytes: Vec<u8> = req.into_bytes();

            let handle = thread::spawn(move || {
                let res = request_parser(&mut bytes, restaurant.clone());
                let s = format!("\"item_id\": {}", test_id);
                assert_eq!(res.contains(&s), true);
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        Ok(())
    }

    #[test]
    fn integration_test_check_all_item() -> Result<(), String> {
        let desired_table_number = 0;
        let add_amount = 20;
        let delete_amount = 17;
        let restaurant = get_restaurant_ready(desired_table_number, add_amount);

        let mut handles = vec![];

        for test_id in 0..delete_amount {
            let restaurant = restaurant.clone();

            let req = format!("DELETE /delete/{}/{}", desired_table_number, test_id);
            let mut bytes: Vec<u8> = req.into_bytes();

            let handle = thread::spawn(move || {
                let _res = request_parser(&mut bytes, restaurant.clone());
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        {
            let restaurant = restaurant.clone();

            let req = format!("GET /get/{}", desired_table_number);
            let mut bytes: Vec<u8> = req.into_bytes();

            let _ = thread::spawn(move || {
                let res = request_parser(&mut bytes, restaurant.clone());
                let s0 = "\"item_id\": 16";
                let s1 = "\"item_id\": 17";
                let s2 = "\"item_id\": 18";
                let s3 = "\"item_id\": 19";
                let s4 = "\"item_id\": 20";
                assert_eq!(res.contains(&s0), false);
                assert_eq!(res.contains(&s1), true);
                assert_eq!(res.contains(&s2), true);
                assert_eq!(res.contains(&s3), true);
                assert_eq!(res.contains(&s4), false);
            });
        }

        Ok(())
    }
}

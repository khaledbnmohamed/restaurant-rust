use crate::items_handler::{add_item, get_all, get_item, remove_item};
use crate::model::{RequestHandler, RequestMethod};
use super::restaurant::Restaurant;

pub fn parse_method(s: &str) -> RequestMethod {
    match s {
        "GET" => RequestMethod::Get,
        "POST" => RequestMethod::Post,
        "DELETE" => RequestMethod::Delete,
        "PUT" => RequestMethod::Put,
        _ => RequestMethod::Unknown,
    }
}

pub fn parse_handler(s: &str) -> (RequestHandler, Vec<&str>) {
    let handler_vec: Vec<&str> = s.split('/').collect();

    if handler_vec.len() < 2 {
        return (RequestHandler::Unknown, vec![]);
    }

    let handler_param = if handler_vec.len() > 2 {
        handler_vec[2..].to_vec()
    } else {
        vec![]
    };

    match handler_vec[1] {
        "add" => (RequestHandler::Add, handler_param),
        "remove" => (RequestHandler::Remove, handler_param),
        "get" => (RequestHandler::Get, handler_param),
        _ => (RequestHandler::Unknown, vec![]),
    }
}

pub fn request_parser(req: &mut [u8], restaurant: Restaurant) -> String {
    let req_str = std::str::from_utf8(req).unwrap();
    println!("Request: {}", req_str);

    let req_vec: Vec<&str> = req_str.split(' ').collect();

    if req_vec.len() < 2 {
        return "some error".to_string();
    }

    let method = parse_method(req_vec[0]);
    let (handler, handler_param) = parse_handler(req_vec[1]);

    match method {
        RequestMethod::Get => match handler {
            RequestHandler::Get => match handler_param.len() {
                1 => {
                    let tid: u32 = handler_param[0].parse().unwrap();
                    return get_all(tid, restaurant);
                }
                2 => {
                    let tid: u32 = handler_param[0].parse().unwrap();
                    let iid: u32 = handler_param[1].parse().unwrap();
                    return get_item(tid, iid, restaurant);
                }
                _ => return "wrong handler".to_string(),
            },
            _ => {}
        },
        RequestMethod::Post => match handler {
            RequestHandler::Add => match handler_param.len() {
                2 => {
                    let tid: u32 = handler_param[0].parse().unwrap();
                    let item_data: &str = handler_param[1];
                    return add_item(tid, item_data, restaurant);
                }
                _ => return "wrong handler".to_string(),
            },
            _ => {}
        },
        RequestMethod::Delete => match handler {
            RequestHandler::Remove => match handler_param.len() {
                2 => {
                    let tid: u32 = handler_param[0].parse().unwrap();
                    let iid: u32 = handler_param[1].parse().unwrap();
                    return remove_item(tid, iid, restaurant);
                }
                _ => return "wrong handler".to_string(),
            },
            _ => {}
        },
        RequestMethod::Put => {}
        _ => {
            return "unknown method".to_string();
        }
    }

    "unknown request".to_string()
}

use std::env;
use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

mod handler;
mod item;
mod restaurant;
mod model;
mod table;

mod main_tests;
mod restaurants_tests;
mod table_test;
mod item_tests;
mod items_handler_tests;
mod items_handler;

use restaurant::Restaurant;
use crate::handler::request_parser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let listener = TcpListener::bind(&addr).await?;
    println!("Server is now running 🚀🚀 at  {}", addr);

    let restaurant = Restaurant::new(200);

    loop {
        let (mut socket, _) = listener.accept().await?;

        let restaurant = restaurant.clone();

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];
            while let Ok(n) = socket.read(&mut buf).await {
                if n == 0 {
                    return;
                }

                let response = request_parser(&mut buf[0..n], restaurant.clone());

                if let Err(e) = socket.write_all(response.as_bytes()).await {
                    eprintln!("Failed to write data to socket: {}", e);
                }
            }
        });
    }
}

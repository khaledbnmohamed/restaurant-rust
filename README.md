# Item App

This is a simple API for managing items in a restaurant. It provides basic CRUD operations (Create, Read, Update, Delete) for items. Below are the details for each operation.

## Getting Started

### Prerequisites

- Rust installed: [Install Rust](https://www.rust-lang.org/learn/get-started)

### Running Locally

#### Using Cargo

To run the application locally using `cargo run`, follow these steps:

# Build and run the project with cargo
```bash
cargo run
```
# Test the project with cargo
```bash 
cargo test
```

# Test using simulation script
```bash
python3 simulation.py
```

# Endpoints are mocked as follows:

## Create Item

- **Method**: POST
- **Endpoint**: `http://Server/api/items/`
  - **Request Body**:

    ```json
    {
    "preparation_time": "5",
    "table_number": "5"
    }
    ```


## Get Item

- **Method**: GET
- **Endpoint**: `http://Server/api/items/{item_id}`
- **Replace `{item_id}` with the ID of the item you want to retrieve.

## Delete Item

- **Method**: DELETE
- **Endpoint**: `http://Server/api/items/{item_id}`
- **Replace `{item_id}` with the ID of the item you want to delete.

## Get All Items

- **Method**: GET
- **Endpoint**: `http://Server/api/items`



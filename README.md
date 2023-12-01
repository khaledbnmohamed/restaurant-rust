# Item App

This is a simple API for managing items in a restaurant. It provides basic CRUD operations (Create, Read, Update, Delete) for items. Below are the details for each operation.

## Getting Started

### Prerequisites

- Docker installed: [Get Docker](https://docs.docker.com/get-docker/)
- Rust installed: [Install Rust](https://www.rust-lang.org/learn/get-started)

### Running Locally

#### Using Cargo

To run the application locally using `cargo run`, follow these steps:

# Run docker compose in the background

```bash
docker-compose up -d 
```
# Run migrations using Make

```bash
 make migrate-up 
```

# Build and run the project with cargo
```bash
cargo run
```
## Create Item

- **Method**: POST
- **Endpoint**: `http://localhost:8000/api/items/`
  - **Request Body**:

    ```json
    {
    "item_name": "item 3",
    "preparation_time_minutes": "5",
    "table_number": "5"
    }
    ```

    Replace the values with your item details.

## Get Item

- **Method**: GET
- **Endpoint**: `http://localhost:8000/api/items/{item_id}`
- **Replace `{item_id}` with the ID of the item you want to retrieve.

## Delete Item

- **Method**: DELETE
- **Endpoint**: `http://localhost:8000/api/items/{item_id}`
- **Replace `{item_id}` with the ID of the item you want to delete.

## Get All Items

- **Method**: GET
- **Endpoint**: `http://localhost:8000/api/items`
- **Query Parameters**:
    - `page`: Page number for pagination (optional, default is 1)
    - `limit`: Number of items per page (optional, default is 10)
    - `table_number`: Filter items by table number (optional)

## Postman Collection

You can import the provided Postman collection to quickly test the API. [Download Postman Collection](/Users/khaled/RustroverProjects/paidy/Items.postman_collection.json)

**Note**: Make sure to replace the placeholder values in the request bodies and URLs with your actual data.

Feel free to reach out if you have any questions or issues!
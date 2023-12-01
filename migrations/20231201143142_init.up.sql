-- Add up migration script here

CREATE TABLE IF NOT EXISTS items (
    id CHAR(36) PRIMARY KEY NOT NULL,
    title VARCHAR(255) NOT NULL UNIQUE,
    table_number VARCHAR(100)  NOT NULL,
    preparation_time_minutes INT CHECK(preparation_time_minutes BETWEEN 5 AND 15) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
    );


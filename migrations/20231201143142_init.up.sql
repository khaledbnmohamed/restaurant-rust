-- Add up migration script here
CREATE TABLE tables (
                        id VARCHAR(36) PRIMARY KEY,
                        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

INSERT INTO tables (id) VALUES
                            ('1'),
                            ('2'),
                            ('3'),
                            ('4'),
                            ('5'),
                            ('6'),
                            ('7'),
                            ('8'),
                            ('9'),
                            ('10');
CREATE TABLE IF NOT EXISTS items (
    id CHAR(36) PRIMARY KEY NOT NULL,
    item_name VARCHAR(255) NOT NULL,
    table_number VARCHAR(100)  NOT NULL,
    preparation_time_minutes INT CHECK(preparation_time_minutes BETWEEN 5 AND 15) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (table_number) REFERENCES tables(id)

    );


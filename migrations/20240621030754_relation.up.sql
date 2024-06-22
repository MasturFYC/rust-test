-- Add up migration script here
CREATE TYPE relation_enum AS ENUM ('customer', 'employee', 'member', 'supplier');

-- CREATE SEQUENCE IF NOT EXISTS relation_id_seq AS INT
--     INCREMENT BY 1
--     START 1;

CREATE TABLE
    "relations" (
        id UUID NOT NULL DEFAULT (uuid_generate_v4()),
        name VARCHAR(50) NOT NULL,
        city VARCHAR(50) NOT NULL,
        street VARCHAR(255),
        phone VARCHAR(25),
        is_active BOOLEAN NOT NULL DEFAULT TRUE,
        is_special BOOLEAN NOT NULL DEFAULT FALSE,
        relation_type relation_enum[] NOT NULL DEFAULT '{customer}',
        created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
        updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
        PRIMARY KEY (id)
    );

CREATE INDEX idx_relation_name ON relations (name);

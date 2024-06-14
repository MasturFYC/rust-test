-- Add up migration script here

CREATE SEQUENCE IF NOT EXISTS product_id_seq AS INT
   INCREMENT BY 1
      START 1;

CREATE SEQUENCE IF NOT EXISTS category_id_seq AS SMALLINT
   INCREMENT BY 1
      START 1;

CREATE TABLE "categories" (
   id SMALLINT DEFAULT nextval('category_id_seq'::regclass),
   name VARCHAR(50) NOT NULL,
   PRIMARY KEY (id)
);

CREATE TABLE "products" (
   id INT DEFAULT nextval('product_id_seq'::regclass),
   name VARCHAR(50) NOT NULL,
   barcode VARCHAR(25) NOT NULL,
   unit VARCHAR(6) NOT NULL,
   unit_in_stock NUMERIC(12,2) NOT NULL DEFAULT 0,
   content NUMERIC(9,2) NOT NULL DEFAULT 1,
   hpp NUMERIC(9,2) NOT NULL DEFAULT 0,
   margin NUMERIC(5,2) NOT NULL DEFAULT 0,
   price NUMERIC(9,2) NOT NULL DEFAULT 0,
   ppn NUMERIC(5,2) NOT NULL DEFAULT 0,
   is_active BOOLEAN NOT NULL DEFAULT TRUE,
   variant_name VARCHAR(50),
   descriptions VARCHAR(128),
   category_id SMALLINT NOT NULL DEFAULT 0,
   created_at TIMESTAMP
      WITH
      TIME ZONE DEFAULT NOW(),
   updated_at TIMESTAMP
      WITH
      TIME ZONE DEFAULT NOW(),
   PRIMARY KEY (id)
);

CREATE UNIQUE INDEX "ixq_category_name" ON categories (name);
CREATE INDEX "ix_product_name" ON products (name);
CREATE UNIQUE INDEX "ixq_product_barcode" ON products (barcode);

ALTER TABLE "products" 
   ADD CONSTRAINT fk_product_category FOREIGN KEY (category_id)
      REFERENCES categories (id);
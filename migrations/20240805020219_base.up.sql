-- Add up migration script here

CREATE TYPE order_enum AS ENUM ('stock', 'order', 'retail', 'order_return', 'stock_return', 'mutation');
-- CREATE TYPE payment_enum AS ENUM ('cash', 'pending', 'loans', 'lunas');
CREATE TYPE transaction_enum AS ENUM ('order', 'stock', 'order_return', 'stock_return', 'loan', 'order_payment', 'stock_payment', 'loan_payment');
CREATE TYPE relation_enum AS ENUM ('customer', 'employee', 'supplier', 'sales');

CREATE TABLE "accounts" (
   id SMALLINT NOT NULL,
   name VARCHAR(50) NOT NULL,
   root SMALLINT,
   normal SMALLINT NOT NULL DEFAULT 0,
   en_name VARCHAR(50),
   is_active boolean NOT NULL DEFAULT TRUE,
   payable boolean NOT NULL DEFAULT FALSE,
   descriptions VARCHAR(128),
   PRIMARY KEY (id)
);

CREATE UNIQUE INDEX "ixq_account_name" ON accounts (name);

ALTER TABLE "accounts"
   ADD CONSTRAINT fk_account_root FOREIGN KEY (root)
      REFERENCES accounts (id);

INSERT INTO accounts (id, name) VALUES (0, 'COA');

CREATE SEQUENCE IF NOT EXISTS category_id_seq AS SMALLINT
   INCREMENT BY 1
      START 1;

CREATE TABLE "categories" (
   id SMALLINT NOT NULL DEFAULT nextval('category_id_seq'::regclass),
   name VARCHAR(50) NOT NULL,
   PRIMARY KEY (id)
);

CREATE SEQUENCE IF NOT EXISTS region_id_seq AS SMALLINT
   INCREMENT BY 1
      START 2;

CREATE TABLE regions (
       id SMALLINT NOT NULL DEFAULT nextval('region_id_seq'::regclass),
       name VARCHAR(50) NOT NULL,
       PRIMARY KEY(id)
);

CREATE INDEX idx_region_name ON regions (name);


CREATE SEQUENCE IF NOT EXISTS relation_id_seq AS SMALLINT
   INCREMENT BY 1
      START 2;

CREATE TABLE
    "relations" (
        id SMALLINT NOT NULL DEFAULT nextval('relation_id_seq'::regclass),
        name VARCHAR(50) NOT NULL,
        city VARCHAR(50) NOT NULL,
        street VARCHAR(255),
        phone VARCHAR(25),
        region_id SMALLINT NOT NULL DEFAULT 1,
        is_active BOOLEAN NOT NULL DEFAULT TRUE,
        is_special BOOLEAN NOT NULL DEFAULT FALSE,
        relation_type relation_enum[] NOT NULL DEFAULT '{customer}',
        photo VARCHAR(256),
        PRIMARY KEY (id)
    );

CREATE INDEX idx_relation_name ON relations (name);

CREATE SEQUENCE IF NOT EXISTS product_id_seq AS SMALLINT
   INCREMENT BY 1
      START 1;

CREATE SEQUENCE "warehouse_id_seq" AS SMALLINT INCREMENT BY 1 START 1;

CREATE TABLE "warehouses" (
    id SMALLINT NOT NULL DEFAULT nextval('warehouse_id_seq'::regclass),
    name VARCHAR (50) NOT NULL,
    employee_id SMALLINT NOT NULL,
    locate VARCHAR(128),
    PRIMARY KEY (id)
);

CREATE TABLE "products" (
   id SMALLINT NOT NULL DEFAULT nextval('product_id_seq'::regclass),
   category_id SMALLINT NOT NULL,
   name VARCHAR(50) NOT NULL,
   barcode VARCHAR(25) NOT NULL,
   unit VARCHAR(6) NOT NULL,
   content NUMERIC(9,2) NOT NULL DEFAULT 1,
   hpp NUMERIC(12,2) NOT NULL DEFAULT 0,
   margin NUMERIC(7,4) NOT NULL DEFAULT 0,
   price NUMERIC(12,2) NOT NULL DEFAULT 0,
   heavy NUMERIC(12,2) NOT NULL DEFAULT 0,
   is_active BOOLEAN NOT NULL DEFAULT TRUE,
   variant_name VARCHAR(50),
   descriptions VARCHAR(128),
   PRIMARY KEY (id)
);

CREATE UNIQUE INDEX "ixq_category_name" ON categories (name);
CREATE INDEX "ix_product_name" ON products (name);
CREATE UNIQUE INDEX "ixq_product_barcode" ON products (barcode);

ALTER TABLE "products"
   ADD CONSTRAINT fk_product_category FOREIGN KEY (category_id)
      REFERENCES categories (id);

CREATE TABLE
    "orders" (
    -- transacion_id
        order_id INT NOT NULL,
    -- for purchase
	order_type order_enum NOT NULL,
        customer_id SMALLINT NOT NULL,
        sales_id SMALLINT NOT NULL,
        total NUMERIC(12,2) NOT NULL DEFAULT 0,
        dp NUMERIC(12,2) NOT NULL DEFAULT 0,
        remain NUMERIC(12,2) NOT NULL DEFAULT 0,
        is_protected BOOLEAN NOT NULL DEFAULT TRUE,
        due_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
	tag VARCHAR(50) NOT NULL,
	descriptions VARCHAR(128),
        invoice_number varchar(50),
        PRIMARY KEY (order_id)
    );

CREATE INDEX ix_order_relation
   ON "orders" (customer_id, sales_id, order_type);

ALTER TABLE "orders"
   ADD CONSTRAINT fk_order_customer FOREIGN KEY (customer_id)
      REFERENCES relations (id);

ALTER TABLE "orders"
   ADD CONSTRAINT fk_order_sales FOREIGN KEY (sales_id)
      REFERENCES relations (id);

CREATE TABLE
   "order_details" (
      order_id INT NOT NULL,
      detail_id SMALLINT NOT NULL,
      product_id SMALLINT NOT NULL,
      warehouse_id SMALLINT NOT NULL DEFAULT 1,
      qty NUMERIC(9,2) NOT NULL DEFAULT 1,
      direction SMALLINT NOT NULL DEFAULT 0,
      unit VARCHAR(6) NOT NULL,
      hpp NUMERIC(12,2) NOT NULL DEFAULT 0,
      price NUMERIC(12,2) NOT NULL DEFAULT 0,
      discount NUMERIC(9,2) NOT NULL DEFAULT 0,
      subtotal NUMERIC(12,2) NOT NULL DEFAULT 0,
      updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
      PRIMARY KEY (order_id, detail_id)
   );

CREATE INDEX ix_order_detail_updated
   ON "order_details" (updated_at DESC, product_id);

CREATE INDEX ix_detail_warehouse
    ON "order_details" (warehouse_id);

ALTER TABLE "order_details"
   ADD CONSTRAINT fk_detail_order FOREIGN KEY (order_id)
      REFERENCES "orders" (order_id);

ALTER TABLE "order_details"
   ADD CONSTRAINT fk_detail_product FOREIGN KEY (product_id)
      REFERENCES "products" (id);

ALTER TABLE "order_details"
    ADD CONSTRAINT fk_warehouse_order FOREIGN KEY (warehouse_id)
    REFERENCES "warehouses" (id);

CREATE SEQUENCE IF NOT EXISTS transaction_id_seq AS INT
   INCREMENT BY 1
      START 1;

CREATE TABLE "transactions" (
   id INT NOT NULL DEFAULT nextval('transaction_id_seq'::regclass),
   transaction_type transaction_enum NOT NULL,
   is_valid BOOL NOT NULL DEFAULT TRUE,
   created_by VARCHAR(50) NOT NULL,
   updated_by VARCHAR(50) NOT NULL,
   descriptions VARCHAR(128),
   memo VARCHAR(128),
   created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
   updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
   PRIMARY KEY (id)
);

CREATE TABLE "transaction_details" (
   transaction_id INT NOT NULL,
   detail_id SMALLINT NOT NULL,
   account_id SMALLINT NOT NULL,
   amount NUMERIC(12,2) NOT NULL DEFAULT 0,
   direction SMALLINT NOT NULL,
   ref_id INT NOT NULL DEFAULT 0,
   descriptions VARCHAR(128),
   PRIMARY KEY (transaction_id, detail_id)
);

CREATE INDEX ix_transaction_detail_account ON transaction_details(account_id);
CREATE INDEX ix_transaction_detail_ref ON transaction_details(ref_id);

ALTER TABLE transaction_details ADD CONSTRAINT fx_transaction_detail
    FOREIGN KEY(transaction_id) REFERENCES transactions(id) ON DELETE CASCADE;
ALTER TABLE transaction_details ADD CONSTRAINT fx_transaction_detail_account
    FOREIGN KEY(account_id) REFERENCES accounts(id);

-- Add up migration script here

CREATE TABLE "stocks" (
    warehouse_id SMALLINT NOT NULL,
    product_id SMALLINT NOT NULL,
    qty NUMERIC(12,2) NOT NULL DEFAULT 0,
    PRIMARY KEY (warehouse_id, product_id)
);

CREATE UNIQUE INDEX "ixq_warehouse_name" ON warehouses (name);

CREATE INDEX "ix_warehouse_employee" ON warehouses (employee_id);

ALTER TABLE "warehouses"
   ADD CONSTRAINT "fk_warehouse_employee" FOREIGN KEY (employee_id)
      REFERENCES "relations" (id);

ALTER TABLE "stocks"
   ADD CONSTRAINT "fk_stock_warehouse" FOREIGN KEY (warehouse_id)
      REFERENCES "warehouses" (id) ON DELETE CASCADE;

ALTER TABLE "stocks"
    ADD CONSTRAINT fk_stock_product FOREIGN KEY (product_id)
        REFERENCES "products" (id) ON DELETE CASCADE;

-- ALTER TABLE "order_details"
   -- ADD CONSTRAINT fk_order_warehouse FOREIGN KEY (warehouse_id)
     --   REFERENCES "warehouses" (id) ON DELETE CASCADE;

ALTER TABLE "relations"
   ADD CONSTRAINT fk_region_relation FOREIGN KEY (region_id)
      REFERENCES regions (id);

INSERT INTO regions (id, name)
    VALUES (1, 'Non Customer');

INSERT INTO relations (
        id,
        name,
        city,
        street,
        phone,
        region_id,
        is_active,
        is_special,
        relation_type,
        photo
    ) VALUES (
    1,
    'Penjaga Gudang Toko',
    'Local',
    '-',
    '0000-0000-0000',
    1,
    true,
    false,
    '{employee}',
    'default.png'
);

INSERT INTO warehouses (id, employee_id, name)
    VALUES (1, 1, 'Gudang Toko');

INSERT INTO accounts (id, name, normal, en_name)
    VALUES (101, 'Kas', 1, 'Cash');
INSERT INTO accounts (id, name, normal, en_name)
    VALUES (106, 'Persediaan barang', 1, 'Inventory');
INSERT INTO accounts (id, name, normal, en_name)
    VALUES (111, 'Piutang penjualan', 1, 'Loan');
INSERT INTO accounts (id, name, normal, en_name)
    VALUES (204, 'Utang dagang', -1, 'Account payable');
INSERT INTO accounts (id, name, normal, en_name)
    VALUES (421, 'Penjualan barang', -1,  'Revenue');
INSERT INTO accounts (id, name, normal, en_name)
    VALUES (521, 'Biaya beli barang', 1,  'Cost of goods sold');

-- Add up migration script here

CREATE TYPE order_enum AS ENUM ('order', 'stock', 'order_return', 'stock_return', 'mutation');
CREATE TYPE payment_enum AS ENUM ('cash', 'pending', 'loans', 'lunas');
CREATE TYPE ledger_enum AS ENUM ('order', 'stock', 'order_return', 'stock_return', 'loan', 'order_payment', 'stock_payment', 'loan_payment');
CREATE TYPE relation_enum AS ENUM ('customer', 'employee', 'supplier', 'Sales');

CREATE TABLE "accounts" (
   id SMALLINT NOT NULL,
   name VARCHAR(50) NOT NULL,
   root SMALLINT,
   normal SMALLINT NOT NULL DEFAULT 0,
   en_name VARCHAR(50),
   is_active boolean NOT NULL DEFAULT TRUE,
   payable boolean NOT NULL DEFAULT FALSE,
   descriptions VARCHAR(128),
   created_at TIMESTAMP
      WITH
      TIME ZONE DEFAULT NOW(),
   updated_at TIMESTAMP
      WITH
      TIME ZONE DEFAULT NOW(),
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

CREATE SEQUENCE IF NOT EXISTS gudang_seq AS SMALLINT
   INCREMENT BY 1
      START 1;

CREATE TABLE "categories" (
   id SMALLINT NOT NULL DEFAULT nextval('category_id_seq'::regclass),
   name VARCHAR(50) NOT NULL,
   PRIMARY KEY (id)
);

CREATE SEQUENCE IF NOT EXISTS relation_id_seq AS SMALLINT
   INCREMENT BY 1
      START 1;

CREATE TABLE
    "relations" (
        id SMALLINT NOT NULL DEFAULT nextval('relation_id_seq'::regclass),
        name VARCHAR(50) NOT NULL,
        city VARCHAR(50) NOT NULL,
        street VARCHAR(255),
        phone VARCHAR(25),
        region VARCHAR(50),
        is_active BOOLEAN NOT NULL DEFAULT TRUE,
        is_special BOOLEAN NOT NULL DEFAULT FALSE,
        relation_type relation_enum[] NOT NULL DEFAULT '{customer}',
        photo VARCHAR(256),
        created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
        updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
        PRIMARY KEY (id)
    );

CREATE INDEX idx_relation_name ON relations (name);

CREATE SEQUENCE IF NOT EXISTS product_id_seq AS SMALLINT
   INCREMENT BY 1
      START 1;

CREATE SEQUENCE "gudang_id_seq" AS SMALLINT INCREMENT BY 1 START 1;

CREATE TABLE "gudangs" (
    id SMALLINT NOT NULL DEFAULT nextval('gudang_id_seq'::regclass),
    name VARCHAR (50) NOT NULL,
    employee_id SMALLINT NOT NULL,
    locate VARCHAR(128),
    PRIMARY KEY (id)
);

CREATE TABLE "products" (
   id SMALLINT NOT NULL DEFAULT nextval('product_id_seq'::regclass),
   supplier_id SMALLINT NOT NULL,
   category_id SMALLINT NOT NULL,
   name VARCHAR(50) NOT NULL,
   barcode VARCHAR(25) NOT NULL,
   unit VARCHAR(6) NOT NULL,
   content NUMERIC(9,2) NOT NULL DEFAULT 1,
   hpp NUMERIC(9,2) NOT NULL DEFAULT 0,
   margin NUMERIC(7,4) NOT NULL DEFAULT 0,
   price NUMERIC(9,2) NOT NULL DEFAULT 0,
   ppn NUMERIC(7,4) NOT NULL DEFAULT 0,
   heavy REAL NOT NULL DEFAULT 0,
   is_active BOOLEAN NOT NULL DEFAULT TRUE,
   variant_name VARCHAR(50),
   descriptions VARCHAR(128),
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
CREATE INDEX "ix_product_supplier" ON products (supplier_id);

ALTER TABLE "products"
   ADD CONSTRAINT fk_product_category FOREIGN KEY (category_id)
      REFERENCES categories (id);

ALTER TABLE "products"
   ADD CONSTRAINT fk_product_supplier FOREIGN KEY (supplier_id)
      REFERENCES relations (id);

CREATE SEQUENCE IF NOT EXISTS order_id_seq AS INT
   INCREMENT BY 1
      START 1;

CREATE TABLE
    "orders" (
        id INT NOT NULL DEFAULT nextval('order_id_seq'::regclass),
        order_type order_enum NOT NULL,
        customer_id SMALLINT NOT NULL,
        sales_id SMALLINT NOT NULL,
        payment_type payment_enum NOT NULL,
        updated_by VARCHAR(50) NOT NULL,
        total NUMERIC(12,2) NOT NULL DEFAULT 0,
        dp NUMERIC(12,2) NOT NULL DEFAULT 0,
        payment NUMERIC(12,2) NOT NULL DEFAULT 0,
        remain NUMERIC(12,2) NOT NULL DEFAULT 0,
        -- Nomor faktur stock
        -- pada saat pembelian barang
        invoice_id VARCHAR(50),
        due_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
        is_protected BOOLEAN NOT NULL DEFAULT TRUE,
        created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
        updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
        PRIMARY KEY (id)
    );

CREATE INDEX ix_order_relation
   ON "orders" (created_at, customer_id, sales_id, order_type, payment_type);

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
      gudang_id SMALLINT NOT NULL DEFAULT 1,
      qty NUMERIC(9,2) NOT NULL DEFAULT 1,
      direction SMALLINT NOT NULL DEFAULT 0,
      unit VARCHAR(6) NOT NULL,
      hpp NUMERIC(12,2) NOT NULL DEFAULT 0,
      price NUMERIC(12,2) NOT NULL DEFAULT 0,
      discount NUMERIC(9,2) NOT NULL DEFAULT 0,
      subtotal NUMERIC(12,2) NOT NULL DEFAULT 0,
      created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
      updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
      PRIMARY KEY (order_id, detail_id)
   );
CREATE INDEX ix_order_detail
   ON "order_details" (order_id, created_at);
CREATE INDEX ix_order_detail_updated
   ON "order_details" (product_id, updated_at DESC);
CREATE INDEX ix_detail_gudang
    ON "order_details" (gudang_id);

ALTER TABLE "order_details"
   ADD CONSTRAINT fk_detail_order FOREIGN KEY (order_id)
      REFERENCES "orders" (id);

ALTER TABLE "order_details"
   ADD CONSTRAINT fk_detail_product FOREIGN KEY (product_id)
      REFERENCES "products" (id);

ALTER TABLE "order_details"
    ADD CONSTRAINT fk_gudang_order FOREIGN KEY (gudang_id)
    REFERENCES "gudangs" (id);

CREATE SEQUENCE IF NOT EXISTS ledger_id_seq AS INT
   INCREMENT BY 1
      START 1;

CREATE TABLE "ledgers" (
   id INT NOT NULL DEFAULT nextval('ledger_id_seq'::regclass),
   relation_id SMALLINT NOT NULL,
   ledger_type ledger_enum NOT NULL,
   is_valid BOOL NOT NULL DEFAULT TRUE,
   updated_by VARCHAR(50) NOT NULL,
   descriptions VARCHAR(128),
   created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
   updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
   PRIMARY KEY (id)
);

CREATE INDEX ix_ledger_relation ON ledgers (relation_id);

CREATE TABLE "ledger_details" (
   ledger_id INT NOT NULL,
   detail_id SMALLINT NOT NULL,
   account_id SMALLINT NOT NULL,
   amount NUMERIC(12,2) NOT NULL DEFAULT 0,
   direction SMALLINT NOT NULL,
   ref_id INT NOT NULL DEFAULT 0,
   descriptions VARCHAR(128),
   PRIMARY KEY (ledger_id, detail_id)
);

CREATE INDEX ix_ledger_detail_account ON ledger_details(account_id);
CREATE INDEX ix_ledger_detail_ref ON ledger_details(ref_id);

ALTER TABLE ledgers ADD CONSTRAINT fx_ledger_relation
    FOREIGN KEY(relation_id) REFERENCES relations(id);
ALTER TABLE ledger_details ADD CONSTRAINT fx_ledger_detail
    FOREIGN KEY(ledger_id) REFERENCES ledgers(id) ON DELETE CASCADE;
ALTER TABLE ledger_details ADD CONSTRAINT fx_ledger_detail_account
    FOREIGN KEY(account_id) REFERENCES accounts(id);

CREATE TABLE
    "order_payments" (
        order_id INT NOT NULL,
        payment_id SMALLINT NOT NULL,
        amount NUMERIC(12,0) NOT NULL DEFAULT 0,
        updated_by VARCHAR(50) NOT NULL,
        via_by VARCHAR(50) NOT NULL,
        descriptions VARCHAR(50),
        created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
        updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
        PRIMARY KEY (order_id, payment_id)
    );

CREATE INDEX ix_order_payment
   ON order_payments(order_id, payment_id);

ALTER TABLE "order_payments"
    ADD CONSTRAINT fk_order_payment FOREIGN KEY (order_id)
        REFERENCES orders (id) ON DELETE CASCADE;

-- Add up migration script here
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

CREATE TABLE "stocks" (
    gudang_id SMALLINT NOT NULL,
    product_id SMALLINT NOT NULL,
    qty NUMERIC(12,2) NOT NULL DEFAULT 0,
    PRIMARY KEY (gudang_id, product_id)
);

CREATE UNIQUE INDEX "ixq_gudang_name" ON gudangs (name);

CREATE INDEX "ix_gudang_employee" ON gudangs (employee_id);

ALTER TABLE "gudangs"
   ADD CONSTRAINT "fk_gudang_employee" FOREIGN KEY (employee_id)
      REFERENCES "relations" (id);

ALTER TABLE "stocks"
   ADD CONSTRAINT "fk_stock_gudang" FOREIGN KEY (gudang_id)
      REFERENCES "gudangs" (id) ON DELETE CASCADE;

ALTER TABLE "stocks"
    ADD CONSTRAINT fk_stock_product FOREIGN KEY (product_id)
        REFERENCES "products" (id) ON DELETE CASCADE;

-- ALTER TABLE "order_details"
   -- ADD CONSTRAINT fk_order_gudang FOREIGN KEY (gudang_id)
     --   REFERENCES "gudangs" (id) ON DELETE CASCADE;
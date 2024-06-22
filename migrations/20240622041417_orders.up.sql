-- Add up migration script here

CREATE TYPE order_enum AS ENUM ('order', 'stock', 'orderreturn', 'stockreturn', 'mutation');
CREATE TYPE payment_enum AS ENUM ('cash', 'pending', 'loans', 'lunas');

CREATE TABLE 
    "orders" (
        id UUID NOT NULL DEFAULT (uuid_generate_v4()),
        order_type order_enum NOT NULL,
        relation_id UUID NOT NULL,
        payment_type payment_enum NOT NULL,
        updated_by VARCHAR(50) NOT NULL,
        total NUMERIC(12,2) NOT NULL DEFAULT 0,
        payment NUMERIC(12,2) NOT NULL DEFAULT 0,
        remain NUMERIC(12,2) NOT NULL DEFAULT 0,
        -- Nomor faktur stock
        -- pada saat pembelian barang
        invoice_id VARCHAR(50),
        due_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
        created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
        updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
        PRIMARY KEY (id)        
    );

CREATE INDEX idx_order_relation ON "orders" (created_at, relation_id, order_type, payment_type);

ALTER TABLE "orders"
   ADD CONSTRAINT fk_order_relation FOREIGN KEY (relation_id)
      REFERENCES relations (id);
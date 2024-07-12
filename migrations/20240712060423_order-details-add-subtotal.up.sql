-- Add up migration script here

ALTER TABLE orders
    ADD COLUMN dp NUMERIC(12,2) NOT NULL DEFAULT 0;

ALTER TABLE order_details
    ADD COLUMN subtotal NUMERIC(12,2) NOT NULL DEFAULT 0;

CREATE TABLE 
    "order_payments" (
        id UUID NOT NULL DEFAULT (uuid_generate_v4()),
        order_id UUID NOT NULL,
        amount NUMERIC(12,0) NOT NULL DEFAULT 0,
        created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
        updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
        updated_by VARCHAR(50) NOT NULL,
        descriptions VARCHAR(50),
        PRIMARY KEY (id)
    );

CREATE INDEX ix_order_payment ON order_payments(order_id, id);

ALTER TABLE "order_payments"
    ADD CONSTRAINT fk_order_payment FOREIGN KEY (order_id)
        REFERENCES orders (id) ON DELETE CASCADE;


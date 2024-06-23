-- Add up migration script here
CREATE TABLE
   "order_details" (
      order_id UUID NOT NULL,
      id UUID NOT NULL DEFAULT (uuid_generate_v4()),
      product_id INT NOT NULL,
      qty NUMERIC(9,2) NOT NULL DEFAULT 1,
      direction SMALLINT NOT NULL DEFAULT 0,
      unit VARCHAR(6) NOT NULL,
      price NUMERIC(12,2) NOT NULL DEFAULT 0,
      discount NUMERIC(9,2) NOT NULL DEFAULT 0,
      hpp NUMERIC(12,2) NOT NULL DEFAULT 0,
      created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
      updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),      
      PRIMARY KEY (id)
   );
CREATE INDEX ix_order_detail ON "order_details" (order_id, created_at);
CREATE INDEX ix_order_detail_updated ON "order_details" (product_id, updated_at DESC);

ALTER TABLE "order_details"
   ADD CONSTRAINT fk_detail_order FOREIGN KEY (order_id)
      REFERENCES "orders" (id);

ALTER TABLE "order_details"
   ADD CONSTRAINT fk_detail_product FOREIGN KEY (product_id)
      REFERENCES "products" (id);      
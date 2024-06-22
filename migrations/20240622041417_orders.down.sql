-- Add down migration script here
DROP INDEX idx_order_relation;
ALTER TABLE "orders"
    DROP CONSTRAINT fk_order_relation;
-- DROP TABLE "order_details";
DROP TABLE "orders";
DROP TYPE order_enum;
DROP TYPE payment_enum;

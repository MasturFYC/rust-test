-- Add down migration script here
DROP INDEX ix_order_detail_updated;
DROP INDEX ix_order_detail;

ALTER TABLE "order_details"
    DROP CONSTRAINT fk_detail_order;

ALTER TABLE "order_details"
    DROP CONSTRAINT fk_detail_product;

DROP TABLE "order_details";
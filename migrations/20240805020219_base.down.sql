-- Add down migration script here
ALTER TABLE orders DROP CONSTRAINT fk_order_customer;
ALTER TABLE orders DROP CONSTRAINT fk_order_sales;
ALTER TABLE order_details DROP CONSTRAINT fk_detail_order;
ALTER TABLE order_details  DROP CONSTRAINT fk_detail_product;
ALTER TABLE ledgers DROP CONSTRAINT fx_ledger_relation;
ALTER TABLE ledger_details DROP CONSTRAINT fx_ledger_detail;
ALTER TABLE ledger_details DROP CONSTRAINT fx_ledger_detail_account;
ALTER TABLE order_payments DROP CONSTRAINT fk_order_payment;
ALTER TABLE products DROP CONSTRAINT fk_product_supplier;
ALTER TABLE products DROP CONSTRAINT fk_product_category;
ALTER TABLE gudangs DROP CONSTRAINT fk_gudang_employee;
ALTER TABLE stocks DROP CONSTRAINT fk_stock_gudang;
ALTER TABLE stocks DROP CONSTRAINT fk_stock_product;

DROP INDEX ixq_gudang_name;
DROP INDEX ix_gudang_employee;
DROP INDEX ix_order_payment;
DROP INDEX ix_order_relation;
DROP INDEX ix_ledger_relation;
DROP INDEX ix_ledger_detail_account;
DROP INDEX ix_ledger_detail_ref;
DROP INDEX ix_order_detail_updated;
DROP INDEX ix_order_detail;
DROP INDEX ix_product_supplier;

DROP TABLE IF EXISTS "gudangs";
DROP TABLE IF EXISTS "stocks";
DROP TABLE IF EXISTS "order_payments";
DROP TABLE IF EXISTS "order_details";
DROP TABLE IF EXISTS "orders";
DROP TABLE IF EXISTS "ledger_details";
DROP TABLE IF EXISTS "ledgers";
DROP TABLE IF EXISTS "products" CASCADE;
DROP TABLE IF EXISTS "categories" CASCADE;
DROP TABLE IF EXISTS "relations" CASCADE;
DROP TABLE IF EXISTS "accounts" CASCADE;

DROP TYPE IF EXISTS order_enum;
DROP TYPE IF EXISTS payment_enum;
DROP TYPE IF EXISTS ledger_enum;
DROP type IF EXISTS relation_enum;

DROP SEQUENCE IF EXISTS "category_id_seq";
DROP SEQUENCE IF EXISTS "product_id_seq";
DROP SEQUENCE IF EXISTS "relation_id_seq";
DROP SEQUENCE IF EXISTS "order_id_seq";
DROP SEQUENCE IF EXISTS "ledger_id_seq";
DROP SEQUENCE IF EXISTS "gudang_id_seq" CASCADE;
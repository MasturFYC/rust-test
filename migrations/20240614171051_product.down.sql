-- Add down migration script here

DROP SEQUENCE IF EXISTS "category_id_seq" CASCADE;
DROP SEQUENCE IF EXISTS "product_id_seq" CASCADE;
DROP TABLE IF EXISTS "categories" CASCADE;
DROP TABLE IF EXISTS "products" CASCADE;
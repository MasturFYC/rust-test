-- Add down migration script here
-- DROP SEQUENCE IF EXISTS "relation_id_seq" CASCADE;
DROP TABLE IF EXISTS "relations" CASCADE;
DROP TYPE relation_enum;

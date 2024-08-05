-- Add down migration script here
ALTER TABLE relations
    DROP COLUMN region;
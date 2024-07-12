-- Add down migration script here

DROP INDEX ix_order_payment;

ALTER TABLE order_payments
    DROP CONSTRAINT fk_order_payments;

DROP TABLE order_payments;

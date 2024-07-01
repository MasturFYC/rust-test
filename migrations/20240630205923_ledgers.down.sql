-- Add down migration script here

DROP INDEX ix_ledger_relation;
DROP INDEX ix_ledger_detail;
DROP INDEX ix_ledger_ref;

ALTER TABLE ledgers DROP CONSTRAINT fx_ledger_relation;
ALTER TABLE ledger_details DROP CONSTRAINT fx_ledger_detail;
ALTER TABLE ledger_details DROP CONSTRAINT fx_ledger_detail_account;
 
DROP TABLE "ledgers";
DROP TABLE "ledger_details";

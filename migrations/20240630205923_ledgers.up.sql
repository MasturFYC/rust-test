-- Add up migration script here
CREATE TABLE "ledgers" (
   id UUID NOT NULL DEFAULT (uuid_generate_v4()),
   relation_id UUID NOT NULL,
   descriptions VARCHAR(128),
   name VARCHAR(50) NOT NULL,
   is_valid BOOL NOT NULL DEFAULT TRUE,
   updated_by VARCHAR(50) NOT NULL,
   created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
   updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
   PRIMARY KEY (id)
);

CREATE TABLE "ledger_details" (
   ledger_id UUID NOT NULL,
   id SMALLINT NOT NULL,
   account_id SMALLINT NOT NULL,
   descriptions VARCHAR(128),
   amount NUMERIC(9,2) NOT NULL DEFAULT 0,
   direction SMALLINT NOT NULL,
   ref_id UUID,
   PRIMARY KEY (ledger_id, id)
);

CREATE INDEX ix_ledger_relation ON ledgers (id);
CREATE INDEX ix_ledger_detail ON ledger_details(account_id);
CREATE INDEX ix_ledger_ref ON ledger_details(ref_id);

ALTER TABLE ledgers ADD CONSTRAINT fx_ledger_relation
    FOREIGN KEY(relation_id) REFERENCES relations(id);
ALTER TABLE ledger_details ADD CONSTRAINT fx_ledger_detail
    FOREIGN KEY(ledger_id) REFERENCES ledgers(id) ON DELETE CASCADE;
ALTER TABLE ledger_details ADD CONSTRAINT fx_ledger_detail_account
    FOREIGN KEY(account_id) REFERENCES accounts(id);


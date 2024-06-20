-- Add up migration script here

CREATE TABLE "accounts" (
   id SMALLINT NOT NULL,
   name VARCHAR(50) NOT NULL,   
   root SMALLINT,
   normal SMALLINT NOT NULL DEFAULT 0,
   en_name VARCHAR(50),
   descriptions VARCHAR(128),
   is_active boolean NOT NULL DEFAULT TRUE,
   payable boolean NOT NULL DEFAULT FALSE,
   created_at TIMESTAMP
      WITH
      TIME ZONE DEFAULT NOW(),
   updated_at TIMESTAMP
      WITH
      TIME ZONE DEFAULT NOW(),
    PRIMARY KEY (id)
);

CREATE UNIQUE INDEX "ixq_account_name" ON accounts (name);

ALTER TABLE "accounts" 
   ADD CONSTRAINT fk_account_root FOREIGN KEY (root)
      REFERENCES accounts (id);

INSERT INTO accounts (id, name) VALUES (0, 'COA');
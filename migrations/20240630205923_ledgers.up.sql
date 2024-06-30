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

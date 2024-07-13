INSERT INTO ledgers (
    relation_id,
    ledger_type,
    updated_by,
    is_valid,
    descriptions
) VALUES ($1, $2, $3, $4, $5)
RETURNING id, created_at, updated_at

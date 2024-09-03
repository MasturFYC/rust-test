UPDATE
    ledgers
SET
    relation_id = $2,
    ledger_type = $3,
    updated_by = $4,
    is_valid = $5,
    descriptions = $6,
    updated_at = now()
WHERE
    id = $1
RETURNING id, created_at, updated_at
SELECT
    id,
    relation_id,
    ledger_type as "ledger_type: LedgerType",
    descriptions,
    updated_by,
    is_valid,
    created_at,
    updated_at
FROM
    ledgers
ORDER BY created_at
LIMIT $1
OFFSET $2
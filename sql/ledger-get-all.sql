SELECT
    id, 
    relation_id,
    name,
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

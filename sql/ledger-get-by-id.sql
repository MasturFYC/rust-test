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
WHERE
    id = $1

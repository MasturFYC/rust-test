UPDATE ledgers SET
    relation_id = $2,
    name = $3,
    descriptions = $4,
    updated_by = $5,
    is_valid = $6,
    updated_at = now()
WHERE
    id = $1
RETURNING *

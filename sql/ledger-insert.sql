INSERT INTO ledgers (
    relation_id,
    name,
    descriptions,
    updated_by,
    is_valid
) VALUES ($1, $2, $3, $4, $5)
RETURNING *

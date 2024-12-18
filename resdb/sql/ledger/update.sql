UPDATE  ledgers
SET     relation_id = $2,
        descriptions = $3,
        updated_by = $4,
        updated_at = $5
WHERE   id = $1


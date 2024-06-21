SELECT  id,
        name,
        city,
        street,
        phone,
        is_active,
        is_special,
        rel_type AS "rel_type: Vec<RelationType>",
        created_at,
        updated_at
    FROM relations
    ORDER BY name
    LIMIT $1
    OFFSET $2

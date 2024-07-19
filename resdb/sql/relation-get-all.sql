SELECT  id,
        name,
        city,
        street,
        phone,
        is_active,
        is_special,
        relation_type AS "relation_type: Vec<RelationType>",
        created_at,
        updated_at
    FROM relations
    ORDER BY name
    LIMIT $1
    OFFSET $2

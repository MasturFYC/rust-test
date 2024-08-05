SELECT  id,
        name,
        city,
        region,
        street,
        phone,
        is_active,
        is_special,
        relation_type AS "relation_type: Vec<RelationType>",
        photo,
        created_at,
        updated_at
    FROM relations
    ORDER BY name
    LIMIT $1
    OFFSET $2
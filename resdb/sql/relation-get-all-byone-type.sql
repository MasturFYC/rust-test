SELECT id, name, city, street, phone, is_active, is_special,
    relation_type as "relation_type: Vec<RelationType>", photo,
    created_at, updated_at
    FROM relations
    WHERE relation_type::TEXT[] && $1
    ORDER BY name
    LIMIT $2
    OFFSET $3
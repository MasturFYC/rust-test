SELECT id, name, city, street, phone, is_active, is_special, 
    relation_type as "relation_type: Vec<RelationType>",
    created_at, updated_at
    FROM relations
    WHERE relation_type::TEXT[] && $1
    -- WHERE relation_type @> $1
    -- WHERE $1 = ANY(relation_type)
    ORDER BY name
    LIMIT $2
    OFFSET $3

SELECT id, name, city, street, phone, is_active, is_special, 
    rel_type as "rel_type: Vec<RelationType>",
    created_at, updated_at
    FROM relations
    WHERE rel_type::TEXT[] && $1
    -- WHERE rel_type @> $1
    -- WHERE $1 = ANY(rel_type)
    ORDER BY name
    LIMIT $2
    OFFSET $3

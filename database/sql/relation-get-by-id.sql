SELECT id, name, city, street, phone, is_active, is_special,
    relation_type as "relation_type: Vec<RelationType>",
    created_at, updated_at
    FROM relations
    WHERE id = $1

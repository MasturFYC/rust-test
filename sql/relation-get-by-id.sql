SELECT id, name, city, street, phone, is_active, is_special, rel_type as "rel_type: Option<Vec<RelationType>>", created_at, updated_at
    FROM relations
    WHERE id = $1

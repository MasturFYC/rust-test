INSERT INTO relations 
    (name, city, street, phone, is_active, is_special, relation_type)
    VALUES
    ($1, $2, $3, $4, $5, $6, $7::relation_enum[])
    RETURNING id, name, city, street, phone, is_active, is_special, 
    relation_type as "relation_type: Vec<RelationType>",
    created_at, updated_at
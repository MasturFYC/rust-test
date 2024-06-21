INSERT INTO relations 
    (name, city, street, phone, is_active, is_special, rel_type)
    VALUES
    ($1, $2, $3, $4, $5, $6, $7::relation_type[])
    RETURNING id, name, city, street, phone, is_active, is_special, 
    rel_type as "rel_type: Vec<RelationType>",
    created_at, updated_at
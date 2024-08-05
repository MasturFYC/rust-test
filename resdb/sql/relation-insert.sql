INSERT INTO
    relations
        (name, city, street, phone, is_active, is_special, relation_type, photo, region)
    VALUES
        ($1, $2, $3, $4, $5, $6, $7::relation_enum[], $8, $9)
    RETURNING id, name, city, region, street, phone, is_active, is_special,
    relation_type as "relation_type: Vec<RelationType>", photo,
    created_at, updated_at
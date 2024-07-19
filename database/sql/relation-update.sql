UPDATE relations SET
    name = $2,
    city = $3,
    street = $4,
    phone = $5,
    is_active = $6,
    is_special = $7,
    relation_type = $8::relation_enum[],
    created_at = $9
  WHERE id = $1
  RETURNING id, name, city, street, phone, is_active, is_special, 
    relation_type as "relation_type: Vec<RelationType>",
    created_at, updated_at
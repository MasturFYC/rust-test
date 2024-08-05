UPDATE relations SET
    name = $2,
    city = $3,
    street = $4,
    phone = $5,
    is_active = $6,
    is_special = $7,
    relation_type = $8::relation_enum[],
    photo = $9,
    created_at = $10,
    region = $11
  WHERE id = $1
  RETURNING id, name, city, region, street, phone, is_active, is_special,
    relation_type as "relation_type: Vec<RelationType>", photo,
    created_at, updated_at
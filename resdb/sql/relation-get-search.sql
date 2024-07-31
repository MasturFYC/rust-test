SELECT  id,
        name,
        city,
        street,
        phone,
        is_active,
        is_special,
        relation_type AS "relation_type: Vec<RelationType>",
        photo,
        created_at,
        updated_at
    FROM relations
    WHERE POSITION($1 IN LOWER(name||' '||city||' '||COALESCE(phone,' ')||' '||COALESCE(street,' '))) > 0
    ORDER BY name
    LIMIT $2
    OFFSET $3
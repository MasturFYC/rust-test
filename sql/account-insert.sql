INSERT INTO accounts (id, name, root, normal, en_name, descriptions, is_active, payable)
   VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
   RETURNING *
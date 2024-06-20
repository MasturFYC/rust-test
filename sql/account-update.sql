UPDATE accounts SET
   name = $2,
   root = $3,
   normal = $4,
   en_name = $5,
   descriptions = $6,
   is_active = $7,
   payable = $8,
   updated_at = $9
   WHERE id = $1
   RETURNING *
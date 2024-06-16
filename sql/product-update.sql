UPDATE products SET
   name = $1,
   barcode = $2,
   unit = $3,
   content = $4,
   hpp = $5,
   margin = $6,
   price = $7,
   ppn = $8,
   is_active = $9,
   variant_name = $10,
   descriptions = $11,
   category_id = $12,
   updated_at = $13
   WHERE id = $14
   RETURNING *
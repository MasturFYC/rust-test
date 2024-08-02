UPDATE products
SET
   name = $2,
   barcode = $3,
   unit = $4,
   content = $5,
   hpp = $6,
   margin = $7,
   price = $8,
   ppn = $9,
   heavy = $10,
   is_active = $11,
   variant_name = $12,
   descriptions = $13,
   category_id = $14,
   supplier_id = $15
WHERE id = $1
   RETURNING *
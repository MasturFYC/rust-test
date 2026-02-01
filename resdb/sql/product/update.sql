UPDATE products
SET
   name = $2,
   barcode = $3,
   unit = $4,
   content = $5,
   hpp = $6,
   margin = $7,
   price = $8,
   heavy = $9,
   is_active = $10,
   variant_name = $11,
   descriptions = $12,
   category_id = $13
WHERE id = $1

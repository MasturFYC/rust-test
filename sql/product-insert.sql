INSERT INTO products
   (name, barcode, unit, content, hpp, margin, price, ppn, is_active, variant_name, descriptions, category_id, supplier_id)
   VALUES
   ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
   RETURNING *
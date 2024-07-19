UPDATE order_details
SET order_id = $2,
   product_id = $3,
   qty = $4,
   direction = $5,
   unit = $6,
   price = $7,
   discount = $8,
   hpp = $9,
   subtotal = $10,
   updated_at = now()
WHERE id = $1

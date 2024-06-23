INSERT INTO order_details (
   order_id
   , product_id
   , qty
   , direction
   , unit
   , price
   , discount
   , hpp
   , created_at
) VALUES (
   $1,$2,$3,$4,$5,$6,$7,$8,$9
) RETURNING id, created_at, updated_at

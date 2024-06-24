SELECT order_id,
   id,
   product_id,
   qty,
   direction,
   unit,
   price,
   discount,
   hpp,
   created_at,
   updated_at
FROM order_details
WHERE order_id = $1
ORDER BY order_id,
   created_at
SELECT order_id,
   detail_id,
   product_id,
   qty,
   direction,
   unit,
   price,
   discount,
   hpp,
   created_at,
   updated_at,
   subtotal
FROM order_details
WHERE order_id = $1
ORDER BY order_id, detail_id
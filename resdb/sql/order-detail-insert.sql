INSERT INTO order_details (
      order_id,
      product_id,
      qty,
      direction,
      unit,
      price,
      discount,
      hpp,
      subtotal
   )
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)

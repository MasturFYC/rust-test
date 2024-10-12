INSERT INTO order_details (
      order_id,
      product_id,
      qty,
      direction,
      unit,
      price,
      discount,
      hpp,
      subtotal,
      detail_id,
      gudang_id
   )
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
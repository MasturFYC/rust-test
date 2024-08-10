SELECT
   d.order_id,
   d.detail_id,
   d.product_id,
   d.qty,
   d.direction,
   d.unit,
   d.price,
   d.discount,
   d.hpp,
   d.created_at,
   d.updated_at,
   d.subtotal,
   p.name,
   p.barcode,
   d.qty as unit_in_stock
FROM
    order_details d
INNER JOIN
    products p ON p.id = d.product_id
WHERE
    d.order_id = $1
ORDER BY
    d.order_id, d.detail_id
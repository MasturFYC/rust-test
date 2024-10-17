SELECT
   d.order_id,
   d.detail_id,
   d.product_id,
   d.gudang_id,
   g.name AS gudang_name,
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
   d.qty AS old_qty,
   d.gudang_id AS old_gudang_id
FROM
    order_details d
INNER JOIN
    products p ON p.id = d.product_id
INNER JOIN
    gudangs g ON g.id = d.gudang_id
WHERE
    d.order_id = $1
ORDER BY
    d.order_id, d.detail_id
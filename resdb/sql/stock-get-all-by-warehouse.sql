SELECT o.id,
   o.customer_id,
   o.sales_id,
   o.payment_type AS "payment_type: PaymentType",
   o.updated_by,
   o.total,
   o.dp,
   o.payment,
   o.remain,
   o.invoice_id,
   o.due_at,
   o.created_at,
   o.updated_at,
   c.name AS supplier_name,
   s.name AS warehouse_name,
   o.is_protected
FROM orders AS o
   INNER JOIN relations AS c ON c.id = o.customer_id
   INNER JOIN relations AS s ON s.id = o.sales_id
WHERE o.order_type = 'stock'::order_enum
      AND o.sales_id = $1
ORDER BY o.id DESC
LIMIT $2 OFFSET $3

SELECT o.id,
   o.order_type AS "order_type: OrderType",
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
   o.is_protected,
   o.created_at,
   o.updated_at,
   c.name AS customer_name,
   s.name AS sales_name
FROM orders AS o
   INNER JOIN relations AS c ON c.id = o.customer_id
   INNER JOIN relations AS s ON s.id = o.sales_id
WHERE o.order_type = 'order'::order_enum
ORDER BY o.id DESC
LIMIT $1 OFFSET $2
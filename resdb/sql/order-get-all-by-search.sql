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
   o.created_at,
   o.updated_at,
   c.name AS customer_name,
   s.name AS sales_name,
   o.is_protected
FROM orders AS o
   INNER JOIN relations AS c ON c.id = o.customer_id
   INNER JOIN relations AS s ON s.id = o.sales_id
WHERE o.order_type = 'order'::order_enum
        AND POSITION($1 IN (o.id::TEXT||' '||LOWER(c.name||' '||s.name))) > 0
ORDER BY o.id DESC
LIMIT $2 OFFSET $3
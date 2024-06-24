SELECT o.id,
   o.order_type AS "order_type: OrderType",
   o.relation_id,
   o.payment_type AS "payment_type: PaymentType",
   o.updated_by,
   o.total,
   o.payment,
   o.remain,
   o.invoice_id,
   o.due_at,
   o.created_at,
   o.updated_at,
   r.name AS relation_name
FROM orders AS o
   INNER JOIN relations AS r ON r.id = o.relation_id
WHERE o.order_type = 'order'::order_enum
ORDER BY o.created_at
LIMIT $1 OFFSET $2
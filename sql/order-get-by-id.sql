SELECT id,
   order_type as "order_type: OrderType",
   relation_id,
   payment_type as "payment_type: PaymentType",
   updated_by,
   total,
   payment,
   remain,
   invoice_id,
   due_at,
   created_at,
   updated_at
FROM orders
WHERE id = $1
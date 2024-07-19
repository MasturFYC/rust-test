SELECT id,
   order_type as "order_type: OrderType",
   customer_id,
   sales_id,
   payment_type as "payment_type: PaymentType",
   updated_by,
   total,
   dp,
   payment,
   remain,
   invoice_id,
   due_at,
   is_protected,
   created_at,
   updated_at
FROM orders
WHERE id = $1
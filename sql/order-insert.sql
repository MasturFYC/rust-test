INSERT INTO "orders" (
    order_type,
    relation_id,
    payment_type,
    updated_by,
    total,
    dp,
    payment,
    remain,
    invoice_id,
    due_at,
    created_at
)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
RETURNING id,
   order_type as "order_type: OrderType",
   relation_id,
   payment_type as "payment_type: PaymentType",
   updated_by,
   total,
   dp,
   payment,
   remain,
   invoice_id,
   due_at,
   created_at,
   updated_at

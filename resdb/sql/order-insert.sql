INSERT INTO "orders" (
    order_type,
    customer_id,
    sales_id,
    payment_type,
    updated_by,
    total,
    dp,
    payment,
    remain,
    invoice_id,
    due_at,
    is_protected,
    created_at
)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
RETURNING id
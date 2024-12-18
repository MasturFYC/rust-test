UPDATE "orders" SET
    customer_id = $2,
    sales_id = $3,
    payment_type = $4,
    updated_by = $5,
    total = $6,
    dp = $7,
    payment = $8,
    remain = $9,
    invoice_id = $10,
    due_at = $11,
    created_at = $12,
    updated_at = now()
WHERE id = $1
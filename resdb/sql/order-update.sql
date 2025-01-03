UPDATE "orders"
SET order_type = $2,
   customer_id = $3,
   sales_id = $4,
   payment_type = $5,
   updated_by = $6,
   total = $7,
   dp = $8,
   payment = $9,
   remain = $10,
   invoice_id = $11,
   due_at = $12,
   is_protected = $13,
   created_at = $14,
   updated_at = now()
WHERE id = $1
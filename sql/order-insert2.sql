INSERT INTO "orders" (
        order_type,
        relation_id,
        payment_type,
        updated_by,
        total,
        payment,
        remain,
        invoice_id,
        due_at,
        created_at
    )
    VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)
    RETURNING id, updated_at
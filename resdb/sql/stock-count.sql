SELECT COUNT(*) AS total
FROM orders
WHERE order_type = 'stock'::order_enum

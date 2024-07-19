SELECT COUNT(*) AS total
FROM orders
WHERE order_type = 'order'::order_enum

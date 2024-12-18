SELECT  COUNT(*)
FROM	orders AS o
WHERE	o.order_type = 'order'::order_enum
		AND o.customer_id = $1


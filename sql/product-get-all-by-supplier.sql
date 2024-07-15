SELECT p.*,
   c.name AS category_name,
   r.name AS supplier_name
FROM products p
   INNER JOIN categories c ON c.id = p.category_id
   INNER JOIN relations r ON r.id = p.supplier_id
WHERE p.supplier_id = $1
ORDER BY p.supplier_id, p.name
LIMIT $2 OFFSET $3;
SELECT p.*,
   c.name AS category_name
FROM products p
   INNER JOIN categories c ON c.id = p.category_id
LIMIT $1 OFFSET $2;
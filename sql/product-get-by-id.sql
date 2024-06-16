SELECT p.*, c.name AS category_name 
   FROM products p 
   INNER JOIN categories c ON c.id = p.category_id 
   WHERE p.id = $1
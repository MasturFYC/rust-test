SELECT c.id, c.name, 
   COALESCE((SELECT json_agg(x) FROM (
      SELECT p.id, p.name
      FROM products p
      WHERE p.category_id = c.id
   ) AS x), '[]') AS "products"
   FROM categories c
   WHERE c.id = $1;

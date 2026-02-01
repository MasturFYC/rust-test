SELECT  p.*,
        c.name AS category_name,
        COALESCE((SELECT json_agg(x) FROM (
                    SELECT s.warehouse_id AS "warehouseId", s.product_id AS "productId", s.qty, g.name
                    FROM stocks s
                    INNER JOIN warehouses g ON g.id = s.warehouse_id
                    WHERE s.product_id = p.id
                    ORDER BY g.name
        ) AS x), '[]') AS "stocks!: Json<Vec<ProductStock>>"
FROM products p
   INNER JOIN categories c ON c.id = p.category_id
WHERE p.category_id = $1
ORDER BY p.category_id, p.name
LIMIT $2 OFFSET $3;

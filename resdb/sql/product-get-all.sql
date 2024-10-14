SELECT  p.*,
        c.name AS category_name,
        r.name AS supplier_name,
        COALESCE((SELECT json_agg(x) FROM (
                    SELECT s.gudang_id AS "gudangId", s.product_id AS "productId", s.qty, g.name
                    FROM stocks s
                    INNER JOIN gudangs g ON g.id = s.gudang_id
                    WHERE s.product_id = p.id
                    ORDER BY g.name
        ) AS x), '[]') AS "stocks!: Json<Vec<ProductStock>>"
FROM products p
   INNER JOIN categories c ON c.id = p.category_id
   INNER JOIN relations r ON r.id = p.supplier_id
ORDER BY p.name
LIMIT $1 OFFSET $2;
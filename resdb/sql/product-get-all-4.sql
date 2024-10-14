SELECT
    p.*,
    c.name AS category_name,
    r.name AS supplier_name,
    COALESCE((SELECT json_agg(x) FROM (
        SELECT s.gudang_id AS "gudangId", s.product_id AS "productId", s.qty, g.name
        FROM stocks s
        INNER JOIN gudangs g ON g.id = s.gudang_id
        WHERE s.product_id = p.id
        ORDER BY g.name
    ) AS x), '[]') AS "stocks!: Json<Vec<ProductStock>>"
FROM
    products AS p
INNER JOIN categories AS c ON c.id = p.category_id
INNER JOIN relations AS r ON r.id = p.supplier_id
WHERE
    category_id = $1
ORDER BY
    p.name
LIMIT $2
OFFSET $3
SELECT  p.id,
        p.name,
        p.barcode,
        p.unit,
        p.hpp,
        p.price,
        COALESCE(j.discount, 0.0)::NUMERIC(9,2) AS discount
FROM    products p
        LEFT JOIN (
          SELECT    d.order_id, d.product_id, d.discount
          FROM      order_details d
                    INNER JOIN orders o ON o.id = d.order_id
          WHERE     o.order_type = 'order'::order_enum
                    AND o.customer_id = $1 
                    AND d.discount > 0::NUMERIC(9,2)
          ORDER BY  d.order_id DESC
        ) AS j ON j.product_id = p.id 
WHERE   p.barcode = $2
LIMIT 1

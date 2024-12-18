SELECT  COUNT(*)
FROM    orders AS o
        INNER JOIN relations AS c ON c.id = o.customer_id
        INNER JOIN relations AS s ON s.id = o.sales_id
WHERE   o.order_type = 'order'::order_enum
        AND POSITION($1 IN (o.id::TEXT||' '||LOWER(c.name||' '||s.name))) > 0


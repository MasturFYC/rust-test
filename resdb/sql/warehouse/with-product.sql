     SELECT	c.id
		, c.name
		, c.employee_id
		, r.name AS employee_name
		, c.locate,
   		COALESCE((SELECT json_agg(x)
			    FROM (
				 SELECT      p.id, p.name
      				   FROM	     products p
      				  WHERE      p.id = s.product_id
				  ) AS x), '[]') AS "products!: Json<Vec<WarehouseProduct>>"
   -- ) AS x), '[]') AS "products"
      FROM     warehouses c
INNER JOIN     relations AS r ON r.id = c.employee_id
INNER JOIN     stocks AS s ON s.warehouse_id = c.id
     WHERE     c.id = $1;

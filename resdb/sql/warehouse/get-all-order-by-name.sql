SELECT
	t.id
	, t.name
	, t.employee_id
	, r.name AS employee_name
	, t.locate
FROM
	warehouses AS t
INNER JOIN
	relations AS r ON r.id = t.employee_id
ORDER BY
	t.name;

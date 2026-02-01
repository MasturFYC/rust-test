INSERT INTO	warehouses
       		(name, employee_id, locate)
VALUES 		($1, $2, $3)
RETURNING id;

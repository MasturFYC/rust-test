SELECT t.id, t.name, t.employee_id, r.name AS employee_name, t.locate
    FROM gudangs AS t
    INNER JOIN relations AS r ON r.id = t.employee_id
    WHERE t.id = $1;
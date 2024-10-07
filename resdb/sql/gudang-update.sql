UPDATE gudangs
    SET name = $2,
        employee_id = $3,
        locate = $4
    WHERE id = $1
    RETURNING *;
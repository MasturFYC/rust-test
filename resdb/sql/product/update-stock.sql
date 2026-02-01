UPDATE products SET
    unit_in_stock = unit_in_stock - $2
    WHERE id = $1
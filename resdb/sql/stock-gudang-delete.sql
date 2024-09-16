DELETE FROM stocks
    WHERE gudang_id = $1 AND product_id = $2;
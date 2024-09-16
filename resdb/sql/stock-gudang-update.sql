    UPDATE stock_gudangs
        SET qty = (qty + $3)
        WHERE gudang_id = $1 AND product_id = $2;
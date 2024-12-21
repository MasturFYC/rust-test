SELECT  p.id,
        p.name,
        p.barcode,
        p.unit,
        p.hpp,
        p.price,
        0.0::NUMERIC(9,2) AS discount
FROM    products p
WHERE   p.barcode = $1 

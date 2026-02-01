    SELECT   COUNT(p.*)
      FROM   products AS p
INNER JOIN   categories AS c ON c.id = p.category_id
     WHERE   POSITION($1 IN LOWER(c.name||' '||p.name||' '||p.barcode||' '||COALESCE(p.variant_name,'')||' '||COALESCE(p.descriptions, ''))) > 0

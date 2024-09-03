SELECT
        id,
        name,
        root,
        normal,
        en_name,
        descriptions,
        is_active,
        payable,
        created_at,
        updated_at
FROM
        accounts
LIMIT   $1
OFFSET  $2
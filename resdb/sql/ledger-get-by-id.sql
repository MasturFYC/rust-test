SELECT
    t.id,
    t.relation_id,
    t.ledger_type as "ledger_type: LedgerType",
    t.updated_by,
    t.is_valid,
    t.descriptions,
    t.created_at,
    t.updated_at,
    COALESCE((SELECT json_agg(x) FROM (
      SELECT
         d.ledger_id,
         d.detail_id,
         d.account_id,
         a.name,
         d.descriptions,
         d.amount,
         d.direction,
         d.ref_id
      FROM ledger_details AS d
      INNER JOIN accounts AS a
         ON a.id = d.account_id
      WHERE d.ledger_id = t.id
      ORDER BY d.ledger_id, d.detail_id
        ) AS x), '[]') AS "details!: Json<Vec<LedgerDetail>>"
FROM
    ledgers AS t
WHERE
    t.id = $1
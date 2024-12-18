INSERT 
INTO ledger_details (
    ledger_id,
    detail_id,
    account_id,
    descriptions,
    amount,
    direction,
    ref_id
)
VALUES ($1, $2, $3, $4, $5, $6, $7)


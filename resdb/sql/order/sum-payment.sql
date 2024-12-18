SELECT  COALESCE(SUM(amount), 0)
FROM    order_payments 
WHERE   order_id = $1

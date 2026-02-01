with product_sales as (
  select      id, name, barcode, unit, hpp, price
  from        products
  where       barcode = $2
), product_discount as (
  select      d.updated_at, d.product_id, d.discount 
  from        order_details d 
  inner join  orders o on o.order_id = d.order_id 
  where       d.product_id = (select id from product_sales) 
              and o.customer_id = $1 
              and d.updated_at >= now() - interval '1 week' 
  order by    d.updated_at desc
)

select    p.id
          , p.name
          , p.barcode
          , p.unit
          , p.hpp
          , p.price
          , d.discount 
from      product_sales p 
left join product_discount d 
          on d.product_id = p.id;

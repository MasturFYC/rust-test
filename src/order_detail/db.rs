use async_trait::async_trait;
use chrono::Utc;
use sqlx::{self, Acquire};

use super::{
    CreateOrderDetailSchema, MatchTrxResult, OrderDetail,
    OrderDetailCreateReturn, TrxOrder,
};
use crate::{
    order::{CreateOrderSchema, Order, OrderType, PaymentType},
    DBClient,
};

#[async_trait]
pub trait OrderDetailExt {
    async fn get_details(&self, order_id: uuid::Uuid) -> Result<Vec<OrderDetail>, sqlx::Error>;
    async fn order_detail_create<T: Into<CreateOrderDetailSchema> + Send>(
        &self,
        data: T,
    ) -> Result<Option<OrderDetailCreateReturn>, sqlx::Error>;
    async fn create_order_with_details<
        O: Into<CreateOrderSchema> + Send,
        T: Into<Vec<CreateOrderDetailSchema>> + Send,
    >(
        &self,
        new_order: O,
        data: T,
    ) -> Result<MatchTrxResult, sqlx::Error>;
}

#[async_trait]
impl OrderDetailExt for DBClient {
    async fn get_details(&self, order_id: uuid::Uuid) -> Result<Vec<OrderDetail>, sqlx::Error> {
        let details: Vec<OrderDetail> =
            sqlx::query_file_as!(OrderDetail, "sql/order-detail-get-by-order.sql", order_id)
                .fetch_all(&self.pool)
                .await?;

        Ok(details)
    }
    async fn order_detail_create<T: Into<CreateOrderDetailSchema> + Send>(
        &self,
        data: T,
    ) -> Result<Option<OrderDetailCreateReturn>, sqlx::Error> {
        let t: CreateOrderDetailSchema = data.try_into().unwrap();
        let detail: Option<OrderDetailCreateReturn> = sqlx::query_file_as!(
            OrderDetailCreateReturn,
            "sql/order-detail-insert.sql",
            t.order_id.to_owned(),
            t.product_id.to_owned(),
            t.qty.to_owned(),
            t.direction.to_owned(),
            t.unit.to_owned(),
            t.price.to_owned(),
            t.discount.to_owned(),
            t.hpp.to_owned(),
            Utc::now()            
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(detail)
    }

    async fn create_order_with_details<        
        O: Into<CreateOrderSchema> + Send,
        T: Into<Vec<CreateOrderDetailSchema>> + std::marker::Send,
    >(
        &self,
        new_order: O,
        data: T,
    ) -> Result<MatchTrxResult, sqlx::Error> {
        let details: Vec<CreateOrderDetailSchema> = data.try_into().unwrap();

        let t: CreateOrderSchema = new_order.try_into().unwrap();
        let o: CreateOrderSchema = CreateOrderSchema::set_default(&t);

        let mut conn: sqlx::pool::PoolConnection<sqlx::Postgres> = self.pool.acquire().await?;
        let mut tx: sqlx::Transaction<sqlx::Postgres> = conn.begin().await?;

        // println!("{:?}", o);

        let order = sqlx::query_file_as!(
            TrxOrder,
            "sql/order-insert2.sql",
            o.order_type.unwrap() as OrderType,
            o.relation_id.to_owned(),
            o.payment_type.unwrap() as PaymentType,
            o.updated_by.to_owned(),
            o.total.to_owned(),
            o.payment.to_owned(),
            o.remain.to_owned(),
            o.invoice_id.to_owned(),
            o.due_at.to_owned(),
            o.created_at.to_owned() // .unwrap_or(Utc::now())
        )
        .fetch_optional(&mut *tx)
        .await?;

        // if order.is_none() {
        //    ()
        // }

        let ord = order.unwrap();
        let new_id = ord.id;

        for (_, d) in details.iter().enumerate() {
            let _row = sqlx::query_file!(
                "sql/order-detail-insert.sql",
                new_id.to_owned(),
                d.product_id.to_owned(),
                d.qty.to_owned(),
                d.direction.to_owned(),
                d.unit.to_owned(),
                d.price.to_owned(),
                d.discount.to_owned(),
                d.hpp.to_owned(),
                Utc::now()
            )
            .fetch_one(&mut *tx)
            .await?;
        }

        //   println!("order: {:?}, with new id: {}", ord, new_id);

        let details: Vec<OrderDetail> =
            sqlx::query_file_as!(OrderDetail, "sql/order-detail-get-by-order.sql", new_id)
                .fetch_all(&mut *tx)
                .await?;

        tx.commit().await?;

        Ok((
            Some(Order {
                id: new_id,
                relation_id: o.relation_id,
                created_at: o.created_at,
                due_at: o.due_at,
                updated_at: ord.updated_at,
                invoice_id: o.invoice_id,
                order_type: o.order_type.unwrap(),
                payment: o.payment,
                remain: o.remain,
                total: o.total,
                payment_type: o.payment_type.unwrap(),
                updated_by: o.updated_by,
            }),
            details,
        ))
    }
}

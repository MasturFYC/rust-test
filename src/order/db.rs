use async_trait::async_trait;
use chrono::Utc;
use sqlx::{self, Acquire};

use super::{CreateOrderSchema, Order, OrderType, PaymentType};
use crate::DBClient;

type MatchResult = (Vec<Order>, i64);

#[async_trait]
pub trait OrderExt {
    async fn get_order(&self, id: uuid::Uuid) -> Result<Option<Order>, sqlx::Error>;
    async fn get_orders(&self, page: usize, limit: usize) -> Result<MatchResult, sqlx::Error>;
    async fn order_create<T: Into<CreateOrderSchema> + Send>(
        &self,
        data: T,
    ) -> Result<Option<Order>, sqlx::Error>;
    async fn order_update<T: Into<CreateOrderSchema> + Send>(
        &self,
        id: uuid::Uuid,
        data: T,
    ) -> Result<Option<Order>, sqlx::Error>;
    async fn order_delete(&self, id: uuid::Uuid) -> Result<u64, sqlx::Error>;
    async fn order_count(&self) -> Result<Option<i64>, sqlx::Error>;
}

#[async_trait]
impl OrderExt for DBClient {
    async fn get_order(&self, id: uuid::Uuid) -> Result<Option<Order>, sqlx::Error> {
        let order = sqlx::query_file_as!(Order, "sql/order-get-by-id.sql", id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(order)
    }

    /// call using curl
    /// curl localhost:8000/api/orders \
    /// -H "content-type: application/json" \
    /// -H "Authorization: Bearer $(cat token.txt)"
    async fn get_orders(&self, page: usize, limit: usize) -> Result<MatchResult, sqlx::Error> {
        let offset = (page - 1) * limit;

        // acquire pg connection from current pool
        let mut conn = self.pool.acquire().await?; //.unwrap();

        // get transaction pool from pg connection
        let mut tx = conn.begin().await?;

        // start transaction
        // get orders data from database
        let orders = sqlx::query_file_as!(
            Order,
            "sql/order-get-all.sql",
            limit as i64,
            offset as i64
        )
        .fetch_all(&mut *tx)
        .await?;

        // start transacrion
        // get total record of orders
        let row = sqlx::query_scalar!("SELECT COUNT(*) as total FROM orders")
            .fetch_one(&mut *tx)
            .await?;    

        // finish transaction
        tx.commit().await?;

        // return result to caller
        Ok((orders, row.unwrap_or(0)))
    }


    async fn order_create<T: Into<CreateOrderSchema> + Send>(
        &self,
        data: T,
    ) -> Result<Option<Order>, sqlx::Error> {
        let t: CreateOrderSchema = data.try_into().unwrap();

        let temp = CreateOrderSchema::set_default(&t);

        let order = sqlx::query_file_as!(
            Order,
            "sql/order-insert.sql",
            temp.order_type.unwrap() as OrderType,
            temp.relation_id.to_owned(),
            temp.payment_type.unwrap() as PaymentType,
            temp.updated_by.to_owned(),
            temp.total.to_owned(),
            temp.payment.to_owned(),
            temp.remain.to_owned(),
            temp.invoice_id.to_owned(),
            temp.due_at.to_owned(), // .unwrap_or(temp.created_at.unwrap_or(Utc::now()).checked_add_days(chrono::Days::new(7)).unwrap()),
            temp.created_at.unwrap_or(Utc::now())
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(order)
    }

    async fn order_update<T: Into<CreateOrderSchema> + Send>(
        &self,
        id: uuid::Uuid,
        data: T,
    ) -> Result<Option<Order>, sqlx::Error> {
        let t: CreateOrderSchema = data.try_into().unwrap();

        let temp = CreateOrderSchema::set_default(&t);
        let order = sqlx::query_file_as!(
            Order,
            "sql/order-update.sql",
            id,
            temp.order_type.unwrap() as OrderType,
            temp.relation_id.to_owned(),
            temp.payment_type.unwrap() as PaymentType,
            temp.updated_by.into(),
            temp.total.into(),
            temp.payment.into(),
            temp.remain.into(),
            temp.invoice_id.to_owned(),
            temp.due_at.to_owned(),
            temp.created_at.to_owned()
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(order)
    }

    async fn order_delete(&self, id: uuid::Uuid) -> Result<u64, sqlx::Error> {
        let rows_affected: u64 = sqlx::query_file!("sql/order-delete.sql", id)
            .execute(&self.pool)
            .await
            .unwrap()
            .rows_affected();

        Ok(rows_affected)
    }

    async fn order_count(&self) -> Result<Option<i64>, sqlx::Error> {
        let row = sqlx::query_scalar!("SELECT COUNT(*) as total FROM orders")
            .fetch_one(&self.pool)
            .await?;

        Ok(row)        
    }

}

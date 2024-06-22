use async_trait::async_trait;
use sqlx;

use super::{CreateOrderSchema, Order, OrderType, PaymentType};
use crate::DBClient;

#[async_trait]
pub trait OrderExt {
    async fn get_order(&self, id: uuid::Uuid) -> Result<Option<Order>, sqlx::Error>;
    async fn get_orders(&self, page: usize, limit: usize) -> Result<Vec<Order>, sqlx::Error>;
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

    async fn get_orders(&self, page: usize, limit: usize) -> Result<Vec<Order>, sqlx::Error> {
        let offset = (page - 1) * limit;

        // let mut tx = &self.pool.begin();

        let orders = sqlx::query_file_as!(
            Order,
            "sql/order-get-all.sql",
            limit as i64,
            offset as i64
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(orders)
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

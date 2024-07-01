use async_trait::async_trait;
use bigdecimal::{BigDecimal, FromPrimitive};
use sqlx::{self, Acquire};
use uuid::Uuid;

use crate::{
    db::DBClient,
    ledger::{CreateLedgerSchema, LedgerDetail},
};

use super::{
    CreateOrderDetailSchema, DetailMark, MatchResult, MatchTrxResult, Order, OrderDetail,
    OrderDtos, OrderType, PaymentType, ResponseOrder,
};
// use crate::{order_detail::{CreateOrderDetailSchema, MatchTrxResult, OrderDetail}, DBClient};

#[async_trait]
pub trait OrderExt {
    async fn get_order(&self, id: uuid::Uuid) -> Result<Option<Order>, sqlx::Error>;
    async fn get_orders(&self, page: usize, limit: usize) -> Result<MatchResult, sqlx::Error>;
    async fn order_create<O: Into<OrderDtos> + Send, T: Into<Vec<CreateOrderDetailSchema>> + Send>(
        &self,
        data: O,
        details: T,
    ) -> Result<MatchTrxResult, sqlx::Error>;
    async fn order_update<
        O: Into<OrderDtos> + Send,
        T: Into<Vec<CreateOrderDetailSchema>> + Send,
        S: Into<Uuid> + Send,
    >(
        &self,
        id: S,
        data: O,
        details: T,
    ) -> Result<MatchTrxResult, sqlx::Error>;
    async fn order_delete(&self, id: uuid::Uuid) -> Result<u64, sqlx::Error>;
    #[allow(dead_code)]
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
            ResponseOrder,
            "sql/order-get-all.sql",
            limit as i64,
            offset as i64
        )
        .fetch_all(&mut *tx)
        .await?;

        // start transacrion
        // get total record of orders
        let row = sqlx::query_file_scalar!("sql/order-count.sql")
            .fetch_one(&mut *tx)
            .await?;

        // finish transaction
        tx.commit().await?;

        // return result to caller
        Ok((orders, row.unwrap_or(0)))
    }

    async fn order_create<
        O: Into<OrderDtos> + Send,
        T: Into<Vec<CreateOrderDetailSchema>> + Send,
    >(
        &self,
        data: O,
        details: T,
    ) -> Result<MatchTrxResult, sqlx::Error> {
        let details: Vec<CreateOrderDetailSchema> = details.try_into().unwrap();

        let t: OrderDtos = data.try_into().unwrap();
        let o: OrderDtos = OrderDtos::set_default(&t);

        let mut conn: sqlx::pool::PoolConnection<sqlx::Postgres> = self.pool.acquire().await?;
        let mut tx: sqlx::Transaction<sqlx::Postgres> = conn.begin().await?;

        // println!("{:?}", o);

        let order = sqlx::query_file_as!(
            Order,
            "sql/order-insert.sql",
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

        let ord = order.unwrap();
        let new_id = ord.id;

        let ledger = create_ledger(o);
        let pass = BigDecimal::from_i32(0).unwrap();
        let modal = details.iter().fold(pass, |o, t| o + &t.hpp);
        let ledger_details =
            create_ledger_details(&ord.payment, &ord.total, modal, Some(new_id), new_id);

        for (_, d) in details.into_iter().enumerate() {
            let _ = sqlx::query_file!(
                "sql/order-detail-insert.sql",
                new_id.to_owned(),
                d.product_id.to_owned(),
                d.qty.to_owned(),
                d.direction.to_owned(),
                d.unit.to_owned(),
                d.price.to_owned(),
                d.discount.to_owned(),
                d.hpp.to_owned()
            )
            .execute(&mut *tx)
            // .fetch_one(&mut *tx)
            .await?;

            let _ = sqlx::query!(
                "UPDATE products SET unit_in_stock = (unit_in_stock - $2) WHERE id = $1",
                d.product_id.to_owned(),
                d.qty.to_owned(),
            )
            .execute(&mut *tx)
            .await?;
        }

        let details: Vec<OrderDetail> =
            sqlx::query_file_as!(OrderDetail, "sql/order-detail-get-by-order.sql", new_id)
                .fetch_all(&mut *tx)
                .await?;

        tx.commit().await?;

        Ok((Some(ord), details))
    }

    async fn order_update<
        O: Into<OrderDtos> + Send,
        T: Into<Vec<CreateOrderDetailSchema>> + Send,
        S: Into<Uuid> + Send,
    >(
        &self,
        id: S,
        data: O,
        details: T,
    ) -> Result<MatchTrxResult, sqlx::Error> {
        let t: OrderDtos = data.try_into().unwrap();
        let details: Vec<CreateOrderDetailSchema> = details.try_into().unwrap();
        let uid: Uuid = id.try_into().unwrap();
        let o = OrderDtos::set_default(&t);

        let mut conn: sqlx::pool::PoolConnection<sqlx::Postgres> = self.pool.acquire().await?;
        let mut tx: sqlx::Transaction<sqlx::Postgres> = conn.begin().await?;

        let order = sqlx::query_file_as!(
            Order,
            "sql/order-update.sql",
            uid,
            o.order_type.unwrap() as OrderType,
            o.relation_id.to_owned(),
            o.payment_type.unwrap() as PaymentType,
            o.updated_by.into(),
            o.total.into(),
            o.payment.into(),
            o.remain.into(),
            o.invoice_id.to_owned(),
            o.due_at.to_owned(),
            o.created_at.to_owned()
        )
        .fetch_optional(&mut *tx)
        .await?;

        for (_, d) in details.iter().enumerate() {
            match d.mark_as.unwrap() {
                DetailMark::Delete => {
                    let _ = sqlx::query!(
                        "UPDATE products SET unit_in_stock = (unit_in_stock + $2) WHERE id = $1",
                        d.old_product_id.to_owned(),
                        d.old_qty,
                    )
                    .execute(&mut *tx)
                    .await?;

                    let _ = sqlx::query_file!("sql/order-detail-delete.sql", d.id.to_owned(),)
                        .execute(&mut *tx)
                        .await?;
                }

                DetailMark::Update => {
                    let _ = sqlx::query_file!(
                        "sql/order-detail-update.sql",
                        d.id.to_owned(),
                        // order_id
                        uid,
                        d.product_id.to_owned(),
                        d.qty.to_owned(),
                        d.direction.to_owned(),
                        d.unit.to_owned(),
                        d.price.to_owned(),
                        d.discount.to_owned(),
                        d.hpp.to_owned(),
                    )
                    .execute(&mut *tx)
                    .await?;

                    let _ = sqlx::query!(
                        "UPDATE products SET unit_in_stock = (unit_in_stock + $2) WHERE id = $1",
                        d.old_product_id.to_owned(),
                        d.old_qty.to_owned()
                    )
                    .execute(&mut *tx)
                    .await?;

                    let _ = sqlx::query!(
                        "UPDATE products SET unit_in_stock = (unit_in_stock - $2) WHERE id = $1",
                        d.product_id.to_owned(),
                        d.qty.to_owned(),
                    )
                    .execute(&mut *tx)
                    .await?;
                }
                _ => {
                    let _ = sqlx::query_file!(
                        "sql/order-detail-insert.sql",
                        uid,
                        d.product_id.to_owned(),
                        d.qty.to_owned(),
                        d.direction.to_owned(),
                        d.unit.to_owned(),
                        d.price.to_owned(),
                        d.discount.to_owned(),
                        d.hpp.to_owned()
                    )
                    .execute(&mut *tx)
                    .await?;

                    let _ = sqlx::query!(
                        "UPDATE products SET unit_in_stock = (unit_in_stock - $2) WHERE id = $1",
                        d.product_id.to_owned(),
                        d.qty.to_owned(),
                    )
                    .execute(&mut *tx)
                    .await?;
                }
            }
        }

        let details: Vec<OrderDetail> =
            sqlx::query_file_as!(OrderDetail, "sql/order-detail-get-by-order.sql", uid)
                .fetch_all(&mut *tx)
                .await?;

        tx.commit().await?;

        Ok((order, details))
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

fn create_ledger(o: OrderDtos) -> CreateLedgerSchema {
    CreateLedgerSchema {
        descriptions: Some(String::from("")),
        is_valid: true,
        name: String::from("ORDER"),
        relation_id: o.relation_id,
        updated_by: o.updated_by,
    }
}

fn create_ledger_details(
    payment: &BigDecimal,
    total: &BigDecimal,
    modal: BigDecimal,
    ref_id: Option<Uuid>,
    ledger_id: Uuid,
) -> Vec<LedgerDetail> {
    let mut ledger_details: Vec<LedgerDetail> = Vec::new();
    let mut i: i16 = 1;

    let remain = total - payment;
    let pass = bigdecimal::BigDecimal::from_i32(0).unwrap();

    ledger_details.push(LedgerDetail {
        account_id: 421,
        amount: total.to_owned(),
        name: String::from("Total Order"),
        descriptions: Some(String::from("Penjualan barang")),
        direction: -1,
        id: i,
        ref_id,
        ledger_id,
    });

    i += 1;

    if remain.le(&pass) {
        ledger_details.push(LedgerDetail {
            account_id: 101,
            amount: total.to_owned(),
            name: String::from("Kas"),
            descriptions: Some(String::from("Kas")),
            direction: 1,
            id: i,
            ref_id,
            ledger_id,
        });
    } else {
        // jika tidak ada pembayaran
        ledger_details.push(LedgerDetail {
            account_id: 111,
            amount: remain,
            name: String::from("Piutang barang"),
            descriptions: Some(String::from("Piutang barang")),
            direction: 1,
            id: i,
            ref_id,
            ledger_id,
        });

        // jika ada pembayaran
        if payment.gt(&pass) {
            i += 1;
            ledger_details.push(LedgerDetail {
                account_id: 101,
                amount: payment.to_owned(),
                name: String::from("Cash DP"),
                descriptions: Some(String::from("Cash DP")),
                direction: 1,
                id: i,
                ref_id,
                ledger_id,
            });
        }
    }

    i += 1;

    ledger_details.push(LedgerDetail {
        account_id: 106,
        amount: modal.to_owned(),
        name: String::from("Persediaan barang"),
        descriptions: Some(String::from("Persediaan barang")),
        direction: -1,
        id: i,
        ref_id,
        ledger_id,
    });

    i += 1;

    ledger_details.push(LedgerDetail {
        account_id: 521,
        amount: modal,
        name: String::from("Biaya Beli Barang"),
        descriptions: Some(String::from("Biaya Beli Barang")),
        direction: 1,
        id: i,
        ref_id,
        ledger_id,
    });

    ledger_details
}

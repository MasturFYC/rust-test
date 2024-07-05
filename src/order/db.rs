use crate::{
    db::DBClient,
    ledger::{CreateLedgerSchema, LedgerDetail},
};
use async_trait::async_trait;
use bigdecimal::{BigDecimal, FromPrimitive};
use chrono::Utc;
use sqlx::{self, Acquire};
use uuid::Uuid;

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
    async fn create_ledger(&self, o: OrderDtos) -> CreateLedgerSchema;
    async fn create_ledger_details(
        &self,
        payment: &BigDecimal,
        total: &BigDecimal,
        modal: BigDecimal,
        ref_id: Option<Uuid>,
        ledger_id: Uuid,
    ) -> Vec<LedgerDetail>;
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
        let mut o: OrderDtos = data.try_into().unwrap();

        let pass = BigDecimal::from_i32(0).unwrap();
        let modal = details
            .iter()
            .fold(pass.to_owned(), |d, t| d + (&t.hpp * &t.qty));
        let total = details.iter().fold(pass.to_owned(), |d, t| {
            d + ((&t.price - &t.discount) * &t.qty)
        });

        let mut conn: sqlx::pool::PoolConnection<sqlx::Postgres> = self.pool.acquire().await?;
        let mut tx: sqlx::Transaction<sqlx::Postgres> = conn.begin().await?;

        o.set_total(&total);
        o.set_due_date();

        let ord = sqlx::query_file_as!(
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

        let order = ord.unwrap();
        let new_id = order.id;

        let payment = order.payment.to_owned();
        let ledger = self.create_ledger(o).await;
        let ledger_details = self
            .create_ledger_details(&payment, &total, modal, Some(new_id), new_id)
            .await;

        let len = details.len();
        let mut i: usize = 0;

        loop {
            if let Some(d) = details.get(i) {
                let _ = sqlx::query_file!(
                    "sql/order-detail-insert.sql",
                    new_id,
                    d.product_id,
                    d.qty,
                    d.direction,
                    d.unit,
                    d.price,
                    d.discount,
                    d.hpp,
                )
                .execute(&mut *tx)
                // .fetch_one(&mut *tx)
                .await?;

                let _ = sqlx::query!(
                    "UPDATE products SET unit_in_stock = (unit_in_stock - $2) WHERE id = $1",
                    d.product_id,
                    d.qty,
                )
                .execute(&mut *tx)
                .await?;

                i = i.checked_add(1).unwrap();
            }

            if i == len {
                break;
            }
        }

        let _ = sqlx::query!(
            r#"INSERT INTO ledgers
                (id, relation_id, name, descriptions, updated_by, is_valid)
                VALUES ($1, $2, $3, $4, $5, $6)"#,
            new_id,
            ledger.relation_id,
            ledger.name,
            ledger.descriptions,
            ledger.updated_by,
            ledger.is_valid
        )
        .execute(&mut *tx)
        .await?;

        // for (_, d) in details.into_iter().enumerate() {
        //     let _ = sqlx::query_file!(
        //         "sql/order-detail-insert.sql",
        //         new_id.to_owned(),
        //         d.product_id.to_owned(),
        //         d.qty.to_owned(),
        //         d.direction.to_owned(),
        //         d.unit.to_owned(),
        //         d.price.to_owned(),
        //         d.discount.to_owned(),
        //         d.hpp.to_owned()
        //     )
        //     .execute(&mut *tx)
        //     // .fetch_one(&mut *tx)
        //     .await?;
        //     let _ = sqlx::query!(
        //         "UPDATE products SET unit_in_stock = (unit_in_stock - $2) WHERE id = $1",
        //         d.product_id.to_owned(),
        //         d.qty.to_owned(),
        //     )
        //     .execute(&mut *tx)
        //     .await?;
        // }

        i = 0;
        let len = ledger_details.len();

        loop {
            let d = ledger_details.get(i).unwrap();

            let _ = sqlx::query!(
                r#"INSERT INTO ledger_details (
                    ledger_id,
                    id,
                    account_id,
                    descriptions,
                    amount,
                    direction,
                    ref_id) VALUES ($1, $2, $3, $4, $5, $6, $7)"#,
                d.ledger_id,
                d.id,
                d.account_id,
                d.descriptions,
                d.amount,
                d.direction,
                d.ref_id,
            )
            .execute(&mut *tx)
            .await?;

            i = i.checked_add(1).unwrap();

            if i == len {
                break;
            }
        }

        let details: Vec<OrderDetail> =
            sqlx::query_file_as!(OrderDetail, "sql/order-detail-get-by-order.sql", new_id)
                .fetch_all(&mut *tx)
                .await?;

        tx.commit().await?;

        Ok((Some(order), details))
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
        let mut o: OrderDtos = data.try_into().unwrap();
        let details: Vec<CreateOrderDetailSchema> = details.try_into().unwrap();
        let uid: Uuid = id.try_into().unwrap();

        let pass = BigDecimal::from_i32(0).unwrap();
        let total = details.iter().fold(pass.to_owned(), |d, t| {
            d + ((&t.price - &t.discount) * &t.qty)
        });
        let hpp = details
            .iter()
            .fold(pass.to_owned(), |d, t| d + (&t.hpp * &t.qty));

        o.set_total(&total);
        o.set_due_date();

        let payment = o.payment.to_owned();
        let ledger_details = self
            .create_ledger_details(&payment, &hpp, total, Some(uid), uid)
            .await;

        let mut conn: sqlx::pool::PoolConnection<sqlx::Postgres> = self.pool.acquire().await?;
        let mut tx: sqlx::Transaction<sqlx::Postgres> = conn.begin().await?;

        let order = sqlx::query_file_as!(
            Order,
            "sql/order-update.sql",
            uid,
            o.order_type.unwrap() as OrderType,
            o.relation_id.to_owned(),
            o.payment_type.unwrap() as PaymentType,
            o.updated_by.to_owned(),
            o.total.into(),
            o.payment.into(),
            o.remain.into(),
            o.invoice_id.to_owned(),
            o.due_at.to_owned(),
            o.created_at.to_owned()
        )
        .fetch_optional(&mut *tx)
        .await?;

        let mut i = 0;
        let len = details.len();

        loop {
            if let Some(d) = details.get(i) {
                match d.mark_as.unwrap() {
                    DetailMark::Delete => {
                        let _ = sqlx::query!(
                                "UPDATE products SET unit_in_stock = (unit_in_stock + $2) WHERE id = $1",
                                d.old_product_id,
                                d.old_qty,
                                )
                            .execute(&mut *tx)
                            .await?;

                        let _ = sqlx::query_file!("sql/order-detail-delete.sql", d.id,)
                            .execute(&mut *tx)
                            .await?;
                    }

                    DetailMark::Update => {
                        let _ = sqlx::query_file!(
                            "sql/order-detail-update.sql",
                            d.id,
                            uid,
                            d.product_id,
                            d.qty,
                            d.direction,
                            d.unit,
                            d.price,
                            d.discount,
                            d.hpp
                        )
                        .execute(&mut *tx)
                        .await?;

                        let _ = sqlx::query!(
                                "UPDATE products SET unit_in_stock = (unit_in_stock + $2) WHERE id = $1",
                                d.old_product_id,
                                d.old_qty
                                )
                            .execute(&mut *tx)
                            .await?;

                        let _ = sqlx::query!(
                                "UPDATE products SET unit_in_stock = (unit_in_stock - $2) WHERE id = $1",
                                d.product_id,
                                d.qty,
                                )
                            .execute(&mut *tx)
                            .await?;
                    }
                    _ => {
                        let _ = sqlx::query_file!(
                            "sql/order-detail-insert.sql",
                            uid,
                            d.product_id,
                            d.qty,
                            d.direction,
                            d.unit,
                            d.price,
                            d.discount,
                            d.hpp,
                        )
                        .execute(&mut *tx)
                        .await?;

                        let _ = sqlx::query!(
                                "UPDATE products SET unit_in_stock = (unit_in_stock - $2) WHERE id = $1",
                                d.product_id,
                                d.qty,
                                )
                            .execute(&mut *tx)
                            .await?;
                    }
                }

                i = i.checked_add(1).unwrap();
            }


            if i == len {
                break;
            }
        }

        let _ = sqlx::query!(
            r#"UPDATE ledgers SET 
                relation_id = $2, 
                descriptions = $3,
                updated_by = $4,
                updated_at = $5
                WHERE id = $1"#,
            uid,
            o.relation_id,
            format!("Order by {}", o.sales_id),
            o.updated_by,
            Utc::now()
        )
        .execute(&mut *tx)
        .await?;

        let _ = sqlx::query!("DELETE FROM ledger_details WHERE ledger_id = $1", uid)
            .execute(&mut *tx)
            .await?;

        i = 0;
        let len = ledger_details.len();

        loop {
            let d = ledger_details.get(i).unwrap();

            let _ = sqlx::query!(
                r#"INSERT INTO ledger_details (
                    ledger_id,
                    id,
                    account_id,
                    descriptions,
                    amount,
                    direction,
                    ref_id) VALUES ($1, $2, $3, $4, $5, $6, $7)"#,
                d.ledger_id,
                d.id,
                d.account_id,
                d.descriptions,
                d.amount,
                d.direction,
                d.ref_id,
            )
            .execute(&mut *tx)
            .await?;

            i = i.checked_add(1).unwrap();

            if i == len {
                break;
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
        let mut conn: sqlx::pool::PoolConnection<sqlx::Postgres> = self.pool.acquire().await?;
        let mut tx: sqlx::Transaction<sqlx::Postgres> = conn.begin().await?;

        let details: Vec<OrderDetail> =
            sqlx::query_file_as!(OrderDetail, "sql/order-detail-get-by-order.sql", id)
                .fetch_all(&mut *tx)
                .await?;

        let mut i = 0;
        let len = details.len();

        loop {
            if let Some(d) = details.get(i) {
                let _ = sqlx::query!(
                    "UPDATE products SET unit_in_stock = (unit_in_stock + $2) WHERE id = $1",
                    d.product_id,
                    d.qty,
                )
                .execute(&mut *tx)
                .await?;

                i = i.checked_add(1).unwrap();
            }

            if i == len {
                break;
            }
        }

        let _ = sqlx::query!("DELETE FROM order_details WHERE order_id = $1", id)
            .execute(&mut *tx)
            .await?;

        //let _ = sqlx::query!("DELETE FROM ledger_details WHERE ledger_id = $1", id)
        //    .execute(&mut *tx)
        //    .await?;

        let _ = sqlx::query!(
            r#"DELETE FROM ledgers WHERE
            id = $1 OR 
            id IN (SELECT ref_id FROM ledger_details WHERE ref_id = $1)"#,
            id
        )
        .execute(&mut *tx)
        .await?;

        let rows_affected: u64 = sqlx::query_file!("sql/order-delete.sql", id)
            .execute(&mut *tx)
            .await
            .unwrap()
            .rows_affected();

        tx.commit().await?;
        Ok(rows_affected)
    }

    async fn order_count(&self) -> Result<Option<i64>, sqlx::Error> {
        let row = sqlx::query_scalar!("SELECT COUNT(*) as total FROM orders")
            .fetch_one(&self.pool)
            .await?;

        Ok(row)
    }

    async fn create_ledger(&self, o: OrderDtos) -> CreateLedgerSchema {
        CreateLedgerSchema {
            descriptions: Some(format!("Order by {}", o.sales_id)),
            is_valid: true,
            name: String::from("ORDER"),
            relation_id: o.relation_id,
            updated_by: o.updated_by,
        }
    }

    async fn create_ledger_details(
        &self,
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
}

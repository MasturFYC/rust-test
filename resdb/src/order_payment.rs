pub mod db {
	use crate::ledger::LedgerUtil;
	use crate::model::LedgerType;
	use async_trait::async_trait;
	use sqlx::{self, pool::PoolConnection, Acquire, Error, Postgres, Transaction};

	use crate::db::DBClient;
	use crate::model::{OrderPayment, OrderPayments};

	#[derive(serde::Deserialize, serde::Serialize, sqlx::FromRow, Clone)]
	struct OrderInfo {
		id: i32,
		customer_id: i16,
		customer_name: String,
		payment: bigdecimal::BigDecimal,
	}
	#[derive(serde::Deserialize, serde::Serialize, sqlx::FromRow, Clone)]
	struct PaymentInfo {
		order_id: i32,
		amount: bigdecimal::BigDecimal,
	}

	#[async_trait]
	pub trait OrderPaymentExt {
		async fn get_order_payment(
			&self,
			id: i32,
		) -> Result<Option<OrderPayments>, Error>;
		async fn get_order_payments(
			&self,
			page: usize,
			limit: usize,
		) -> Result<(Vec<OrderPayments>, i64), Error>;
		async fn order_payment_create<T>(
			&self,
			data: T,
		) -> Result<Option<OrderPayments>, Error>
		where
			T: Into<OrderPayment> + Send;
		async fn order_payment_update<T>(
			&self,
			pid: i32,
			data: T,
		) -> Result<Option<OrderPayments>, Error>
		where
			T: Into<OrderPayment> + Send;
		async fn order_payment_delete(&self, pid: i32) -> Result<u64, Error>;
	}

	#[async_trait]
	impl OrderPaymentExt for DBClient {
		async fn get_order_payment(
			&self,
			id: i32,
		) -> Result<Option<OrderPayments>, Error> {
			let payment = sqlx::query_as!(
				OrderPayments,
                r#"
            SELECT
                order_id, payment_id, amount, updated_by, via_by, descriptions, created_at, updated_at
            FROM
                order_payments
            WHERE
                order_id = $1
            "#,
                id
            )
            .fetch_optional(&self.pool)
            .await?;
			Ok(payment)
		}

		async fn get_order_payments(
			&self,
			page: usize,
			limit: usize,
		) -> Result<(Vec<OrderPayments>, i64), Error> {
			let x: usize = 1;
			let offset: usize = (page - x) * limit;

			let mut cnn: PoolConnection<Postgres> = self.pool.acquire().await?;
			let mut tx: Transaction<Postgres> = cnn.begin().await?;

			let payments = sqlx::query_as!(
				OrderPayments,
				r#"
				SELECT
             	order_id, payment_id, amount, updated_by, via_by, descriptions, created_at, updated_at
         	FROM
            	order_payments
         	ORDER BY
            	order_id, payment_id
         	OFFSET $1
         	LIMIT $2
            "#,
                limit as i64,
                offset as i64
            )
            .fetch_all(&mut *tx)
            .await?;

			let count = sqlx::query_scalar!(
				r#"
            SELECT
                COUNT(*)
            FROM
                order_payments
            "#
			)
			.fetch_one(&mut *tx)
			.await?
			.unwrap_or(0);

			tx.commit().await?;

			Ok((payments, count))
		}

		async fn order_payment_create<T>(
			&self,
			data: T,
		) -> Result<Option<OrderPayments>, Error>
		where
			T: Into<OrderPayment> + Send,
		{
			let op: OrderPayment = data.into();

			let mut cnn: PoolConnection<Postgres> = self.pool.acquire().await?;
			let mut tx: Transaction<Postgres> = cnn.begin().await?;

			let order = sqlx::query_as!(
				OrderInfo,
				r#"
            SELECT
                o.id,
                o.customer_id,
                r.name as customer_name,
                o.payment
            FROM
                orders o INNER JOIN relations r
                ON r.id = o.customer_id
            WHERE
                o.id = $1
            "#,
				op.order_id
			)
			.fetch_one(&mut *tx)
			.await?;

			let query = sqlx::query_as!(
				OrderPayments,
				r#"
            INSERT INTO
                order_payments (
                    order_id,
                    amount,
                    updated_by,
                    via_by,
                    descriptions
                )
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#,
				op.order_id,
				op.amount,
				op.updated_by,
				op.via_by,
				op.descriptions
			)
			.fetch_optional(&mut *tx)
			.await?;

			let payment = query.unwrap();
			let pid = payment.order_id;

			let _ = sqlx::query!(
                r#"
            INSERT INTO
                ledgers (
                    id, relation_id, ledger_type, descriptions, updated_by, is_valid
                )
            VALUES
                ($1, $2, $3, $4, $5, $6)
            "#,
                pid,
                order.customer_id,
                LedgerType::OrderPayment as LedgerType,
                format!(
                    "Titip bayar {}, No order {} lewat ...",
                    order.customer_name, order.id
                ),
                op.updated_by,
                false
            )
            .execute(&mut *tx)
            .await?;

			let (ledger_details, count) =
				LedgerUtil::from_order_payment(&op.amount, pid, pid);

			let mut i: usize = 0;

			loop {
				let detail = ledger_details.get(i).unwrap();

				let _ = sqlx::query!(
					r#"
                INSERT INTO
                    ledger_details(
                        ledger_id,
                        detail_id,
                        account_id,
                        descriptions,
                        amount,
                        direction,
                        ref_id
                    )
                VALUES
                    ($1, $2, $3, $4, $5, $6, $7)
                "#,
					detail.ledger_id,
					detail.detail_id,
					detail.account_id,
					detail.descriptions,
					detail.amount,
					detail.direction,
					detail.ref_id
				)
				.execute(&mut *tx)
				.await?;

				i = i.checked_add(1).unwrap();

				if i == count {
					break;
				}
			}

			let _ = sqlx::query!(
                r#"
            UPDATE
                orders
            SET
                payment = $2,
                remain = total - (dp + $2),
                payment_type = (
                CASE
                    WHEN (dp + $2) > 0 AND (dp + $2) < total THEN 'pending'::payment_enum
                    ELSE 'lunas'::payment_enum
                END
                )
            WHERE
                id = $1
            "#,
                order.id,
                order.payment + op.amount
            )
            .execute(&mut *tx)
            .await?;

			tx.commit().await?;

			Ok(Some(payment))
		}

		async fn order_payment_update<T>(
			&self,
			pid: i32,
			data: T,
		) -> Result<Option<OrderPayments>, Error>
		where
			T: Into<OrderPayment> + Send,
		{
			let op: OrderPayment = data.into();

			let mut cnn: PoolConnection<Postgres> = self.pool.acquire().await?;
			let mut tx: Transaction<Postgres> = cnn.begin().await?;

			let order = sqlx::query_as!(
				OrderInfo,
				r#"
            SELECT
                o.id,
                o.customer_id,
                r.name as customer_name,
                o.payment
            FROM
                orders o INNER JOIN relations r
                ON r.id = o.customer_id
            WHERE
                o.id = $1
            "#,
				op.order_id
			)
			.fetch_one(&mut *tx)
			.await?;

			let old_op = sqlx::query_as!(
				PaymentInfo,
				r#"
            SELECT
                order_id,
                amount
            FROM
                order_payments
            WHERE
                order_id = $1
            "#,
				pid
			)
			.fetch_one(&mut *tx)
			.await?;

			let query = sqlx::query_as!(
				OrderPayments,
				r#"
            UPDATE
                order_payments
            SET
                order_id = $2,
                amount = $3,
                updated_by = $4,
                via_by = $5,
                descriptions = $6,
                updated_at = now()
            WHERE order_id = $1
            RETURNING *
            "#,
				pid,
				op.order_id,
				op.amount,
				op.updated_by,
				op.via_by,
				op.descriptions
			)
			.fetch_optional(&mut *tx)
			.await?;

			let payment = query.unwrap();

			let _ = sqlx::query!(
				r#"
            UPDATE
                ledgers
            SET
                relation_id = $2,
                ledger_type = $3,
                descriptions = $4,
                updated_by = $5,
                is_valid = $6
            WHERE
                id = $1
            "#,
				pid,
				order.customer_id,
				LedgerType::OrderPayment as LedgerType,
				format!(
					"Titip bayar {}, No order {} lewat ...",
					order.customer_name, order.id
				),
				op.updated_by,
				false
			)
			.execute(&mut *tx)
			.await?;

			let _ = sqlx::query!(
				r#"
            DELETE FROM
                ledger_details
            WHERE
                ledger_id = $1
            "#,
				pid
			)
			.execute(&mut *tx)
			.await?;

			let (ledger_details, count) =
				LedgerUtil::from_order_payment(&op.amount, pid, pid);

			let mut i: usize = 0;

			loop {
				let detail = ledger_details.get(i).unwrap();

				let _ = sqlx::query!(
					r#"
                INSERT INTO
                    ledger_details(
                        ledger_id,
                        detail_id,
                        account_id,
                        descriptions,
                        amount,
                        direction,
                        ref_id
                    )
                VALUES
                    ($1, $2, $3, $4, $5, $6, $7)
                "#,
					detail.ledger_id,
					detail.detail_id,
					detail.account_id,
					detail.descriptions,
					detail.amount,
					detail.direction,
					detail.ref_id
				)
				.execute(&mut *tx)
				.await?;

				i = i.checked_add(1).unwrap();

				if i == count {
					break;
				}
			}

			let _ = sqlx::query!(
            r#"
            UPDATE
                orders
            SET
                payment = payment - $2 + $3,
                remain = remain + $2 - $3,
                payment_type = (
                    CASE
                        WHEN (dp - $2 + $3) > 0 AND (dp - $2 + $3) < total THEN 'pending'::payment_enum
                        ELSE 'lunas'::payment_enum
                    END
                    )
                WHERE
                id = $1
            "#,
            order.id,
            old_op.amount,
            op.amount
        )
        .execute(&mut *tx)
        .await?;

			tx.commit().await?;

			Ok(Some(payment))
		}

		/// Hapus **data pembayaran order** dan **data ledger**.
		async fn order_payment_delete(&self, pid: i32) -> Result<u64, Error> {
			let mut cnn: PoolConnection<Postgres> = self.pool.acquire().await?;
			let mut tx: Transaction<Postgres> = cnn.begin().await?;

			let _ = sqlx::query!(
				r#"
            DELETE FROM
                ledgers
            WHERE
                id = $1
            "#,
				pid,
			)
			.execute(&mut *tx)
			.await?;

			let op = sqlx::query_as!(
				PaymentInfo,
				r#"
            DELETE FROM
                order_payments
            WHERE
                order_id = $1
            RETURNING
                order_id, amount
            "#,
				pid,
			)
			.fetch_optional(&mut *tx)
			.await?;

			let affected = if op.is_none() { 0 } else { 1 };
			let payment = op.unwrap();

			let _ = sqlx::query!(
                r#"
            UPDATE
                orders
            SET
                payment = payment - $2,
                remain = remain + $2,
                payment_type = (
                    CASE
                        WHEN (dp - $2) > 0 AND (dp - $2) < total THEN 'pending'::payment_enum
                        ELSE 'lunas'::payment_enum
                    END
                    )
                WHERE
                id = $1
            "#,
                payment.order_id,
                payment.amount
            )
            .execute(&mut *tx)
            .await?;

			tx.commit().await?;

			Ok(affected)
		}
	}
}

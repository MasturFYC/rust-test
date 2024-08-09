pub mod db {

	use async_trait::async_trait;
	use bigdecimal::{BigDecimal, FromPrimitive};
	use chrono::Utc;
	use sqlx::{self, Acquire};
	use i32;

	use crate::{
		db::DBClient,
		ledger::LedgerUtil,
		model::{
			CreateOrderDetailSchema, DetailMark, Order, OrderBuilder, OrderDetail, OrderDtos,
			OrderType, PaymentType, ResponseOrder, LedgerBuilder, LedgerType,
		},
	};

	// use crate::{order_detail::{CreateOrderDetailSchema, MatchTrxResult, OrderDetail}, DBClient};

	#[async_trait]
	pub trait OrderExt {
		async fn get_order(&self, id: i32) -> Result<Option<Order>, sqlx::Error>;
		async fn get_orders(
			&self,
			page: usize,
			limit: usize,
		) -> Result<(Vec<ResponseOrder>, i64), sqlx::Error>;
		async fn order_create<O, T>(
			&self,
			data: O,
			details: T,
		) -> Result<(Option<Order>, Vec<OrderDetail>), sqlx::Error>
		where
			O: Into<OrderDtos> + Send,
			T: Into<Vec<CreateOrderDetailSchema>> + Send;
		async fn order_update<S, O, T>(
			&self,
			id: S,
			data: O,
			details: T,
		) -> Result<(Option<Order>, Vec<OrderDetail>), sqlx::Error>
		where
			S: Into<i32> + Send,
			O: Into<OrderDtos> + Send,
			T: Into<Vec<CreateOrderDetailSchema>> + Send;
		async fn order_delete(&self, id: i32) -> Result<u64, sqlx::Error>;
		// #[allow(dead_code)]
		// async fn order_count(&self) -> Result<Option<i64>, sqlx::Error>;
		// async fn create_ledger(&self, o: OrderDtos) -> CreateLedgerSchema;
		/*
		async fn create_ledger_details(
			&self,
			payment: &BigDecimal,
			total: &BigDecimal,
			modal: BigDecimal,
			ref_id: Option<Uuid>,
			ledger_id: Uuid,
		) -> (Vec<LedgerDetail>, i16);
		*/
	}

	#[async_trait]
	impl OrderExt for DBClient {
		async fn get_order(&self, id: i32) -> Result<Option<Order>, sqlx::Error> {
			let order = sqlx::query_file_as!(Order, "sql/order-get-by-id.sql", id)
				.fetch_optional(&self.pool)
				.await?;

			Ok(order)
		}

		/// call using curl
		/// curl localhost:8000/api/orders \
		/// -H "content-type: application/json" \
		/// -H "Authorization: Bearer $(cat token.txt)"
		async fn get_orders(
			&self,
			page: usize,
			limit: usize,
		) -> Result<(Vec<ResponseOrder>, i64), sqlx::Error> {
			let offset = (page - 1) * limit;

			// acquire pg connection from current pool
			let mut conn = self.pool.acquire().await?;

			// get transaction pool from pg connection
			let mut tx = conn.begin().await?;

			// start transaction
			// get orders data from database
			let orders = sqlx::query_file_as!(
				ResponseOrder,
				"sql/order-get-all.sql",
				limit as i64,
				offset as i64,
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

		async fn order_create<O, T>(
			&self,
			data: O,
			details: T,
		) -> Result<(Option<Order>, Vec<OrderDetail>), sqlx::Error>
		where
			O: Into<OrderDtos> + Send,
			T: Into<Vec<CreateOrderDetailSchema>> + Send,
		{
			let details: Vec<CreateOrderDetailSchema> = details.try_into().unwrap();
			let dtos: OrderDtos = data.try_into().unwrap();

			let o = OrderBuilder::new(
				dtos.order_type.unwrap_or(OrderType::Order),
				dtos.updated_by,
				dtos.total,
				BigDecimal::from(0),
				dtos.is_protected,
				dtos.created_at,
				dtos.invoice_id,
				dtos.customer_id,
				dtos.sales_id,
			)
			.with_dp(dtos.dp)
			.with_due_range(dtos.due_range.unwrap_or(0))
			.build();

			let pass = BigDecimal::from_i32(0).unwrap();
			let hpp = details
				.iter()
				.fold(pass.to_owned(), |d, t| d + (&t.hpp * &t.qty));
			let total = details.iter().fold(pass.to_owned(), |d, t| {
				d + ((&t.price - &t.discount) * &t.qty)
			});

			let mut conn: sqlx::pool::PoolConnection<sqlx::Postgres> = self.pool.acquire().await?;
			let mut tx: sqlx::Transaction<sqlx::Postgres> = conn.begin().await?;

			let ord = sqlx::query_file_as!(
				Order,
				"sql/order-insert.sql",
				o.order_type.unwrap() as OrderType,
				o.customer_id,
				o.sales_id,
				o.payment_type.unwrap() as PaymentType,
				o.updated_by,
				o.total,
				o.dp,
				o.payment,
				o.remain,
				o.invoice_id,
				o.due_at,
				o.is_protected,
				o.created_at
			)
			.fetch_optional(&mut *tx)
			.await?;

			let sql = "SELECT name FROM relations WHERE id = $1";

			let sales = sqlx::query_as::<_, (String,)>(sql)
				.bind(o.sales_id)
				.fetch_one(&mut *tx)
				.await?;
			let customer = sqlx::query_as::<_, (String,)>(sql)
				.bind(o.customer_id)
				.fetch_one(&mut *tx)
				.await?;

			let sales_name = sales.0;
			let customer_name = customer.0;

			let order = ord.unwrap();
			let nid = order.id;

			let led = LedgerBuilder::default()
				.relation_id(o.customer_id)
				.ledger_type(LedgerType::Order)
				.is_valid(true)
				.updated_by(o.updated_by)
				.descriptions(format!("Order {} by {}", customer_name, sales_name))
				.build();
			//self.create_ledger(o).await;
			let len = details.len();
			let mut i = 0;

			loop {
				if let Some(d) = details.get(i) {
					let subtotal = (&d.price - &d.discount) * &d.qty;
					let _ = sqlx::query_file!(
						"sql/order-detail-insert.sql",
						nid,
						d.product_id,
						d.qty,
						d.direction,
						d.unit,
						d.price,
						d.discount,
						d.hpp,
						subtotal,
						(i + 1) as i16
					)
					.execute(&mut *tx)
					.await?;

					let _ = sqlx::query!(
						r#"
                    UPDATE
                        products
                    SET
                        unit_in_stock = (unit_in_stock - $2)
                    WHERE
                        id = $1"#,
						d.product_id,
						d.qty
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
				r#"
            INSERT INTO ledgers
                (id, relation_id, ledger_type, descriptions, updated_by, is_valid)
            VALUES
                ($1, $2, $3, $4, $5, $6)
        "#,
				nid,
				led.relation_id,
				led.ledger_type as LedgerType,
				led.descriptions,
				led.updated_by,
				led.is_valid
			)
			.execute(&mut *tx)
			.await?;

			// for (_, d) in details.into_iter().enumerate() {
			//     let _ = sqlx::query_file!(
			//         "sql/order-detail-insert.sql",
			//         nid.to_owned(),
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
			//        let payment = .to_owned();
			let (ledger_details, len) = LedgerUtil::from_order(&total, &order.dp, &hpp, nid, nid);

			let mut i: usize = 0;
			//        let len = ledger_details.len();

			loop {
				let d = ledger_details.get(i).unwrap();

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
					d.ledger_id,
					d.detail_id,
					d.account_id,
					d.descriptions,
					d.amount,
					d.direction,
					d.ref_id
				)
				.execute(&mut *tx)
				.await?;

				i = i.checked_add(1).unwrap();

				if i == len {
					break;
				}
			}

			let details: Vec<OrderDetail> =
				sqlx::query_file_as!(OrderDetail, "sql/order-detail-get-by-order.sql", nid)
					.fetch_all(&mut *tx)
					.await?;

			tx.commit().await?;

			Ok((Some(order), details))
		}

		async fn order_update<S, O, T>(
			&self,
			id: S,
			data: O,
			details: T,
		) -> Result<(Option<Order>, Vec<OrderDetail>), sqlx::Error>
		where
			O: Into<OrderDtos> + Send,
			T: Into<Vec<CreateOrderDetailSchema>> + Send,
			S: Into<i32> + Send,
		{
			let uid: i32 = id.try_into().unwrap();
			let dtos: OrderDtos = data.try_into().unwrap();
			let details: Vec<CreateOrderDetailSchema> = details.try_into().unwrap();

			let pass = BigDecimal::from(0);
			let total = details.iter().fold(pass.to_owned(), |d, t| {
				d + ((&t.price - &t.discount) * &t.qty)
			});
			let hpp = details
				.iter()
				.fold(pass.to_owned(), |d, t| d + (&t.hpp * &t.qty));

			let mut conn: sqlx::pool::PoolConnection<sqlx::Postgres> = self.pool.acquire().await?;
			let mut tx: sqlx::Transaction<sqlx::Postgres> = conn.begin().await?;

			let test = sqlx::query_scalar(
				r#"
                SELECT
                    SUM(amount)
                FROM
                    order_payments
                WHERE
                    id = $1
            "#,
			)
			.bind(uid)
			.fetch_one(&mut *tx)
			.await?;

			let payment = Some(test).unwrap_or(BigDecimal::from(0));

			let o = OrderBuilder::new(
				dtos.order_type.unwrap_or_default(),
				dtos.updated_by,
				dtos.total,
				payment.to_owned(),
				dtos.is_protected,
				dtos.created_at,
				dtos.invoice_id,
				dtos.customer_id,
				dtos.sales_id,
			)
			.with_dp(dtos.dp)
			.with_due_range(dtos.due_range.unwrap_or(0))
			.build();

			let order = sqlx::query_file_as!(
				Order,
				"sql/order-update.sql",
				uid,
				o.order_type.unwrap() as OrderType,
				o.customer_id,
				o.sales_id,
				o.payment_type.unwrap() as PaymentType,
				o.updated_by,
				o.total,
				o.dp,
				o.payment,
				o.remain,
				o.invoice_id,
				o.due_at,
				o.is_protected,
				o.created_at
			)
			.fetch_optional(&mut *tx)
			.await?;

			let mut i = 0;
			let len = details.len();

			loop {
				if let Some(d) = details.get(i) {
					let subtotal = (&d.price - &d.discount) * &d.qty;
					match d.mark_as.unwrap() {
						DetailMark::Delete => {
							let _ = sqlx::query!(
								r#"
                            UPDATE
                                products
                            SET
                                unit_in_stock = (unit_in_stock + $2)
                            WHERE
                                id = $1
                            "#,
								d.old_product_id,
								d.old_qty
							)
							.execute(&mut *tx)
							.await?;

							let _ = sqlx::query_file!("sql/order-detail-delete.sql", d.detail_id)
								.execute(&mut *tx)
								.await?;
						}

						DetailMark::Update => {
							let _ = sqlx::query_file!(
								"sql/order-detail-update.sql",
								d.detail_id,
								uid,
								d.product_id,
								d.qty,
								d.direction,
								d.unit,
								d.price,
								d.discount,
								d.hpp,
								subtotal
							)
							.execute(&mut *tx)
							.await?;

							let _ = sqlx::query!(
								r#"
                            UPDATE
                                products
                            SET
                                unit_in_stock = (unit_in_stock + $2)
                            WHERE
                                id = $1
                            "#,
								d.old_product_id,
								d.old_qty
							)
							.execute(&mut *tx)
							.await?;

							let _ = sqlx::query!(
								r#"
                            UPDATE
                                products
                            SET
                                unit_in_stock = (unit_in_stock - $2)
                            WHERE
                                id = $1
                            "#,
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
								subtotal,
								(i + 1) as i16
							)
							.execute(&mut *tx)
							.await?;

							let _ = sqlx::query!(
								r#"
                            UPDATE
                                products
                            SET
                                unit_in_stock = (unit_in_stock - $2)
                            WHERE
                                id = $1"#,
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
				r#"
            UPDATE
                ledgers
            SET
                relation_id = $2,
                descriptions = $3,
                updated_by = $4,
                updated_at = $5
            WHERE
                id = $1
            "#,
				uid,
				o.customer_id,
				format!("Order by {}", o.sales_id),
				o.updated_by,
				Utc::now()
			)
			.execute(&mut *tx)
			.await?;

			let _ = sqlx::query!(
				r#"
            DELETE
            FROM
                ledger_details
            WHERE
                ledger_id = $1
            "#,
				uid
			)
			.execute(&mut *tx)
			.await?;

			let (ledger_details, len) = LedgerUtil::from_order(&total, &o.dp, &hpp, uid, uid);

			let mut i = 0;

			loop {
				let d = ledger_details.get(i).unwrap();

				let _ = sqlx::query!(
					r#"
                INSERT INTO
                    ledger_details (
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
					d.ledger_id,
					d.detail_id,
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

		async fn order_delete(&self, id: i32) -> Result<u64, sqlx::Error> {
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
						r#"
                    UPDATE
                        products
                    SET
                        unit_in_stock = (unit_in_stock + $2)
                    WHERE
                        id = $1
                    "#,
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
				r#"
            DELETE FROM
                order_details
            WHERE
                order_id = $1
            "#,
				id
			)
			.execute(&mut *tx)
			.await?;

			let _ = sqlx::query!(
				r#"
                DELETE FROM
                    ledgers
                WHERE
                    id = $1 OR
                    id IN (SELECT id FROM order_payments WHERE order_id = $1)
                    -- id IN (SELECT ref_id FROM ledger_details WHERE ref_id = $1) OR
                    "#,
				id,
			)
			.execute(&mut *tx)
			.await?;

			let rows_affected: u64 = sqlx::query_file!("sql/order-delete.sql", id,)
				.execute(&mut *tx)
				.await
				.unwrap()
				.rows_affected();

			tx.commit().await?;
			Ok(rows_affected)
		}

		// async fn order_count(&self) -> Result<Option<i64>, sqlx::Error> {
		//     let row = sqlx::query_scalar!(
		//         r#"
		//         SELECT
		//             COUNT(*) as total
		//         FROM
		//             orders
		//         "#
		//     )
		//     .fetch_one(&self.pool)
		//     .await?;

		//     Ok(row)
		// }
	}
}

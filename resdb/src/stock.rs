pub mod model {
	use serde::{Deserialize, Serialize};
	use bigdecimal::BigDecimal;
	use chrono::prelude::*;
	use validator::Validate;

	use crate::model::PaymentType;

	#[derive(Validate, Debug, Deserialize, Serialize, Clone)]
	pub struct RequestQueryStock {
		pub stock: Stock,
		pub details: Vec<StockDetail>,
	}
	#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
	pub struct Stock {
		#[serde(rename = "supplierId")]
		pub customer_id: i16,
		#[serde(rename = "warehouseId")]
		pub sales_id: i16,
		#[serde(rename = "paymentType")]
		pub payment_type: PaymentType,
		#[serde(rename = "updatedBy")]
		pub updated_by: String,
		pub total: BigDecimal,
		pub dp: BigDecimal,
		pub payment: BigDecimal,
		pub remain: BigDecimal,
		#[serde(rename = "invoiceId")]
		pub invoice_id: String,
		#[serde(rename = "dueRange")]
		pub due_range: Option<u64>,
		#[serde(rename = "dueAt")]
		pub due_at: Option<DateTime<Utc>>,
		#[serde(rename = "createdAt")]
		pub created_at: Option<DateTime<Utc>>,
		#[serde(rename = "updatedAt")]
		pub updated_at: Option<DateTime<Utc>>,
		#[serde(
			rename = "supplierName",
			skip_serializing_if = "Option::is_none"
		)]
		pub supplier_name: Option<String>,
		#[serde(
			rename = "warehouseName",
			skip_serializing_if = "Option::is_none"
		)]
		pub warehouse_name: Option<String>,
		#[serde(
			rename = "isProtected",
			skip_serializing_if = "Option::is_none"
		)]
		pub is_protected: Option<bool>,
	}

	#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
	pub struct StockInsertedId {
		pub id: i32,
	}

	#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
	pub struct Stocks {
		pub id: i32,
		#[serde(rename = "supplierId")]
		pub customer_id: i16,
		#[serde(rename = "warehouseId")]
		pub sales_id: i16,
		#[serde(rename = "paymentType")]
		pub payment_type: PaymentType,
		#[serde(rename = "updatedBy")]
		pub updated_by: String,
		pub total: BigDecimal,
		pub dp: BigDecimal,
		pub payment: BigDecimal,
		pub remain: BigDecimal,
		#[serde(
			rename = "supplierName",
			skip_serializing_if = "Option::is_none"
		)]
		pub supplier_name: Option<String>,
		#[serde(
			rename = "warehouseName",
			skip_serializing_if = "Option::is_none"
		)]
		pub warehouse_name: Option<String>,
		#[serde(rename = "invoiceId")]
		pub invoice_id: Option<String>,
		#[serde(rename = "dueAt")]
		pub due_at: Option<DateTime<Utc>>,
		#[serde(rename = "createdAt")]
		pub created_at: Option<DateTime<Utc>>,
		#[serde(rename = "updatedAt")]
		pub updated_at: Option<DateTime<Utc>>,
		#[serde(
			rename = "isProtected",
			skip_serializing_if = "Option::is_none"
		)]
		pub is_protected: Option<bool>,
	}

	#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, Clone)]
	pub struct ProductQuantity {
		pub product_id: i16,
		pub gudang_id: i16,
		pub qty: BigDecimal,
	}

	#[derive(
		Validate, Default, Debug, Deserialize, sqlx::FromRow, Serialize, Clone,
	)]
	pub struct StockDetail {
		#[serde(rename = "stockId")]
		pub order_id: i32,
		#[serde(rename = "id")]
		pub detail_id: i16,
		#[serde(rename = "productId")]
		pub product_id: i16,
		#[serde(rename = "gudangId")]
		pub gudang_id: i16,
		pub qty: BigDecimal,
		pub direction: i16,
		pub unit: String,
		pub price: BigDecimal,
		pub discount: BigDecimal,
		pub hpp: BigDecimal,
		#[serde(rename = "createdAt")]
		pub created_at: Option<DateTime<Utc>>,
		#[serde(rename = "updatedAt")]
		pub updated_at: Option<DateTime<Utc>>,
		pub subtotal: BigDecimal,
	}
	#[derive(
		Validate, Default, Debug, Deserialize, sqlx::FromRow, Serialize, Clone,
	)]
	pub struct StockDetails {
		#[serde(rename = "stockId")]
		pub order_id: i32,
		#[serde(rename = "id")]
		pub detail_id: i16,
		#[serde(rename = "productId")]
		pub product_id: i16,
		#[serde(rename = "gudangId")]
		pub gudang_id: i16,
		#[serde(rename = "gudangName")]
		pub gudang_name: String,
		pub qty: BigDecimal,
		pub direction: i16,
		pub unit: String,
		pub price: BigDecimal,
		pub discount: BigDecimal,
		pub hpp: BigDecimal,
		#[serde(rename = "createdAt")]
		pub created_at: Option<DateTime<Utc>>,
		#[serde(rename = "updatedAt")]
		pub updated_at: Option<DateTime<Utc>>,
		pub subtotal: BigDecimal,
		pub name: String,
		pub barcode: String,
		#[serde(rename = "oldQty")]
		pub old_qty: BigDecimal,
		#[serde(rename = "oldGudangId")]
		pub old_gudang_id: i16,
	}

	#[derive(Serialize, Deserialize, Validate, Clone)]
	pub struct RequestStock {
		#[validate(range(min = 1))]
		pub page: Option<usize>,
		#[validate(range(min = 1, max = 50))]
		pub limit: Option<usize>,
		pub opt: Option<usize>,
		pub supid: Option<i16>,
		pub wareid: Option<i16>,
		pub txt: Option<String>,
	}
}

pub mod db {

	use async_trait::async_trait;
	use bigdecimal::{BigDecimal, FromPrimitive};
	use chrono::Utc;
	use sqlx::{self, Acquire};
	use i32;

	use crate::{
		db::DBClient,
		ledger::LedgerUtil,
		model::{OrderBuilder, OrderType, LedgerBuilder, LedgerType},
	};

	use super::model::{
		ProductQuantity, RequestStock, Stock, StockDetail, StockDetails,
		StockInsertedId, Stocks,
	};
	use crate::model::PaymentType;

	#[async_trait]
	pub trait StockExt {
		async fn get_stock(
			&self,
			id: i32,
		) -> Result<Option<Stocks>, sqlx::Error>;
		async fn get_stock_with_details(
			&self,
			id: i32,
		) -> Result<(Option<Stocks>, Vec<StockDetails>), sqlx::Error>;
		async fn get_stocks(
			&self,
			opts: RequestStock,
		) -> Result<(Vec<Stocks>, i64), sqlx::Error>;
		async fn stock_create<O, T>(
			&self,
			data: O,
			details: T,
		) -> Result<(i32, usize), sqlx::Error>
		where
			O: Into<Stock> + Send,
			T: Into<Vec<StockDetail>> + Send;
		async fn stock_update<S, O, T>(
			&self,
			id: S,
			data: O,
			details: T,
		) -> Result<(i32, usize), sqlx::Error>
		where
			S: Into<i32> + Send,
			O: Into<Stock> + Send,
			T: Into<Vec<StockDetail>> + Send;
		async fn stock_update_only<S, O>(
			&self,
			id: S,
			data: O,
		) -> Result<i32, sqlx::Error>
		where
			S: Into<i32> + Send,
			O: Into<Stock> + Send;
		async fn stock_delete(&self, ids: Vec<i32>)
			-> Result<u64, sqlx::Error>;
	}

	#[async_trait]
	impl StockExt for DBClient {
		async fn get_stock(
			&self,
			id: i32,
		) -> Result<Option<Stocks>, sqlx::Error> {
			let stock =
				sqlx::query_file_as!(Stocks, "sql/stock-get-by-id.sql", id)
					.fetch_optional(&self.pool)
					.await?;

			Ok(stock)
		}
		async fn get_stock_with_details(
			&self,
			id: i32,
		) -> Result<(Option<Stocks>, Vec<StockDetails>), sqlx::Error> {
			let mut conn = self.pool.acquire().await?;
			let mut tx = conn.begin().await?;

			let stock =
				sqlx::query_file_as!(Stocks, "sql/stock-get-by-id.sql", id)
					.fetch_optional(&mut *tx)
					.await?;
			let details = sqlx::query_file_as!(
				StockDetails,
				"sql/order-detail-get-by-order-2.sql",
				id
			)
			.fetch_all(&mut *tx)
			.await?;

			tx.commit().await?;

			Ok((stock, details))
		}
		async fn get_stocks(
			&self,
			opts: RequestStock,
		) -> Result<(Vec<Stocks>, i64), sqlx::Error> {
			let limit = opts.limit.unwrap_or(10);
			let page = opts.page.unwrap_or(1);
			let offset = (page - 1) * limit;
			// acquire pg connection from current pool
			let mut conn = self.pool.acquire().await?;
			// get transaction pool from pg connection
			let mut tx = conn.begin().await?;
			// start transaction
			// get orders data from database
			let opt = opts.opt.unwrap_or(0);
			let stocks = match opt {
				1 => {
					let txt = opts.txt.unwrap_or("".to_string()).to_lowercase();
					let data = sqlx::query_file_as!(
						Stocks,
						"sql/stock-get-all-by-search.sql",
						txt,
						limit as i64,
						offset as i64
					)
					.fetch_all(&mut *tx)
					.await?;
					let row = sqlx::query_scalar!(
					r#"SELECT COUNT(*) FROM	orders AS o 
                    INNER JOIN relations AS c ON c.id = o.customer_id
   					INNER JOIN relations AS s ON s.id = o.sales_id
   					WHERE o.order_type = 'stock'::order_enum
                    AND POSITION($1 IN (o.id::TEXT||' '||LOWER(c.name||' '||s.name||' '||COALESCE(o.invoice_id,' ')))) > 0
						"#, txt)
						.fetch_one(&mut *tx)
						.await?;

					(data, row)
				}
				2 => {
					let supid = opts.supid.unwrap_or(0);
					let data = sqlx::query_file_as!(
						Stocks,
						"sql/stock-get-all-by-supplier.sql",
						supid,
						limit as i64,
						offset as i64
					)
					.fetch_all(&mut *tx)
					.await?;
					let row = sqlx::query_scalar!(
						r#"
						SELECT
							COUNT(*)
						FROM
							orders AS o
						WHERE
							o.order_type = 'stock'::order_enum
							AND o.customer_id = $1
							"#,
						supid
					)
					.fetch_one(&mut *tx)
					.await?;
					(data, row)
				}
				3 => {
					let wareid = opts.wareid.unwrap_or(0);
					let data = sqlx::query_file_as!(
						Stocks,
						"sql/stock-get-all-by-warehouse.sql",
						wareid,
						limit as i64,
						offset as i64
					)
					.fetch_all(&mut *tx)
					.await?;
					let row = sqlx::query_scalar!(
						r#"
						SELECT
							COUNT(*)
						FROM
							orders AS o
						WHERE
							o.order_type = 'stock'::order_enum
							AND o.sales_id = $1
							"#,
						wareid
					)
					.fetch_one(&mut *tx)
					.await?;
					(data, row)
				}
				_ => {
					let data = sqlx::query_file_as!(
						Stocks,
						"sql/stock-get-all.sql",
						limit as i64,
						offset as i64,
					)
					.fetch_all(&mut *tx)
					.await?;
					let row = sqlx::query_file_scalar!("sql/stock-count.sql")
						.fetch_one(&mut *tx)
						.await?;
					(data, row)
				}
			};

			// start transacrion
			// get total record of stocks

			// finish transaction
			tx.commit().await?;

			// return result to caller
			Ok((stocks.0, stocks.1.unwrap_or(0)))
		}

		async fn stock_create<O, T>(
			&self,
			data: O,
			details: T,
		) -> Result<(i32, usize), sqlx::Error>
		where
			O: Into<Stock> + Send,
			T: Into<Vec<StockDetail>> + Send,
		{
			let details: Vec<StockDetail> = details.try_into().unwrap();
			let dto: Stock = data.try_into().unwrap();
			let o = OrderBuilder::new(
				OrderType::Stock,
				dto.updated_by,
				dto.total,
				BigDecimal::from(0),
				true,
				dto.created_at,
				Some(dto.invoice_id),
				dto.customer_id,
				dto.sales_id,
			)
			.with_dp(dto.dp)
			.with_due_range(dto.due_range.unwrap_or(0))
			.build();
			let pass = BigDecimal::from_i32(0).unwrap();
			let total = details.iter().fold(pass.to_owned(), |d, t| {
				d + ((&t.price - &t.discount) * &t.qty)
			});
			let mut conn: sqlx::pool::PoolConnection<sqlx::Postgres> =
				self.pool.acquire().await?;
			let mut tx: sqlx::Transaction<sqlx::Postgres> =
				conn.begin().await?;
			let new_stock = sqlx::query_file_as!(
				StockInsertedId,
				"sql/stock-insert.sql",
				o.customer_id,
				o.sales_id,
				o.payment_type.unwrap() as PaymentType,
				o.updated_by.to_owned(),
				o.total,
				o.dp,
				o.payment,
				o.remain,
				o.invoice_id,
				o.due_at,
				o.created_at
			)
			.fetch_optional(&mut *tx)
			.await?;
			let stock = new_stock.unwrap();
			let pid = stock.id;

			let led = LedgerBuilder::default()
				.relation_id(o.customer_id)
				.ledger_type(LedgerType::Stock)
				.is_valid(true)
				.updated_by(o.updated_by.to_owned())
				.descriptions(format!(
					"Stock {} by {}",
					dto.supplier_name.to_owned().unwrap_or("".to_string()),
					dto.warehouse_name.to_owned().unwrap_or("".to_string())
				))
				.build();
			//self.create_ledger(o).await;
			let detail_len: usize = details.len();
			let mut i: usize = 0;
			loop {
				if let Some(d) = details.get(i) {
					let subtotal = (&d.price - &d.discount) * &d.qty;
					let _ = sqlx::query_file!(
						"sql/order-detail-insert.sql",
						pid,
						d.product_id,
						d.qty,
						d.direction,
						d.unit,
						d.price,
						d.discount,
						d.hpp,
						subtotal,
						(i + 1) as i16,
						d.gudang_id
					)
					.execute(&mut *tx)
					.await?;

					let _ = sqlx::query!(
						r#"
                    UPDATE
                        stocks
                    SET
                        qty = (qty + $3)
                    WHERE
                        product_id = $1 AND gudang_id = $2
                        "#,
						d.product_id,
						d.gudang_id,
						d.qty
					)
					.execute(&mut *tx)
					.await?;

					i = i.checked_add(1).unwrap();
				}

				if i == detail_len {
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
				pid,
				led.relation_id,
				LedgerType::Stock as _,
				led.descriptions,
				led.updated_by,
				led.is_valid
			)
			.execute(&mut *tx)
			.await?;
			let (ledger_details, len) =
				LedgerUtil::from_stock(&total, &o.dp, pid, pid);

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

			// let details: Vec<StockDetails> =
			// 	sqlx::query_file_as!(StockDetails, "sql/order-detail-get-by-order-2.sql", pid)
			// 		.fetch_all(&mut *tx)
			// 		.await?;

			tx.commit().await?;

			Ok((pid, detail_len))
		}

		async fn stock_update<S, O, T>(
			&self,
			id: S,
			data: O,
			details: T,
		) -> Result<(i32, usize), sqlx::Error>
		where
			O: Into<Stock> + Send,
			T: Into<Vec<StockDetail>> + Send,
			S: Into<i32> + Send,
		{
			let pid: i32 = id.into(); //.unwrap();
			let dto: Stock = data.into(); //.unwrap();
			let details: Vec<StockDetail> = details.into(); //.try_into().unwrap();

			let pass = BigDecimal::from(0);
			let total = details.iter().fold(pass.to_owned(), |d, t| {
				d + ((&t.price - &t.discount) * &t.qty)
			});

			let mut conn: sqlx::pool::PoolConnection<sqlx::Postgres> =
				self.pool.acquire().await?;
			let mut tx: sqlx::Transaction<sqlx::Postgres> =
				conn.begin().await?;

			let test = sqlx::query_scalar(
				r#"
                SELECT
                    COALESCE(SUM(amount), 0)
                FROM
                    order_payments
                WHERE
                   order_id = $1
            "#,
			)
			.bind(pid)
			.fetch_optional(&mut *tx)
			.await?;

			let payment = test.unwrap_or(BigDecimal::from(0));

			let o = OrderBuilder::new(
				OrderType::Stock,
				dto.updated_by,
				total.to_owned(),
				payment.to_owned(),
				true,
				dto.created_at,
				Some(dto.invoice_id),
				dto.customer_id,
				dto.sales_id,
			)
			.with_dp(dto.dp)
			.with_due_range(dto.due_range.unwrap_or(0))
			.build();

			let old_details = sqlx::query_as!(
				ProductQuantity,
				"SELECT product_id, gudang_id, qty FROM order_details WHERE order_id = $1",
				pid
			)
			.fetch_all(&mut *tx)
			.await?;

			let mut i: usize = 0;
			let len = old_details.len();

			loop {
				if let Some(d) = old_details.get(i) {
					let _ = sqlx::query!(
						r#"
                        UPDATE  stocks
                        SET     qty = (qty - $3)
                        WHERE   product_id = $1 AND gudang_id = $2
                        "#,
						d.product_id,
						d.gudang_id,
						d.qty
					)
					.execute(&mut *tx)
					.await?;
				}

				i = i.checked_add(1).unwrap();

				if i == len {
					break;
				}
			}

			let _ = sqlx::query!(
				"DELETE FROM order_details WHERE order_id = $1",
				pid
			)
			.execute(&mut *tx)
			.await?;

			let _ = sqlx::query_file!(
				"sql/stock-update.sql",
				pid,
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
				o.created_at
			)
			.execute(&mut *tx)
			.await?;

			let mut i = 0;
			let detail_len = details.len();

			// print!("\n\nTOTAL NEW DETAILS: {}\n\n", detail_len);

			loop {
				if let Some(d) = details.get(i) {
					let subtotal = (&d.price - &d.discount) * &d.qty;
					let _ = sqlx::query!(
						r#"
					UPDATE
					    stocks
					SET
						qty = (qty + $3)
					WHERE
						product_id = $1 AND gudang_id = $2
					"#,
						d.product_id,
						d.gudang_id,
						d.qty
					)
					.execute(&mut *tx)
					.await?;

					// let xx = test.rows_affected();

					let _ = sqlx::query_file!(
						"sql/order-detail-insert.sql",
						pid,
						d.product_id,
						d.qty,
						d.direction,
						d.unit,
						d.price,
						d.discount,
						d.hpp,
						subtotal,
						(i + 1) as i16,
						d.gudang_id
					)
					.execute(&mut *tx)
					.await?;

					i = i.checked_add(1).unwrap();

					if i == detail_len {
						break;
					}
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
				pid,
				o.customer_id,
				format!(
					"Stock {} by {}",
					dto.supplier_name.to_owned().unwrap_or("".to_string()),
					dto.warehouse_name.to_owned().unwrap_or("".to_string())
				),
				o.updated_by.to_owned(),
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
				pid
			)
			.execute(&mut *tx)
			.await?;

			let (ledger_details, len) =
				LedgerUtil::from_stock(&total, &o.dp, pid, pid);

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

			// let details: Vec<StockDetails> =
			// 	sqlx::query_file_as!(StockDetails, "sql/order-detail-get-by-order-2.sql", pid)
			// 		.fetch_all(&mut *tx)
			// 		.await?;

			tx.commit().await?;

			Ok((pid, detail_len))
		}

		async fn stock_update_only<S, O>(
			&self,
			id: S,
			data: O,
		) -> Result<i32, sqlx::Error>
		where
			S: Into<i32> + Send,
			O: Into<Stock> + Send,
		{
			let pid: i32 = id.try_into().unwrap();
			let dto: Stock = data.try_into().unwrap();

			let total = dto.total.to_owned();

			let mut conn: sqlx::pool::PoolConnection<sqlx::Postgres> =
				self.pool.acquire().await?;
			let mut tx: sqlx::Transaction<sqlx::Postgres> =
				conn.begin().await?;

			let test = sqlx::query_scalar(
				r#"
                SELECT
                    COALESCE(SUM(amount), 0)
                FROM
                    order_payments
                WHERE
                   order_id = $1
            "#,
			)
			.bind(pid)
			.fetch_optional(&mut *tx)
			.await?;

			let payment = test.unwrap_or(BigDecimal::from(0));

			let o = OrderBuilder::new(
				OrderType::Stock,
				dto.updated_by,
				total.to_owned(),
				payment.to_owned(),
				true,
				dto.created_at,
				Some(dto.invoice_id),
				dto.customer_id,
				dto.sales_id,
			)
			.with_dp(dto.dp)
			.with_due_range(dto.due_range.unwrap_or(0))
			.build();

			let _ = sqlx::query_file!(
				"sql/stock-update.sql",
				pid,
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
				o.created_at
			)
			.execute(&mut *tx)
			.await?;

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
				pid,
				o.customer_id,
				format!(
					"Stock {} by {}",
					dto.supplier_name.to_owned().unwrap_or("".to_string()),
					dto.warehouse_name.to_owned().unwrap_or("".to_string())
				),
				o.updated_by.to_owned(),
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
				pid
			)
			.execute(&mut *tx)
			.await?;

			let (ledger_details, len) =
				LedgerUtil::from_stock(&total, &o.dp, pid, pid);

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

			tx.commit().await?;

			Ok(pid)
		}

		async fn stock_delete(
			&self,
			ids: Vec<i32>,
		) -> Result<u64, sqlx::Error> {
			let mut conn: sqlx::pool::PoolConnection<sqlx::Postgres> =
				self.pool.acquire().await?;
			let mut tx: sqlx::Transaction<sqlx::Postgres> =
				conn.begin().await?;

			let details = sqlx::query_as!(
				ProductQuantity,
				r#"
                SELECT 
                    product_id, gudang_id, qty 
                FROM 
                    order_details 
                WHERE 
                    order_id IN (SELECT unnest($1::INT[]))"#,
				&ids[..]
			)
			.fetch_all(&mut *tx)
			.await?;

			let mut i = 0;
			let len = details.len();

			loop {
				if let Some(d) = details.get(i) {
					let _ = sqlx::query!(
						r#"
                    UPDATE
                       stocks 
                    SET
                        qty = (qty - $3)
                    WHERE
                        product_id = $1 AND gudang_id = $2
                    "#,
						d.product_id,
						d.gudang_id,
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
                order_id IN (SELECT unnest($1::int[]))
            "#,
				&ids[..]
			)
			.execute(&mut *tx)
			.await?;

			let _ = sqlx::query!(
				r#"
                DELETE FROM
                    ledgers
                WHERE
                    id IN (SELECT unnest($1::int[])) OR
                    id IN (SELECT id FROM order_payments
				WHERE order_id IN (SELECT unnest($1::int[])))
                -- id IN (SELECT ref_id FROM ledger_details WHERE ref_id = $1) OR
                    "#,
				&ids[..],
			)
			.execute(&mut *tx)
			.await?;

			let rows_affected: u64 = sqlx::query!(
				r#"
				DELETE FROM orders
				WHERE id IN (SELECT unnest($1::int[]))
				"#,
				&ids[..]
			)
			.execute(&mut *tx)
			.await
			.unwrap()
			.rows_affected();

			tx.commit().await?;

			Ok(rows_affected)
		}
	}
}
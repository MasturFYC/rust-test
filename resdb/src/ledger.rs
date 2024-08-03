use bigdecimal::BigDecimal;

use crate::model::{
	Coa, LedgerBuilder, LedgerDetail, LedgerDetailBuilder,
	LedgerSchema, LedgerType,
};

// #[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq)]
// #[serde(rename_all = "snake_case")]

impl LedgerDetailBuilder {
	pub fn with_ledger_id<T: Into<i32>>(
		mut self,
		value: T,
	) -> LedgerDetailBuilder {
		self.ledger_id = Some(value.into());
		self
	}
	pub fn with_detail_id<T: Into<i16>>(mut self, value: T) -> LedgerDetailBuilder
	{
		// let mi: MixedInts = Into::into(value);
		self.detail_id = Some(value.into());
		self
	}
	pub fn with_account_id<T: Into<i16>>(
		mut self,
		value: T,
	) -> LedgerDetailBuilder {
		self.account_id = Some(value.into());
		self
	}
	pub fn with_descriptions<T: Into<String>>(
		mut self,
		value: T,
	) -> LedgerDetailBuilder {
		self.descriptions = Some(value.into());
		self
	}
	pub fn with_amount<T: Into<BigDecimal>>(
		mut self,
		value: T,
	) -> LedgerDetailBuilder {
		self.amount = Some(value.into());
		self
	}

	pub fn with_direction<T: Into<i16>>(mut self, value: T) -> LedgerDetailBuilder
	// where
	// 	T: Direction,
	{
		// let v = value.into().into();

		self.direction = Some(value.into());
		self
	}
	pub fn with_ref_id<T: Into<i32>>(
		mut self,
		value: T,
	) -> LedgerDetailBuilder {
		self.ref_id = Some(value.into());
		self
	}
	pub fn build(&self) -> LedgerDetail {
		LedgerDetail {
			ledger_id: self.ledger_id.expect("ledger_id not define"),
			detail_id: self.detail_id.expect("detail_id not define"),
			account_id: self.account_id.expect("account_id not define"),
			descriptions: self.descriptions.to_owned(),
			amount: self
				.amount
				.to_owned()
				.unwrap_or(bigdecimal::BigDecimal::from(0)), //.to_owned(),// .expect("amount not define"),
			direction: self.direction.expect("direction must be 1 o r -1"),
			ref_id: self.ref_id,
		}
	}
}

impl LedgerBuilder {
	pub fn relation_id<T: Into<i16>>(mut self, value: T) -> LedgerBuilder {
		self.relation_id = Some(value.into());
		self
	}
	pub fn ledger_type<T: Into<LedgerType>>(
		mut self,
		value: T,
	) -> LedgerBuilder {
		self.ledger_type = Some(value.into());
		self
	}
	pub fn is_valid<T: Into<bool>>(mut self, value: T) -> LedgerBuilder {
		self.is_valid = Some(value.into());
		self
	}
	pub fn updated_by<T: Into<String>>(mut self, value: T) -> LedgerBuilder {
		self.updated_by = Some(value.into());
		self
	}
	pub fn descriptions<T: Into<String>>(mut self, value: T) -> LedgerBuilder {
		self.descriptions = Some(value.into());
		self
	}

	pub fn build(&self) -> LedgerSchema {
		LedgerSchema {
			relation_id: self.relation_id.expect("relation_id not initialize"),
			ledger_type: self.ledger_type.expect("ledger_type not define"),
			is_valid: self.is_valid.unwrap_or(false),
			updated_by: self.updated_by.to_owned().expect("updater not define"),
			descriptions: self.descriptions.to_owned(),
		}
	}
}

pub struct LedgerUtil {}
impl LedgerUtil {
	/// ## Ledger untuk transaksi order
	/// `(-) 106 - Persediaan barang` -> **hpp**
	///
	/// `(+) 521 - Biaya beli barang` -> **hpp**
	///
	/// `(-) 421 - Penjualan barang` -> **total**
	///
	/// Tidak digunakan jika tidak ada pembayaran sama sekali dan diganti dengan
	/// *`111 - Piutang barang`*.
	///
	/// `(+) 101 - Kas` -> **dp**
	///
	/// Jika pembayaran tidak lunas atau tidak ada pembayaran sama sekali.
	/// Catat sisa pembayaran ke dalam piutang,
	/// data diambil dari total order dikurangi dp.
	/// Tidak digunakan jika *`dp`* lebih besar atau sama dengan *`total`*
	///
	/// `(+) 111 - Piutang barang` -> (**total - dp**)
	///
	/// **`ref_id`** order id
	///
	/// **`ledger_id`** biasanya sama dengan order id
	///    
	pub fn from_order(
		total: &BigDecimal,
		dp: &BigDecimal,
		hpp: &BigDecimal,
		ref_id: i32,
		ledger_id: i32,
	) -> (Vec<LedgerDetail>, usize) {
		let mut details: Vec<LedgerDetail> = Vec::new();
		let mut i: i16 = 1;
		let mut direction: i16 = -1;
		let remain = total - dp;
		let pass = bigdecimal::BigDecimal::from(0);

		let detail = LedgerDetailBuilder::default()
			.with_ref_id(ref_id)
			.with_ledger_id(ledger_id)
			.with_detail_id(i)
			.with_account_id(Coa::Revenue)
			.with_direction(direction)
			.with_amount(total.to_owned())
			.with_descriptions("Penjualan barang")
			.build();

		details.push(detail);

		if remain.le(&pass) {
			i += 1;
			direction = 1;

			let detail = LedgerDetailBuilder::default()
				.with_ref_id(ref_id)
				.with_ledger_id(ledger_id)
				.with_detail_id(i)
				.with_account_id(Coa::Cash)
				.with_direction(direction)
				.with_amount(total.to_owned())
				.with_descriptions("Cash payment")
				.build();

			details.push(detail);
		} else {
			// sisa pembayaran
			i += 1;
			direction = 1;

			let detail = LedgerDetailBuilder::default()
				.with_ref_id(ref_id)
				.with_ledger_id(ledger_id)
				.with_detail_id(i)
				.with_account_id(Coa::Loan)
				.with_direction(direction)
				.with_amount(remain)
				.with_descriptions("Piutang barang")
				.build();

			details.push(detail);

			// jika ada pembayaran
			if dp.gt(&pass) {
				i += 1;
				direction = 1;
				let detail = LedgerDetailBuilder::default()
					.with_ref_id(ref_id)
					.with_ledger_id(ledger_id)
					.with_detail_id(i)
					.with_account_id(Coa::Cash)
					.with_direction(direction)
					.with_amount(dp.to_owned())
					.with_descriptions("Cash DP")
					.build();

				details.push(detail);
			}
		}

		i += 1;
		direction = -1;

		let detail = LedgerDetailBuilder::default()
			.with_ref_id(ref_id)
			.with_ledger_id(ledger_id)
			.with_detail_id(i)
			.with_account_id(Coa::Inventory)
			.with_direction(direction)
			.with_amount(hpp.to_owned())
			.with_descriptions("Persediaan barang")
			.build();

		details.push(detail);

		i += 1;
		direction = 1;

		let detail = LedgerDetailBuilder::default()
			.with_ref_id(ref_id)
			.with_ledger_id(ledger_id)
			.with_detail_id(i)
			.with_account_id(Coa::GoodCost)
			.with_direction(direction)
			.with_amount(hpp.to_owned())
			.with_descriptions("Biaya Beli Barang")
			.build();

		details.push(detail);

		(details, i as usize)
	}

	/// ## Ledger untuk transaksi pembayaran piutang
	///
	/// `(+) 101 - Kas` -> **amount**
	///
	/// `(-) 111 - Piutang barang` -> **amount**
	///
	/// **`ref_id`** payment id
	///
	/// **`ledger_id`** ledger id biasanya sama dengan payment id
	///

	pub fn from_order_payment(
		amount: &BigDecimal,
		ref_id: i32,
		ledger_id: i32,
	) -> (Vec<LedgerDetail>, usize) {
		let mut details: Vec<LedgerDetail> = Vec::new();
		let mut i: i16 = 1;
		let mut direction: i16 = 1;

		let detail = LedgerDetailBuilder::default()
			.with_ref_id(ref_id)
			.with_ledger_id(ledger_id)
			.with_detail_id(i)
			.with_account_id(Coa::Cash)
			.with_direction(direction)
			.with_amount(amount.to_owned())
			.with_descriptions("Titip bayar")
			.build();

		details.push(detail);

		i += 1;
		direction = -1;

		let detail = LedgerDetailBuilder::default()
			.with_ref_id(ref_id)
			.with_ledger_id(ledger_id)
			.with_detail_id(i)
			.with_account_id(Coa::Loan)
			.with_direction(direction)
			.with_amount(amount.to_owned())
			.with_descriptions("Piutang penjualan")
			.build();

		details.push(detail);

		(details, i as usize)
	}
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     pub fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

pub mod db {
	use crate::{
		db::DBClient,
		model::{Ledger, LedgerResult, LedgerWithDetails, MatchResult},
	};
	use async_trait::async_trait;
	use crate::model::{LedgerSchema, LedgerType, LedgerDetail};
	use sqlx::{self, types::Json, Acquire, Error};

	#[async_trait]
	pub trait LedgerExt {
		async fn get_ledger(
			&self,
			id: i32,
		) -> Result<Option<LedgerWithDetails>, Error>;
		async fn get_ledgers(
			&self,
			page: usize,
			limit: usize,
		) -> Result<MatchResult, Error>;
		async fn ledger_create<T>(
			&self,
			data: T,
		) -> Result<Option<LedgerResult>, Error>
		where
			T: Into<LedgerSchema> + Send;
		async fn ledger_update<T>(
			&self,
			id: i32,
			data: T,
		) -> Result<Option<LedgerResult>, Error>
		where
			T: Into<LedgerSchema> + Send;
		async fn ledger_delete(&self, id: i32) -> Result<u64, Error>;
	}

	#[async_trait]
	impl LedgerExt for DBClient {
		async fn get_ledger(
			&self,
			id: i32,
		) -> Result<Option<LedgerWithDetails>, Error> {
			let query = sqlx::query_file_as!(
				LedgerWithDetails,
				"sql/ledger-get-by-id.sql",
				id
			);
			let ledger = query.fetch_optional(&self.pool).await?;

			Ok(ledger)
		}

		async fn get_ledgers(
			&self,
			page: usize,
			limit: usize,
		) -> Result<MatchResult, Error> {
			let x: usize = 1;
			let offset = (page - x) * limit;

			// acquire pg connection from current pool
			let mut conn = self.pool.acquire().await?; //.unwrap();

			// get transaction pool from pg connection
			let mut tx = conn.begin().await?;

			// start transaction
			// get orders data from database
			let query = sqlx::query_file_as!(
				Ledger,
				"sql/ledger-get-all.sql",
				limit as i64,
				offset as i64
			);
			let ledgers = query.fetch_all(&mut *tx).await?;

			// start transacrion
			// get total record of orders
			let scalar = sqlx::query_file_scalar!("sql/ledger-count.sql");
			let row = scalar.fetch_one(&mut *tx).await?;

			// finish transaction
			tx.commit().await?;

			Ok((ledgers, row.unwrap_or(0)))
		}

		async fn ledger_create<T>(
			&self,
			data: T,
		) -> Result<Option<LedgerResult>, Error>
		where
			T: Into<LedgerSchema> + Send,
		{
			let t: LedgerSchema = data.try_into().unwrap();
			let ledger = sqlx::query_file_as!(
				LedgerResult,
				"sql/ledger-insert.sql",
				t.relation_id,
				t.ledger_type as LedgerType,
				t.updated_by,
				t.is_valid,
				t.descriptions,
			)
			.fetch_optional(&self.pool)
			.await?;

			Ok(ledger)
		}

		async fn ledger_update<T: Into<LedgerSchema> + Send>(
			&self,
			id: i32,
			data: T,
		) -> Result<Option<LedgerResult>, Error>
		where
			T: Into<LedgerSchema> + Send,
		{
			let t: LedgerSchema = data.try_into().unwrap();
			let ledger = sqlx::query_file_as!(
				LedgerResult,
				"sql/ledger-update.sql",
				id,
				t.relation_id.to_owned(),
				t.ledger_type as LedgerType,
				t.updated_by.to_owned(),
				t.is_valid.to_owned(),
				t.descriptions.to_owned()
			)
			.fetch_optional(&self.pool)
			.await?;

			Ok(ledger)
		}

		async fn ledger_delete(&self, id: i32) -> Result<u64, Error> {
			let rows_affected: u64 =
				sqlx::query_file!("sql/ledger-delete.sql", id)
					.execute(&self.pool)
					.await
					.unwrap()
					.rows_affected();

			Ok(rows_affected)
		}
	}
}

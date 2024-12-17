pub mod db {
	use crate::{
		db::DBClient,
		model::{Account, Accounts},
	};
	use async_trait::async_trait;
	use sqlx;

	#[async_trait]
	pub trait AccountExt {
		async fn get_account(
			&self,
			id: i16,
		) -> Result<Option<Accounts>, sqlx::Error>;
		async fn get_accounts(
			&self,
			page: u32,
			limit: usize,
		) -> Result<Vec<Accounts>, sqlx::Error>;
		async fn account_create<T: Into<Account> + Send>(
			&self,
			data: T,
		) -> Result<Option<Accounts>, sqlx::Error>;
		async fn account_update<T: Into<Account> + Send>(
			&self,
			id: i16,
			data: T,
		) -> Result<Option<Accounts>, sqlx::Error>;
		async fn account_delete(&self, id: i16) -> Result<u64, sqlx::Error>;
	}

	#[async_trait]
	impl AccountExt for DBClient {
		async fn get_account(
			&self,
			id: i16,
		) -> Result<Option<Accounts>, sqlx::Error> {
			let account =
				sqlx::query_file_as!(Accounts, "sql/account-get-by-id.sql", id)
					.fetch_optional(&self.pool)
					.await?;
			Ok(account)
		}

		async fn get_accounts(
			&self,
			page: u32,
			limit: usize,
		) -> Result<Vec<Accounts>, sqlx::Error> {
			let offset = (page - 1) * limit as u32;

			let accounts = sqlx::query_file_as!(
				Accounts,
				"sql/account-get-all.sql",
				limit as i64,
				offset as i64
			)
			.fetch_all(&self.pool)
			.await?;
			Ok(accounts)
		}

		async fn account_create<T: Into<Account> + Send>(
			&self,
			data: T,
		) -> Result<Option<Accounts>, sqlx::Error> {
			let t: Account = data.into(); // as CreateAccountSchema;
			let account = sqlx::query_file_as!(
				Accounts,
				"sql/account-insert.sql",
				t.id.to_owned(),
				t.name.to_string(),
				t.root.to_owned().unwrap_or(0),
				t.normal.to_owned(),
				t.en_name.to_owned(),
				t.descriptions.to_owned(),
				t.is_active.to_owned(),
				t.payable.to_owned(),
			)
			.fetch_optional(&self.pool)
			.await?;

			Ok(account)
		}

		async fn account_update<T: Into<Account> + Send>(
			&self,
			id: i16,
			data: T,
		) -> Result<Option<Accounts>, sqlx::Error> {
			let t: Account = data.into();
			let account = sqlx::query_file_as!(
				Accounts,
				"sql/account-update.sql",
				id,
				t.name.to_owned(),
				t.root.to_owned().unwrap_or(0),
				t.normal.to_owned(),
				t.en_name.to_owned(),
				t.descriptions.to_owned(),
				t.is_active.to_owned(),
				t.payable.to_owned(),
				chrono::offset::Utc::now()
			)
			.fetch_optional(&self.pool)
			.await?;

			Ok(account)
		}

		async fn account_delete(&self, id: i16) -> Result<u64, sqlx::Error> {
			let rows_affected: u64 =
				sqlx::query_file!("sql/account-delete.sql", id)
					.execute(&self.pool)
					.await
					.unwrap()
					.rows_affected();

			Ok(rows_affected)
		}
	}
}

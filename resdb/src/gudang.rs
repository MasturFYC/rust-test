pub mod model {
	use serde::{Deserialize, Serialize};
	use sqlx::{types::Json, FromRow, Row};
	use validator::Validate;

	#[derive(Validate, Debug, Deserialize, Serialize, Clone)]
	pub struct Gudang {
		#[serde(skip_serializing_if = "Option::is_none")]
		pub id: Option<i16>,
		#[validate(length(min = 1, message = "Name is required"))]
		pub name: String,
		#[serde(rename = "employeeId")]
		pub employee_id: i16,
		#[serde(
			rename = "employeeName",
			skip_serializing_if = "Option::is_none"
		)]
		pub employee_name: Option<String>,
		#[serde(skip_serializing_if = "Option::is_none")]
		pub locate: Option<String>,
	}

	#[derive(Debug, Deserialize, Serialize, Validate)]
	pub struct Gudangs {
		pub id: Option<i16>,
		pub name: String,
		#[serde(rename = "employeeId")]
		pub employee_id: i16,
		#[serde(skip_serializing_if = "Option::is_none")]
		pub locate: Option<String>,
	}

	#[derive(Default, Serialize, Deserialize, Debug, FromRow, sqlx::Type)]
	pub struct GudangProduct {
		pub id: i32,
		pub name: String,
	}

	#[derive(Default, Serialize, Deserialize, Debug)]
	pub struct GudangWithProduct {
		pub id: i16,
		pub name: String,
		#[serde(rename = "employeeId")]
		pub employee_id: i16,
		#[serde(rename = "employeeName")]
		pub employee_name: String,
		#[serde(skip_serializing_if = "Option::is_none")]
		pub locate: Option<String>,
		#[serde(skip_serializing_if = "Option::is_none")]
		pub products: Option<Json<Vec<GudangProduct>>>,
	}

	impl<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> for GudangWithProduct {
		fn from_row(
			row: &'r sqlx::postgres::PgRow,
		) -> Result<Self, sqlx::Error> {
			let id = row.get("id");
			let name = row.get("name");
			let employee_id = row.get("employee_id");
			let employee_name = row.get("employee_name");
			let locate = row.get("locate");
			// let products = row
			//     .try_get::<sqlx::types::Json<Vec<CategoryProduct>>, _>("products")
			//     .map(|r| if r.is_empty() {None} else { Some (r.0) })
			//     .unwrap_or(None);
			let products = row
				.try_get::<sqlx::types::Json<Vec<GudangProduct>>, _>("products")
				.map(Some)
				.unwrap();
			// serde_json::from_value(row.get("products")).unwrap_or(None);

			Ok(Self {
				id,
				name,
				employee_id,
				employee_name,
				locate,
				products,
			})
		}
	}
}

pub mod db {
	use super::model::{Gudang, Gudangs, GudangProduct, GudangWithProduct};
	use crate::{db::DBClient, product::model::ProductIds};
	use async_trait::async_trait;
	use sqlx::{self, types::Json, Acquire};

	#[async_trait]
	pub trait GudangExt {
		async fn get_gudang(
			&self,
			id: i16,
		) -> Result<Option<Gudang>, sqlx::Error>;
		async fn get_gudangs(&self) -> Result<Vec<Gudang>, sqlx::Error>;
		async fn get_gudang_with_products(
			&self,
			id: i16,
		) -> Result<Option<GudangWithProduct>, sqlx::Error>;
		async fn gudang_create<T>(
			&self,
			data: T,
		) -> Result<Option<Gudangs>, sqlx::Error>
		where
			T: Into<Gudang> + Send;
		async fn gudang_update<T>(
			&self,
			id: i16,
			data: T,
		) -> Result<Option<Gudangs>, sqlx::Error>
		where
			T: Into<Gudang> + Send;
		async fn gudang_delete(
			&self,
			id: i16,
		) -> Result<Option<u64>, sqlx::Error>;
	}

	#[async_trait]
	impl GudangExt for DBClient {
		async fn get_gudang(
			&self,
			id: i16,
		) -> Result<Option<Gudang>, sqlx::Error> {
			let query_result = sqlx::query_file_as!(
				Gudang, //
				"sql/gudang-get-by-id.sql",
				id
			)
			.fetch_optional(&self.pool)
			.await?;
			Ok(query_result)
		}

		async fn get_gudangs(&self) -> Result<Vec<Gudang>, sqlx::Error> {
			let query_result = sqlx::query_file_as!(
				Gudang,
				"sql/gudang-get-all-order-by-name.sql"
			)
			.fetch_all(&self.pool)
			.await?;

			Ok(query_result)
		}

		async fn gudang_create<T>(
			&self,
			data: T,
		) -> Result<Option<Gudangs>, sqlx::Error>
		where
			T: Into<Gudang> + Send,
		{
			let mut cnn = self.pool.acquire().await?;
			let mut tx = cnn.begin().await?;
			let g: Gudang = data.into();
			let products = sqlx::query_as!(
				ProductIds,
				"SELECT id FROM products ORDER BY id;"
			)
			.fetch_all(&mut *tx)
			.await?;
			let query_result = sqlx::query_file_as!(
				Gudangs,
				"sql/gudang-insert.sql",
				g.name,
				g.employee_id,
				g.locate,
			)
			.fetch_optional(&mut *tx)
			.await?;
			let gudang = query_result.unwrap();
			let ids_length = products.len();
			if ids_length > 0 {
				let new_gudang_id = gudang.id;
				let mut i: usize = 0;
				loop {
					if let Some(x) = products.get(i) {
						let _ = sqlx::query!(
							r#"
                    INSERT INTO stocks
                        (gudang_id, product_id, qty)
                    VALUES
                        ($1, $2, 0)
                    "#,
							new_gudang_id,
							x.id
						)
						.execute(&mut *tx)
						.await?;
					}
					i = i.checked_add(1).unwrap();
					if i == ids_length {
						break;
					}
				}
			}
			tx.commit().await?;
			Ok(Some(gudang))
		}

		async fn gudang_update<T>(
			&self,
			id: i16,
			data: T,
		) -> Result<Option<Gudangs>, sqlx::Error>
		where
			T: Into<Gudang> + Send,
		{
			let g: Gudang = data.into();
			let query_result = sqlx::query_file_as!(
				Gudangs,
				"sql/gudang-update.sql",
				id,
				g.name,
				g.employee_id,
				g.locate,
			)
			.fetch_optional(&self.pool)
			.await?;

			Ok(query_result)
		}

		async fn gudang_delete(
			&self,
			id: i16,
		) -> Result<Option<u64>, sqlx::Error> {
			//print!("GUDANG TO BE DELETED: {}", id);
			let rows_affected = sqlx::query_file!("sql/gudang-delete.sql", id)
				.execute(&self.pool)
				.await?
				.rows_affected();

			Ok(Some(rows_affected))
		}

		async fn get_gudang_with_products(
			&self,
			id: i16,
		) -> Result<Option<GudangWithProduct>, sqlx::Error> {
			let query_result = sqlx::query_file_as!(
				GudangWithProduct,
				"sql/gudang-with-product.sql",
				id
			)
			.fetch_optional(&self.pool)
			.await?;
			Ok(query_result)
		}
	}
}

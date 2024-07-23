use serde::{Deserialize, Serialize};
use sqlx::{types::Json, FromRow, Row};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct Category {
	#[validate(length(min = 1, message = "Name is required"))]
	pub name: String,
}

// #[derive(Debug, Serialize, Deserialize, sqlx::FromRow, sqlx::Type, Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Categories {
	pub id: i16,
	pub name: String,
}

#[derive(Default, Serialize, Deserialize, Debug, FromRow, sqlx::Type)]
pub struct CategoryProduct {
	pub id: i32,
	pub name: String,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct CategoryWithProduct {
	pub id: i16,
	pub name: String,
	pub products: Option<Json<Vec<CategoryProduct>>>,
}

impl<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> for CategoryWithProduct {
	fn from_row(row: &'r sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
		let id = row.get("id");
		let name = row.get("name");
		// let products = row
		//     .try_get::<sqlx::types::Json<Vec<CategoryProduct>>, _>("products")
		//     .map(|r| if r.is_empty() {None} else { Some (r.0) })
		//     .unwrap_or(None);
		let products = row
			.try_get::<sqlx::types::Json<Vec<CategoryProduct>>, _>("products")
			.map(|x| if x.is_empty() { None } else { Some(x) })
			.unwrap_or(None); // serde_json::from_value(row.get("products")).unwrap_or(None);

		// let products: Option<Vec<CategoryProduct>> = serde_json::from_value(r.get("products")).unwrap_or(None);

		Ok(Self { id, name, products })
	}
}

// impl<'r> FromRow<'r, PgRow> for CategoryProduct {
//     fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
//         let id = row.get("id");
//         let name = row.get("name");
//         Ok(Self {id, name})
//     }
// }

pub mod db {
	use super::*;
	use crate::db::DBClient;
	use async_trait::async_trait;
	use sqlx;

	#[async_trait]
	pub trait CategoryExt {
		async fn get_category(
			&self,
			id: i16,
		) -> Result<Option<Categories>, sqlx::Error>;
		async fn get_categories(&self) -> Result<Vec<Categories>, sqlx::Error>;
		async fn get_category_with_products(
			&self,
			id: i16,
		) -> Result<Option<CategoryWithProduct>, sqlx::Error>;
		// async fn get_products(&self, page: u32, limit: usize) -> Result<(Vec<Products>, i64), sqlx::Error>;
		// async fn get_products_by_category(&self, category_id: i16, page: u32, limit: usize) -> Result<(Vec<Products>, i64), sqlx::Error>;
		// async fn get_products_by_supplier(&self, supplier_id: Uuid, page: u32, limit: usize) -> Result<(Vec<Products>, i64), sqlx::Error>;
		async fn category_create<T>(
			&self,
			data: T,
		) -> Result<Option<Categories>, sqlx::Error>
		where
			T: Into<Category> + Send;
		async fn category_update<T>(
			&self,
			id: i16,
			data: T,
		) -> Result<Option<Categories>, sqlx::Error>
		where
			T: Into<Category> + Send;
		async fn category_delete(
			&self,
			id: i16,
		) -> Result<Option<u64>, sqlx::Error>;
	}

	#[async_trait]
	impl CategoryExt for DBClient {
		async fn get_category(
			&self,
			id: i16,
		) -> Result<Option<Categories>, sqlx::Error> {
			let query_result = sqlx::query_file_as!(
				Categories,
				"sql/category-get-by-id.sql",
				id
			)
			.fetch_optional(&self.pool)
			.await?;
			Ok(query_result)
		}

		async fn get_categories(&self) -> Result<Vec<Categories>, sqlx::Error> {
			let query_result =
				sqlx::query_file_as!(Categories, "sql/category-get-all.sql")
					.fetch_all(&self.pool)
					.await?;

			Ok(query_result)
		}

		async fn category_create<T>(
			&self,
			data: T,
		) -> Result<Option<Categories>, sqlx::Error>
		where
			T: Into<Category> + Send,
		{
			let cat: Category = data.try_into().unwrap();
			let query_result = sqlx::query_file_as!(
				Categories,
				"sql/category-insert.sql",
				cat.name
			)
			.fetch_optional(&self.pool)
			.await?;

			Ok(query_result)
		}

		async fn category_update<T>(
			&self,
			id: i16,
			data: T,
		) -> Result<Option<Categories>, sqlx::Error>
		where
			T: Into<Category> + Send,
		{
			let cat: Category = data.try_into().unwrap();
			let query_result = sqlx::query_file_as!(
				Categories,
				"sql/category-update.sql",
				id,
				cat.name
			)
			.fetch_optional(&self.pool)
			.await?;

			Ok(query_result)
		}

		async fn category_delete(
			&self,
			id: i16,
		) -> Result<Option<u64>, sqlx::Error> {
			let rows_affected =
				sqlx::query_file!("sql/category-delete.sql", id)
					.execute(&self.pool)
					.await?		
					.rows_affected();

			Ok(Some(rows_affected))
		}

		async fn get_category_with_products(
			&self,
			id: i16,
		) -> Result<Option<CategoryWithProduct>, sqlx::Error> {
			let query_result = sqlx::query_file_as!(
				CategoryWithProduct,
				"sql/category-with-product.sql",
				id
			)
			.fetch_optional(&self.pool)
			.await?;
			Ok(query_result)
		}
	}
}
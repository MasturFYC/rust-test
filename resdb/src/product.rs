use bigdecimal::BigDecimal;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct Products {
	pub id: Uuid,
	#[serde(rename = "supplierId")]
	pub supplier_id: Uuid,
	pub name: String,
	pub barcode: String,
	pub unit: String,
	#[serde(rename = "unitInStock")]
	pub unit_in_stock: BigDecimal,
	pub content: BigDecimal,
	pub hpp: BigDecimal,
	pub margin: BigDecimal,
	pub price: BigDecimal,
	pub ppn: BigDecimal,
	#[serde(rename = "isActive")]
	pub is_active: bool,
	#[serde(rename = "variantName")]
	pub variant_name: Option<String>,
	pub descriptions: Option<String>,
	#[serde(rename = "categoryId")]
	pub category_id: i16,
	#[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
	pub created_at: Option<DateTime<Utc>>,
	#[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
	pub updated_at: Option<DateTime<Utc>>,
	#[serde(rename = "categoryName")]
	pub category_name: String,
	#[serde(rename = "supplierName")]
	pub supplier_name: String,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct Product {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<Uuid>,
	#[serde(rename = "supplierId")]
	pub supplier_id: Uuid,
	#[validate(length(min = 1, message = "Name is required"))]
	pub name: String,
	#[validate(length(min = 1, message = "Barcode is required"))]
	pub barcode: String,
	#[validate(length(min = 1, message = "Unit is required"))]
	pub unit: String,
	pub content: BigDecimal,
	pub hpp: BigDecimal,
	pub margin: BigDecimal,
	pub price: BigDecimal,
	pub ppn: BigDecimal,
	#[serde(rename = "isActive")]
	pub is_active: bool,
	#[serde(rename = "variantName", skip_serializing_if = "Option::is_none")]
	pub variant_name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub descriptions: Option<String>,
	#[serde(rename = "categoryId")]
	pub category_id: i16,
	// #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
	// pub created_at: Option<DateTime<Utc>>,
	// #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
	// pub updated_at: Option<DateTime<Utc>>,
}

// #[derive(Debug, Serialize, Deserialize, sqlx::FromRow, sqlx::Type, Clone)]
#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct ProductOriginal {
	pub id: Uuid,
	#[serde(rename = "supplierId")]
	pub supplier_id: Uuid,
	pub name: String,
	pub barcode: String,
	pub unit: String,
	#[serde(rename = "unitInStock")]
	pub unit_in_stock: BigDecimal,
	pub content: BigDecimal,
	pub hpp: BigDecimal,
	pub margin: BigDecimal,
	pub price: BigDecimal,
	pub ppn: BigDecimal,
	#[serde(rename = "isActive")]
	pub is_active: bool,
	#[serde(rename = "variantName")]
	pub variant_name: Option<String>,
	pub descriptions: Option<String>,
	#[serde(rename = "categoryId")]
	pub category_id: i16,
	#[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
	pub created_at: Option<DateTime<Utc>>,
	#[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
	pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductResponseDto {
	pub status: String,
	pub data: Products,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateResponseDto {
	pub status: String,
	pub data: Product,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteResponseDto {
	pub status: String,
	pub data: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductListResponseDto {
	pub status: String,
	pub data: Vec<Products>,
	pub count: usize,
}

pub mod db {

	use super::*;
	use crate::db::DBClient;
	use async_trait::async_trait;
	use sqlx::{self, Acquire};

	#[async_trait]
	pub trait ProductExt {
		async fn get_product_origin(
			&self,
			id: uuid::Uuid,
		) -> Result<Option<ProductOriginal>, sqlx::Error>;
		async fn get_product(
			&self,
			id: uuid::Uuid,
		) -> Result<Option<Products>, sqlx::Error>;
		async fn get_products(
			&self,
			page: u32,
			limit: usize,
		) -> Result<(Vec<Products>, i64), sqlx::Error>;
		async fn get_products_by_category(
			&self,
			category_id: i16,
			page: u32,
			limit: usize,
		) -> Result<(Vec<Products>, i64), sqlx::Error>;
		async fn get_products_by_supplier(
			&self,
			supplier_id: uuid::Uuid,
			page: u32,
			limit: usize,
		) -> Result<(Vec<Products>, i64), sqlx::Error>;
		async fn product_create(
			&self,
			data: Product,
		) -> Result<ProductOriginal, sqlx::Error>;
		async fn product_update(
			&self,
			id: uuid::Uuid,
			data: Product,
			old: ProductOriginal,
		) -> Result<ProductOriginal, sqlx::Error>;
		async fn product_delete(
			&self,
			id: uuid::Uuid,
		) -> Result<u64, sqlx::Error>;
	}

	#[async_trait]
	impl ProductExt for DBClient {
		async fn get_product_origin(
			&self,
			id: uuid::Uuid,
		) -> Result<Option<ProductOriginal>, sqlx::Error> {
			let product = sqlx::query_file_as!(
				ProductOriginal,
				"sql/product-get-origin-by-id.sql",
				id
			)
			.fetch_optional(&self.pool)
			.await?;

			Ok(product)
		}

		async fn get_product(
			&self,
			id: uuid::Uuid,
		) -> Result<Option<Products>, sqlx::Error> {
			let product =
				sqlx::query_file_as!(Products, "sql/product-get-by-id.sql", id)
					.fetch_optional(&self.pool)
					.await?;

			Ok(product)
		}

		async fn get_products(
			&self,
			page: u32,
			limit: usize,
		) -> Result<(Vec<Products>, i64), sqlx::Error> {
			let offset = (page - 1) * limit as u32;

			let mut conn = self.pool.acquire().await?;
			let mut tx = conn.begin().await?;

			let count = sqlx::query_scalar!("SELECT COUNT(*) FROM products")
				.fetch_one(&mut *tx)
				.await?;

			let products = sqlx::query_file_as!(
				Products,
				"sql/product-get-all.sql",
				limit as i64,
				offset as i64
			)
			.fetch_all(&mut *tx)
			.await?;

			tx.commit().await?;

			Ok((products, count.unwrap_or(0)))
		}

		async fn get_products_by_category(
			&self,
			category_id: i16,
			page: u32,
			limit: usize,
		) -> Result<(Vec<Products>, i64), sqlx::Error> {
			let offset = (page - 1) * limit as u32;

			let mut conn = self.pool.acquire().await?;
			let mut tx = conn.begin().await?;

			let count = sqlx::query_scalar!(
				"SELECT COUNT(*) FROM products WHERE category_id = $1",
				category_id
			)
			.fetch_one(&mut *tx)
			.await?;

			let products = sqlx::query_file_as!(
				Products,
				"sql/product-get-all-by-category.sql",
				category_id,
				limit as i64,
				offset as i64
			)
			.fetch_all(&mut *tx)
			.await?;

			tx.commit().await?;

			Ok((products, count.unwrap_or(0)))
		}

		async fn get_products_by_supplier(
			&self,
			supplier_id: uuid::Uuid,
			page: u32,
			limit: usize,
		) -> Result<(Vec<Products>, i64), sqlx::Error> {
			let offset = (page - 1) * limit as u32;

			let mut conn = self.pool.acquire().await?;
			let mut tx = conn.begin().await?;

			let count = sqlx::query_scalar!(
				"SELECT COUNT(*) FROM products WHERE supplier_id = $1",
				supplier_id
			)
			.fetch_one(&mut *tx)
			.await?;

			let products = sqlx::query_file_as!(
				Products,
				"sql/product-get-all-by-supplier.sql",
				supplier_id,
				limit as i64,
				offset as i64
			)
			.fetch_all(&mut *tx)
			.await?;

			tx.commit().await?;

			Ok((products, count.unwrap_or(0)))
		}

		async fn product_create(
			&self,
			data: Product,
		) -> Result<ProductOriginal, sqlx::Error> {
			// let mut stmt = self.pool.prepare("SELECT * FROM users WHERE id = $1").await?;
			let product = sqlx::query_file_as!(
				ProductOriginal,
				"sql/product-insert.sql",
				data.name,
				data.barcode,
				data.unit,
				data.content,
				data.hpp,
				data.margin,
				data.price,
				data.ppn,
				data.is_active,
				data.variant_name.to_owned().unwrap_or("".to_string()),
				data.descriptions.to_owned().unwrap_or("".to_string()),
				data.category_id,
				data.supplier_id
			)
			.fetch_one(&self.pool)
			.await?;

			Ok(product)
		}

		async fn product_update(
			&self,
			id: uuid::Uuid,
			data: Product,
			old: ProductOriginal,
		) -> Result<ProductOriginal, sqlx::Error> {
			let product = sqlx::query_file_as!(
				ProductOriginal,
				"sql/product-update.sql",
				id,
				data.name,
				data.barcode,
				data.unit,
				data.content,
				data.hpp,
				data.margin,
				data.price,
				data.ppn,
				data.is_active,
				data.variant_name
					.to_owned()
					.unwrap_or(old.variant_name.unwrap_or("".to_string())),
				data.descriptions
					.to_owned()
					.unwrap_or(old.descriptions.unwrap_or("".to_string())),
				data.category_id,
				data.supplier_id
			)
			.fetch_one(&self.pool)
			.await?;

			Ok(product)
		}

		async fn product_delete(
			&self,
			id: uuid::Uuid,
		) -> Result<u64, sqlx::Error> {
			let rows_affected: u64 =
				sqlx::query_file!("sql/product-delete.sql", id)
					.execute(&self.pool)
					.await
					.unwrap()
					.rows_affected();

			Ok(rows_affected)
		}
	}
}

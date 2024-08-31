pub mod model {
	use bigdecimal::BigDecimal;
	use chrono::prelude::*;
	use serde::{Deserialize, Serialize};
	use validator::Validate;

	#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
	pub struct BarcodeList {
		pub barcode: String,
	}

	#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
	pub struct Products {
		pub id: i16,
		#[serde(rename = "supplierId")]
		pub supplier_id: i16,
		#[serde(rename = "categoryId")]
		pub category_id: i16,
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
		pub heavy: f32,
		#[serde(rename = "isActive")]
		pub is_active: bool,
		#[serde(rename = "variantName")]
		pub variant_name: Option<String>,
		pub descriptions: Option<String>,
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
		pub id: Option<i16>,
		#[serde(rename = "supplierId")]
		pub supplier_id: i16,
		#[serde(rename = "categoryId")]
		pub category_id: i16,
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
		pub heavy: f32,
		#[serde(rename = "isActive")]
		pub is_active: bool,
		#[serde(rename = "variantName", skip_serializing_if = "Option::is_none")]
		pub variant_name: Option<String>,
		#[serde(skip_serializing_if = "Option::is_none")]
		pub descriptions: Option<String>,
		// #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
		// pub created_at: Option<DateTime<Utc>>,
		// #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
		// pub updated_at: Option<DateTime<Utc>>,
	}

	// #[derive(Debug, Serialize, Deserialize, sqlx::FromRow, sqlx::Type, Clone)]
	#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
	pub struct ProductOriginal {
		pub id: i16,
		#[serde(rename = "supplierId")]
		pub supplier_id: i16,
		#[serde(rename = "categoryId")]
		pub category_id: i16,
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
		pub heavy: f32,
		#[serde(rename = "isActive")]
		pub is_active: bool,
		#[serde(rename = "variantName")]
		pub variant_name: Option<String>,
		pub descriptions: Option<String>,
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
	#[derive(Deserialize)]
	pub struct FindName {
		pub txt: String,
		pub limit: Option<usize>
	}
}

pub mod db {

	use crate::db::DBClient;
	use async_trait::async_trait;
	use sqlx::{self, Acquire};

	use super::model::{BarcodeList, Product, ProductOriginal, Products};

	#[async_trait]
	pub trait ProductExt {
		async fn get_product_origin(
			&self,
			id: i16,
		) -> Result<Option<ProductOriginal>, sqlx::Error>;
		async fn get_product(&self, id: i16) -> Result<Option<Products>, sqlx::Error>;
		async fn get_barcodes(&self) -> Result<Vec<BarcodeList>, sqlx::Error>;
		async fn get_product_by_barcode(&self, barcode: String) -> Result<Option<Products>, sqlx::Error>;
		async fn get_products(
			&self,
			page: u32,
			limit: usize,
			opt: Option<i8>,
			txt: Option<String>,
			relid: Option<i16>,
			catid: Option<i16>,
		) -> Result<(Vec<Products>, i64), sqlx::Error>;
		async fn get_products_by_name(
			&self,
			limit: i64,
			txt: String
		) -> Result<Vec<Products>, sqlx::Error>;
		async fn get_products_by_category(
			&self,
			category_id: i16,
			page: u32,
			limit: usize,
		) -> Result<(Vec<Products>, i64), sqlx::Error>;
		async fn get_products_by_supplier(
			&self,
			supplier_id: i16,
			page: u32,
			limit: usize,
		) -> Result<(Vec<Products>, i64), sqlx::Error>;
		async fn product_create(&self, data: Product) -> Result<ProductOriginal, sqlx::Error>;
		async fn product_update(
			&self,
			id: i16,
			data: Product,
			old: ProductOriginal,
		) -> Result<ProductOriginal, sqlx::Error>;
		async fn product_delete(&self, id: i16) -> Result<u64, sqlx::Error>;
	}

	#[async_trait]
	impl ProductExt for DBClient {
		async fn get_product_origin(
			&self,
			id: i16,
		) -> Result<Option<ProductOriginal>, sqlx::Error> {
			let product =
				sqlx::query_file_as!(ProductOriginal, "sql/product-get-origin-by-id.sql", id)
					.fetch_optional(&self.pool)
					.await?;

			Ok(product)
		}

		async fn get_product(&self, id: i16) -> Result<Option<Products>, sqlx::Error> {
			let product = sqlx::query_file_as!(Products, "sql/product-get-by-id.sql", id)
				.fetch_optional(&self.pool)
				.await?;

			Ok(product)
		}
		async fn get_product_by_barcode(&self, barcode: String) -> Result<Option<Products>, sqlx::Error> {
			let product = sqlx::query_file_as!(
				Products,
				"sql/product-get-by-barcode.sql",
				barcode.to_uppercase())
				.fetch_optional(&self.pool)
				.await?;

			Ok(product)
		}

		async fn get_barcodes(&self) -> Result<Vec<BarcodeList>, sqlx::Error> {
			
			let barcodes = sqlx::query_file_as!(BarcodeList, "sql/product-get-barcodes.sql")
			.fetch_all(&self.pool)
			.await?;
			Ok(barcodes)
		}
		async fn get_products_by_name(
			&self,
			limit: i64,
			txt: String
		) -> Result<Vec<Products>, sqlx::Error> {
			let search_text = txt.trim().to_lowercase();

			let products = sqlx::query_as!(Products,
				r#"
				SELECT 
					p.*,
					c.name AS category_name,
					r.name AS supplier_name
				FROM 
					products AS p
					INNER JOIN categories AS c ON c.id = p.category_id
					INNER JOIN relations AS r ON r.id = p.supplier_id
				WHERE 
					POSITION($1 IN LOWER(p.name||' '||p.barcode)) > 0
				ORDER BY
					p.name
				LIMIT $2
				"#, 
					search_text,
					limit
					)
				.fetch_all(&self.pool)
				.await?;

			Ok(products)
		}

		async fn get_products(
			&self,
			page: u32,
			limit: usize,
			opt: Option<i8>,
			txt: Option<String>,
			relid: Option<i16>,
			catid: Option<i16>,
		) -> Result<(Vec<Products>, i64), sqlx::Error> {
            // calculate offset from page
			let offset = (page - 1) * limit as u32;
            // get connection from current pool
			let mut conn = self.pool.acquire().await?;
            // start transaction
			let mut tx = conn.begin().await?;
            // remove space and convert to lower case from search parameter
			let search_text = txt.unwrap_or("".to_string()).trim().to_lowercase();
            //get supplier id
            let op = opt.unwrap_or(0_18);

            // get total count of product
			let count = match op {
				1_i8 =>	sqlx::query_scalar!(r#"
                        SELECT 
                            COUNT(p.*)
                        FROM 
                            products AS p
                            INNER JOIN categories AS c ON c.id = p.category_id
                            INNER JOIN relations AS r ON r.id = p.supplier_id
                        WHERE 
                            POSITION($1 IN LOWER(r.name||' '||c.name||' '||p.name||' '||p.barcode||' '||COALESCE(p.variant_name,'')||' '||COALESCE(p.descriptions, ''))) > 0"#
                            , 
                            search_text,
                            )
                        .fetch_one(&mut *tx)
                        .await?,
				
				2_i8 => {
                    let sup_id = relid.unwrap();
                    sqlx::query_scalar!("SELECT COUNT(*) FROM products WHERE supplier_id = $1", sup_id)
						.fetch_one(&mut *tx)
						.await?
                },
				3_i8 => {
                    let cat_id = catid.unwrap_or(0);
                    sqlx::query_scalar!("SELECT COUNT(*) FROM products WHERE category_id = $1", cat_id)
						.fetch_one(&mut *tx)
						.await?
				},
				_ => sqlx::query_scalar!("SELECT COUNT(*) FROM products")
						.fetch_one(&mut *tx)
						.await?,				
			};

			//			let count = sqlx::query_scalar!("SELECT COUNT(*) FROM products")
			//				.fetch_one(&mut *tx)
			//				.await?;

			let products = match op {
                    1_i8 => sqlx::query_as!(
                        Products,r#"
                        SELECT
                           p.*,
                           c.name AS category_name,
                           r.name AS supplier_name
                        FROM 
                           products AS p
                           INNER JOIN categories AS c ON c.id = p.category_id
                           INNER JOIN relations AS r ON r.id = p.supplier_id
                        WHERE 
                           POSITION($1 IN LOWER(r.name||' '||c.name||' '||p.name||' '||p.barcode||' '||COALESCE(p.variant_name,'')||' '||COALESCE(p.descriptions, ''))) > 0
								ORDER BY
									p.name
                        LIMIT $2
                        OFFSET $3"#,
                        search_text,
                        limit as i64,
                        offset as i64,
                        )
                    .fetch_all(&mut *tx)
                    .await?,
                    2_i8 => sqlx::query_as!(
                        Products,r#"
                        SELECT
                           p.*,
                           c.name AS category_name,
                           r.name AS supplier_name
                        FROM 
                           products AS p
                           INNER JOIN categories AS c ON c.id = p.category_id
                           INNER JOIN relations AS r ON r.id = p.supplier_id
                        WHERE 
                           supplier_id = $1
								ORDER BY
									p.name
                        LIMIT $2
                        OFFSET $3"#,
                        relid.unwrap(),
                        limit as i64,
                        offset as i64,
                        )
                    .fetch_all(&mut *tx)
                    .await?,
					3_i8 => {
						let cat_id = catid.unwrap_or(0);
						sqlx::query_as!(
							Products,r#"
							SELECT
								p.*,
								c.name AS category_name,
								r.name AS supplier_name
							FROM 
								products AS p
								INNER JOIN categories AS c ON c.id = p.category_id
								INNER JOIN relations AS r ON r.id = p.supplier_id
							WHERE 
								category_id = $1
							ORDER BY
								p.name
							LIMIT $2
							OFFSET $3"#,
							cat_id,
							limit as i64,
							offset as i64,
							)
						.fetch_all(&mut *tx)
						.await?
					},
                    _ => sqlx::query_file_as!(
                        Products, "sql/product-get-all.sql", limit as i64, offset as i64)
                        .fetch_all(&mut * tx)
                        .await?,
            };
                    
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
			supplier_id: i16,
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

		async fn product_create(&self, data: Product) -> Result<ProductOriginal, sqlx::Error> {
			// let mut stmt = self.pool.prepare("SELECT * FROM users WHERE id = $1").await?;
			let product = sqlx::query_file_as!(
				ProductOriginal,
				"sql/product-insert.sql",
				data.name,
				data.barcode.to_ascii_uppercase(),
				data.unit,
				data.content,
				data.hpp,
				data.margin,
				data.price,
				data.ppn,
				data.heavy,
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
			id: i16,
			data: Product,
			old: ProductOriginal,
		) -> Result<ProductOriginal, sqlx::Error> {
			let product = sqlx::query_file_as!(
				ProductOriginal,
				"sql/product-update.sql",
				id,
				data.name,
				data.barcode.to_uppercase(),
				data.unit,
				data.content,
				data.hpp,
				data.margin,
				data.price,
				data.ppn,
				data.heavy,
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

		async fn product_delete(&self, id: i16) -> Result<u64, sqlx::Error> {
			let rows_affected: u64 = sqlx::query_file!("sql/product-delete.sql", id)
				.execute(&self.pool)
				.await
				.unwrap()
				.rows_affected();
			Ok(rows_affected)
		}
	}
}
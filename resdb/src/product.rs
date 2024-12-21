pub mod model {
    use bigdecimal::BigDecimal;
    use chrono::prelude::*;
    use serde::{Deserialize, Serialize};
    use sqlx::{types::Json, FromRow, Row};
    use validator::Validate;

    #[derive(Default, Serialize, Clone, Deserialize, Debug, FromRow, sqlx::Type)]
    pub struct ProductStock {
        #[serde(rename = "gudangId")]
        pub gudang_id: i16,
        #[serde(rename = "productId")]
        pub product_id: i16,
        pub qty: BigDecimal,
        pub name: String,
    }

    #[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
    pub struct BarcodeList {
        pub barcode: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct ProductIds {
        pub id: i16,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct Products {
        pub id: i16,
        #[serde(rename = "supplierId")]
        pub supplier_id: i16,
        #[serde(rename = "categoryId")]
        pub category_id: i16,
        pub name: String,
        pub barcode: String,
        pub unit: String,
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
        #[serde(skip_serializing_if = "Option::is_none")]
        pub stocks: Option<Json<Vec<ProductStock>>>,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct ProductUnit {
        pub id: i16,
        pub name: String,
        pub barcode: String,
        pub unit: String,
        pub hpp: BigDecimal,
        pub price: BigDecimal,
        pub discount: Option<BigDecimal>,
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
    }

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
        pub limit: Option<usize>,
    }

    impl<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> for Products {
        fn from_row(row: &'r sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
            let id = row.get("id");
            let supplier_id = row.get("supplier_id");
            let category_id = row.get("category_id");
            let name = row.get("name");
            let barcode = row.get("barcode");
            let unit = row.get("unit");
            let content = row.get("content");
            let hpp = row.get("hpp");
            let margin = row.get("margin");
            let price = row.get("price");
            let ppn = row.get("ppn");
            let heavy = row.get("heavy");
            let is_active = row.get("is_active");
            let variant_name = Some(row.get("variant_name"));
            let descriptions = Some(row.get("descriptions"));
            let created_at = Some(row.get("created_at"));
            let updated_at = Some(row.get("updated_at"));
            let category_name = row.get("category_name");
            let supplier_name = row.get("supplier_name");
            // let products = rows_affected
            // .try_get::<sqlx::types::Json<Vec<CategoryProduct>>, _>("products")
            // .map(|r| if r.is_empty() {None} else { Some (r.0) })
            // .unwrap_or(None);
            let stocks = row
                .try_get::<sqlx::types::Json<Vec<ProductStock>>, _>("stocks")
                .map(|x| if x.is_empty() { None } else { Some(x) })
                .unwrap_or(None);
            // serde_json::from_value(row.get("products")).unwrap_or(None);
            // let products: Option<Vec<CategoryProduct>> = serde_json::from_value(r.get("products")).unwrap_or(None);

            Ok(Self {
                id,
                supplier_id,
                category_id,
                name,
                barcode,
                unit,
                content,
                hpp,
                margin,
                price,
                ppn,
                heavy,
                is_active,
                variant_name,
                descriptions,
                created_at,
                updated_at,
                category_name,
                supplier_name,
                stocks,
            })
        }
    }
}

pub mod db {

    use super::model::{
        BarcodeList, Product, ProductIds, ProductOriginal, ProductStock, ProductUnit, Products,
    };
    use crate::db::DBClient;
    use async_trait::async_trait;
    use sqlx::{self, types::Json, Acquire};

    #[async_trait]
    pub trait ProductExt {
        async fn get_product_origin(&self, id: i16)
            -> Result<Option<ProductOriginal>, sqlx::Error>;
        async fn get_product(&self, id: i16) -> Result<Option<Products>, sqlx::Error>;
        async fn get_barcodes(&self) -> Result<Vec<BarcodeList>, sqlx::Error>;
        async fn get_product_by_barcode(
            &self,
            barcode: String,
        ) -> Result<Option<ProductUnit>, sqlx::Error>;
        async fn get_product_special(
            &self,
            barcode: String,
            cust_id: i16,
        ) -> Result<Option<ProductUnit>, sqlx::Error>;
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
            txt: String,
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
        async fn get_product_by_barcode(
            &self,
            barcode: String,
        ) -> Result<Option<ProductUnit>, sqlx::Error> {
            let product = sqlx::query_file_as!(
                ProductUnit,
                "sql/product/get-barcode.sql",
                barcode.to_uppercase()
            )
            .fetch_optional(&self.pool)
            .await?;

            Ok(product)
        }
        async fn get_product_special(
            &self,
            barcode: String,
            cust_id: i16,
        ) -> Result<Option<ProductUnit>, sqlx::Error> {
            let product = sqlx::query_file_as!(
                ProductUnit,
                "sql/product/get-special.sql",
                cust_id,
                barcode.to_uppercase()
            )
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
            txt: String,
        ) -> Result<Vec<Products>, sqlx::Error> {
            let search_text = txt.trim().to_lowercase();

            let products =
                sqlx::query_file_as!(Products, "sql/product-get-by-name.sql", search_text, limit)
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
            let search_text = txt //
                .unwrap_or("".to_string())
                .trim()
                .to_lowercase();
            //get supplier id
            let op = opt.unwrap_or(0_i8);

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
                1_i8 => {
                    sqlx::query_file_as!(
                        Products,
                        "sql/product-get-all-2.sql",
                        search_text,
                        limit as i64,
                        offset as i64
                    )
                    .fetch_all(&mut *tx)
                    .await?
                }
                2_i8 => {
                    sqlx::query_file_as!(
                        Products,
                        "sql/product-get-all-3.sql",
                        relid.unwrap(),
                        limit as i64,
                        offset as i64,
                    )
                    .fetch_all(&mut *tx)
                    .await?
                }
                3_i8 => {
                    let cat_id = catid.unwrap_or(0);
                    sqlx::query_file_as!(
                        Products,
                        "sql/product-get-all-4.sql",
                        cat_id,
                        limit as i64,
                        offset as i64,
                    )
                    .fetch_all(&mut *tx)
                    .await?
                }
                _ => {
                    sqlx::query_file_as!(
                        Products,
                        "sql/product-get-all.sql",
                        limit as i64,
                        offset as i64
                    )
                    .fetch_all(&mut *tx)
                    .await?
                }
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
            let mut conn = self.pool.acquire().await?;
            let mut tx = conn.begin().await?;
            let ids = sqlx::query_file_as!(ProductIds, "sql/gudang-get-all-order-by-id.sql")
                .fetch_all(&mut *tx)
                .await?;

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
            .fetch_optional(&mut *tx)
            .await?;

            let ids_length = ids.len();
            let mut i: usize = 0;
            let p = product.unwrap();

            loop {
                if let Some(x) = ids.get(i) {
                    let _ = sqlx::query!(
                        "INSERT INTO stocks (gudang_id, product_id, qty) VALUES ($1, $2, 0)",
                        x.id,
                        p.id
                    )
                    .execute(&mut *tx)
                    .await?;
                }
                i = i.checked_add(1).unwrap();
                if i == ids_length {
                    break;
                }
            }

            tx.commit().await?;

            Ok(p)
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

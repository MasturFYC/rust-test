use async_trait::async_trait;
// use futures_util::TryFutureExt;
use sqlx;

use super::{CreateProductSchema, UpdateProductSchema, Product, ProductFull};
use crate::DBClient;

#[async_trait]
pub trait ProductExt {
    async fn get_product(&self, id: i32) -> Result<Option<ProductFull>, sqlx::Error>;
    async fn get_products(&self, page: u32, limit: usize) -> Result<Vec<ProductFull>, sqlx::Error>;
    async fn product_create(&self, data: CreateProductSchema) -> Result<Product, sqlx::Error>;
    async fn product_update(&self, id: i32, data: UpdateProductSchema, old: ProductFull) -> Result<Product, sqlx::Error>;
    async fn product_delete(&self, id: i32) -> Result<u64, sqlx::Error>;
}

#[async_trait]
impl ProductExt for DBClient {
    async fn get_product(&self, id: i32) -> Result<Option<ProductFull>, sqlx::Error> {
        let product = sqlx::query_file_as!(ProductFull, "sql/product-get-by-id.sql", id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(product)
    }
    async fn get_products(&self, page: u32, limit: usize) -> Result<Vec<ProductFull>, sqlx::Error> {
        let offset = (page - 1) * limit as u32;

        // let names: Vec<String> = sqlx::query!(
        //     "SELECT name FROM customers WHERE id IN (SELECT customer_id FROM (SELECT customer_id, MAX(date) AS max_date FROM orders GROUP BY customer_id) AS latest_orders WHERE max_date >= NOW() - INTERVAL '30 days')"
        // )        
        // println!("{} {} {}", offset, page, limit);

        let products = sqlx::query_file_as!(
            ProductFull,
            "sql/product-get-all.sql",
            limit as i64,
            offset as i64
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(products)
    }

    async fn product_create(&self, data: CreateProductSchema) -> Result<Product, sqlx::Error> {
        // let mut stmt = self.pool.prepare("SELECT * FROM users WHERE id = $1").await?;
        let product = sqlx::query_file_as!(
            Product,
            "sql/product-insert.sql",
            data.name.to_string(),
            data.barcode.to_string(),
            data.unit.to_string(),
            data.content.into(),
            data.hpp.into(),
            data.margin.into(),
            data.price.into(),
            data.ppn.into(),
            data.is_active.into(),
            data.variant_name.to_owned().unwrap_or("".to_string()),
            data.descriptions.to_owned().unwrap_or("".to_string()),
            data.category_id.into()
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(product)
    }

    async fn product_update(
        &self,
        id: i32,
        data: UpdateProductSchema,
        old: ProductFull
    ) -> Result<Product, sqlx::Error> {
        let product = sqlx::query_file_as!(
            Product,
            "sql/product-update.sql",
            data.name.to_owned().unwrap_or(old.name),
            data.barcode.to_owned().unwrap_or(old.barcode),
            data.unit.to_owned().unwrap_or(old.unit),
            data.content.to_owned().unwrap_or(old.content),
            data.hpp.to_owned().unwrap_or(old.hpp),
            data.margin.to_owned().unwrap_or(old.margin),
            data.price.to_owned().unwrap_or(old.price),
            data.ppn.to_owned().unwrap_or(old.ppn),
            data.is_active.to_owned().unwrap_or(old.is_active),
            data.variant_name.to_owned().unwrap_or(old.variant_name.unwrap_or("".to_string())),
            data.descriptions.to_owned().unwrap_or(old.descriptions.unwrap_or("".to_string())),
            data.category_id.to_owned().unwrap_or(old.category_id),
            data.updated_at.to_owned(),
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(product)
    }

    async fn product_delete(
        &self,
        id: i32
    ) -> Result<u64, sqlx::Error> {
        let rows_affected: u64 = sqlx::query_file!(
            "sql/product-delete.sql",
            id
        )
        .execute(&self.pool)        
        .await
        .unwrap()
        .rows_affected();

        Ok(rows_affected)
    }
}

use async_trait::async_trait;
use sqlx::{self, Acquire};
use uuid::Uuid;

use super::{CreateProductSchema, Product, Products};
use crate::{DBClient};

#[async_trait]
pub trait ProductExt {
    async fn get_product_origin(&self, id: Uuid) -> Result<Option<Product>, sqlx::Error>;
    async fn get_product(&self, id: Uuid) -> Result<Option<Products>, sqlx::Error>;
    async fn get_products(&self, page: u32, limit: usize) -> Result<(Vec<Products>, i64), sqlx::Error>;
    async fn get_products_by_category(&self, category_id: i16, page: u32, limit: usize) -> Result<(Vec<Products>, i64), sqlx::Error>;
    async fn get_products_by_supplier(&self, supplier_id: Uuid, page: u32, limit: usize) -> Result<(Vec<Products>, i64), sqlx::Error>;
    async fn product_create(&self, data: CreateProductSchema) -> Result<Product, sqlx::Error>;
    async fn product_update(&self, id: Uuid, data: CreateProductSchema, old: Product) -> Result<Product, sqlx::Error>;
    async fn product_delete(&self, id: Uuid) -> Result<u64, sqlx::Error>;
}

#[async_trait]
impl ProductExt for DBClient {
    async fn get_product_origin(&self, id: Uuid) -> Result<Option<Product>, sqlx::Error> {
        let product = sqlx::query_file_as!(Product, "sql/product-get-origin-by-id.sql", id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(product)
    }

    async fn get_product(&self, id: Uuid) -> Result<Option<Products>, sqlx::Error> {
        let product = sqlx::query_file_as!(Products, "sql/product-get-by-id.sql", id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(product)
    }

    async fn get_products(&self, page: u32, limit: usize) -> Result<(Vec<Products>, i64), sqlx::Error> {
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

    async fn get_products_by_category(&self, category_id: i16, page: u32, limit: usize) -> Result<(Vec<Products>, i64), sqlx::Error> {
        let offset = (page - 1) * limit as u32;

        let mut conn = self.pool.acquire().await?;
        let mut tx = conn.begin().await?;

        let count = sqlx::query_scalar!("SELECT COUNT(*) FROM products WHERE category_id = $1", category_id)
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

    async fn get_products_by_supplier(&self, supplier_id: Uuid, page: u32, limit: usize) -> Result<(Vec<Products>, i64), sqlx::Error> {
        let offset = (page - 1) * limit as u32;

        let mut conn = self.pool.acquire().await?;
        let mut tx = conn.begin().await?;

        let count = sqlx::query_scalar!("SELECT COUNT(*) FROM products WHERE supplier_id = $1", supplier_id)            
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

    async fn product_create(&self, data: CreateProductSchema) -> Result<Product, sqlx::Error> {
        // let mut stmt = self.pool.prepare("SELECT * FROM users WHERE id = $1").await?;
        let product = sqlx::query_file_as!(
            Product,
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
        id: Uuid,
        data: CreateProductSchema,
        old: Product
    ) -> Result<Product, sqlx::Error> {
        let product = sqlx::query_file_as!(
            Product,
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
            data.variant_name.to_owned().unwrap_or(old.variant_name.unwrap_or("".to_string())),
            data.descriptions.to_owned().unwrap_or(old.descriptions.unwrap_or("".to_string())),
            data.category_id,
            data.supplier_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(product)
    }

    async fn product_delete(
        &self,
        id: Uuid
    ) -> Result<u64, sqlx::Error> {
        let rows_affected: u64 = sqlx::query_file!("sql/product-delete.sql", id)
        .execute(&self.pool)        
        .await
        .unwrap()
        .rows_affected();

        Ok(rows_affected)
    }
}
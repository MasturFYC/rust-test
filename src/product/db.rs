use async_trait::async_trait;
use sqlx;

use super::{CreateProductSchema, Product, ProductFull};
use crate::DBClient;

#[async_trait]
pub trait ProductExt {
   async fn get_product(&self, id: i32) -> Result<Option<ProductFull>, sqlx::Error>;
   async fn get_products(&self, page: u32, limit: usize) -> Result<Vec<ProductFull>, sqlx::Error>;
   async fn create_product(&self, data: CreateProductSchema) -> Result<Product, sqlx::Error>;

   
   // async fn save_user<T: Into<String> + Send>(
   //     &self,
   //     name: T,
   //     email: T,
   //     password: T,
   // ) -> Result<User, sqlx::Error>;
   
   // async fn user_update<T: Into<String> + Send>(
   //     &self,
   //     user_id: Option<Uuid>,
   //     name: T,
   //     email: T,
   // ) -> Result<User, sqlx::Error>;

   // #[allow(dead_code)]
   // async fn save_admin_user<T: Into<String> + Send>(
   //     &self,
   //     name: T,
   //     email: T,
   //     password: T,
   // ) -> Result<User, sqlx::Error>;
}

#[async_trait]
impl ProductExt for DBClient {
   async fn get_product(&self, id: i32) -> Result<Option<ProductFull>, sqlx::Error> {
      let product = sqlx::query_as!(
          ProductFull, 
          r#"SELECT p.*, c.name AS category_name 
          FROM products p 
          INNER JOIN categories c ON c.id = p.category_id 
          WHERE p.id = $1"#,
          id
      )
      .fetch_optional(&self.pool)
      .await?;

      Ok(product)
   }
   async fn get_products(&self, page: u32, limit: usize) -> Result<Vec<ProductFull>, sqlx::Error> {
      let offset = (page - 1) * limit as u32;

      // println!("{} {} {}", offset, page, limit);

      let products = sqlx::query_as!(
         ProductFull,
         r#"SELECT p.*, c.name AS category_name 
         FROM products p 
         INNER JOIN categories c ON c.id = p.category_id
         LIMIT $1
         OFFSET $2"#,
         limit as i64,
         offset as i64
     )
     .fetch_all(&self.pool)
     .await?;

     Ok(products)
   }


   async fn create_product(
      &self,
      data: CreateProductSchema,
  ) -> Result<Product, sqlx::Error> {
      let product = sqlx::query_as!(
         Product,
          r#"INSERT INTO products 
          (name,barcode,unit,content,hpp,margin,price,ppn,is_active,variant_name,descriptions,category_id) VALUES 
          ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12) 
          RETURNING *"#,
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
}
use crate::models::Category;
use async_trait::async_trait;
use sqlx::{Pool, Postgres};

#[derive(Debug, Clone)]
pub struct DBClient {
    pool: Pool<Postgres>,
}

impl DBClient {
    pub fn new(pool: Pool<Postgres>) -> Self {
        DBClient { pool }
    }
}

#[async_trait]
pub trait CategoryExt {
    async fn get_category(
        &self,
        id: Option<i16>,
        name: Option<&str>,
    ) -> Result<Option<Category>, sqlx::Error>;
    
    async fn get_categories(&self, page: u32, limit: usize) -> Result<Vec<Category>, sqlx::Error>;
    
    async fn category_create<T: Into<String> + Send>(
        &self,
        name: T,
    ) -> Result<Category, sqlx::Error>;
    
    async fn category_update<T: Into<String> + Send>(
        &self,
        id: Option<i16>,
        name: T,
    ) -> Result<Category, sqlx::Error>;

    async fn category_delete<T: Into<String> + Send>(
        &self,
        id: Option<i16>,
    ) -> Result<i16, sqlx::Error>;    
}

#[async_trait]
impl CategoryExt for DBClient {

    async fn get_category(
        &self,
        id: Option<i16>,
        name: Option<&str>,
    ) -> Result<Option<Category>, sqlx::Error> {
        let mut category: Option<Category> = None;

        category = sqlx::query_as!(Category, r#"SELECT id, name FROM categories WHERE id = $1"#, id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(category)
    }

    async fn get_categories(&self, page: u32, limit: usize) -> Result<Vec<Category>, sqlx::Error> {
        let offset = (page - 1) * limit as u32;

        let categories = sqlx::query_as!(
            Category,
            r#"SELECT id,name FROM categories
            LIMIT $1 OFFSET $2"#,
            limit as i64,
            offset as i64
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(categories)
    }

    async fn category_create<T: Into<String> + Send>(
        &self,
        name: T
    ) -> Result<Category, sqlx::Error> {
        let category = sqlx::query_as!(
            Category,
            r#"INSERT INTO categories (name) VALUES ($1) RETURNING id, name"#,
            name.into(),
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(category)
    }

    async fn category_update<T: Into<String> + Send>(
        &self,
        id: Option<i16>,
        name: T,
    ) -> Result<Category, sqlx::Error> {
        let category = sqlx::query_as!(
            Category,
            r#"UPDATE categories SET name = $1 WHERE id = $2 RETURNING id, name"#,
            name.into(),
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(category)
    }

    async fn category_delete<T: Into<String> + Send>(
        &self,
        id: Option<i16>,
    ) -> Result<i16, sqlx::Error> {
        let row = sqlx::query_as!(
            User,
            r#"DELETE FROM categories WHERE id = $1"#,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(1)
    }
}

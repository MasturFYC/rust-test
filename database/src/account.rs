use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

// #[serde_as]
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, Clone)]
pub struct Accounts {
    pub id: i16,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root: Option<i16>,
    pub normal: i16,
    #[serde(rename = "enName")]
    // #[serde_as(deserialize_as = "DefaultOnNull")]
    pub en_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    // #[serde_as(deserialize_as = "DefaultOnNull")]
    pub descriptions: Option<String>,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    pub payable: bool,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,    
}

#[derive(Validate, Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i16>,
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root: Option<i16>,
    pub normal: i16,
    #[serde(rename = "enName",skip_serializing_if = "Option::is_none")]
    pub en_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<String>,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    pub payable: bool,
}


pub mod db {
    use crate::db::DBClient;
    use super::*;
    use async_trait::async_trait;
    use sqlx;


    #[async_trait]
    pub trait AccountExt {
        async fn get_account(&self, id: i16) -> Result<Option<Accounts>, sqlx::Error>;
        async fn get_accounts(&self, page: u32, limit: usize) -> Result<Vec<Accounts>, sqlx::Error>;
        async fn account_create<T: Into<Account> + Send>(&self, data: T) -> Result<Option<Accounts>, sqlx::Error>;
        async fn account_update<T: Into<Account> + Send>(&self, id: i16, data: T) -> Result<Option<Accounts>, sqlx::Error>;
        async fn account_delete(&self, id: i16) -> Result<u64, sqlx::Error>;
    }
    
    #[async_trait]
    impl AccountExt for DBClient {
        async fn get_account(&self, id: i16) -> Result<Option<Accounts>, sqlx::Error> {
            let account = sqlx::query_file_as!(Accounts, "sql/account-get-by-id.sql", id)
                .fetch_optional(&self.pool)
                .await?;
            Ok(account)
        }
    
        async fn get_accounts(&self, page: u32, limit: usize) -> Result<Vec<Accounts>, sqlx::Error> {
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
    
        async fn account_create<T: Into<Account> + Send>(&self, data: T) -> Result<Option<Accounts>, sqlx::Error> {
            let t: Account = data.try_into().unwrap();// as CreateAccountSchema;
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
            let t: Account = data.try_into().unwrap();
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
            let rows_affected: u64 = sqlx::query_file!("sql/account-delete.sql", id)
                .execute(&self.pool)
                .await
                .unwrap()
                .rows_affected();
    
            Ok(rows_affected)
        }
    }
    
}
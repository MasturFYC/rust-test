use async_trait::async_trait;
use sqlx::Error;

use super::Ledger;
use crate::DBClient;
use uuid::Uuid;

#[async_trait]
pub trait LedgerExt {
    async fn get_ledger(&self, id: i16) -> Result<Option<Ledger>, Error>;
    async fn get_ledgers(&self, page: u32, limit: usize) -> Result<Vec<Ledger>, Error>;
    //    async fn account_create<T: Into<CreateAccountSchema> + Send>(&self, data: T) -> Result<Option<Account>, Error>;
    //    async fn account_update<T: Into<UpdateAccountSchema> + Send>(&self, id: i16, data: T) -> Result<Option<Account>, Error>;
    //    async fn account_delete(&self, id: i16) -> Result<u64, Error>;
}

#[async_trait]
impl AccountExt for DBClient {
    async fn get_account(&self, id: i16) -> Result<Option<Ledger>, Error> {
        let ledger = sqlx::query_file_as!(Ledger, "sql/ledger-get-by-id.sql", id,)
            .fetch_optional(&self.pool)
            .await?;
        Ok(ledger)
    }

    async fn get_ledgers(&self, page: u32, limit: usize) -> Result<Vec<Account>, Error> {
        let offset = (page - 1) * limit as u32;

        let ledgers = sqlx::query_file_as!(
            Account,
            "sql/ledger-get-all.sql",
            limit as i64,
            offset as i64
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(ledgers)
    }
    /*
    async fn account_create<T: Into<CreateAccountSchema> + Send>(&self, data: T) -> Result<Option<Account>, Error> {
        let t: CreateAccountSchema = data.try_into().unwrap();// as CreateAccountSchema;
        let account = sqlx::query_file_as!(
            Account,
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

    async fn account_update<T: Into<UpdateAccountSchema> + Send>(
        &self,
        id: i16,
        data: T,
    ) -> Result<Option<Account>, Error> {
        let t: UpdateAccountSchema = data.try_into().unwrap();
        let account = sqlx::query_file_as!(
            Account,
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

    async fn account_delete(&self, id: i16) -> Result<u64, Error> {
        let rows_affected: u64 = sqlx::query_file!("sql/account-delete.sql", id)
            .execute(&self.pool)
            .await
            .unwrap()
            .rows_affected();

        Ok(rows_affected)
    }
    */
}

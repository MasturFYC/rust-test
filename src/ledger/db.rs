use super::{CreateLedgerSchema, Ledger, LedgerDetail, LedgerWithDetails, MatchResult};
use crate::DBClient;
use async_trait::async_trait;
use sqlx::{self, types::Json, Acquire, Error};
use uuid::Uuid;

#[async_trait]
pub trait LedgerExt {
    async fn get_ledger(&self, id: Uuid) -> Result<Option<LedgerWithDetails>, Error>;
    async fn get_ledgers(&self, page: usize, limit: usize) -> Result<MatchResult, Error>;
    async fn ledger_create<T: Into<CreateLedgerSchema> + Send>(
        &self,
        data: T,
    ) -> Result<Option<Ledger>, Error>;
    async fn ledger_update<T: Into<CreateLedgerSchema> + Send>(
        &self,
        id: Uuid,
        data: T,
    ) -> Result<Option<Ledger>, Error>;
    async fn ledger_delete(&self, id: Uuid) -> Result<u64, Error>;
}

#[async_trait]
impl LedgerExt for DBClient {
    async fn get_ledger(&self, id: Uuid) -> Result<Option<LedgerWithDetails>, Error> {
        let ledger = sqlx::query_file_as!(
            LedgerWithDetails,
            "sql/ledger-get-by-id.sql",
            id,
        )
        .fetch_optional(&self.pool)
        .await?;
    
        Ok(ledger)
    }

    async fn get_ledgers(&self, page: usize, limit: usize) -> Result<MatchResult, Error> {
        let offset = (page - 1) * limit;

        // acquire pg connection from current pool
        let mut conn = self.pool.acquire().await?; //.unwrap();

        // get transaction pool from pg connection
        let mut tx = conn.begin().await?;

        // start transaction
        // get orders data from database
        let ledgers = sqlx::query_file_as!(
            Ledger,
            "sql/ledger-get-all.sql",
            limit as i64,
            offset as i64
        )
        .fetch_all(&mut *tx)
        .await?;

        // start transacrion
        // get total record of orders
        let row = sqlx::query_file_scalar!("sql/ledger-count.sql")
            .fetch_one(&mut *tx)
            .await?;

        // finish transaction
        tx.commit().await?;

        Ok((ledgers, row.unwrap_or(0)))
    }

    async fn ledger_create<T: Into<CreateLedgerSchema> + Send>(
        &self,
        data: T,
    ) -> Result<Option<Ledger>, Error> {
        let t: CreateLedgerSchema = data.try_into().unwrap();
        let ledger = sqlx::query_file_as!(
            Ledger,
            "sql/ledger-insert.sql",
            t.relation_id.to_owned(),
            t.name.to_string(),
            t.descriptions.to_owned(),
            t.updated_by.to_owned(),
            t.is_valid.to_owned(),
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(ledger)
    }

    async fn ledger_update<T: Into<CreateLedgerSchema> + Send>(
        &self,
        id: Uuid,
        data: T,
    ) -> Result<Option<Ledger>, Error> {
        let t: CreateLedgerSchema = data.try_into().unwrap();
        let ledger = sqlx::query_file_as!(
            Ledger,
            "sql/ledger-update.sql",
            id,
            t.relation_id.to_owned(),
            t.name.to_owned(),
            t.descriptions.to_owned(),
            t.updated_by.to_owned(),
            t.is_valid.to_owned(),
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(ledger)
    }

    async fn ledger_delete(&self, id: Uuid) -> Result<u64, Error> {
        let rows_affected: u64 = sqlx::query_file!("sql/ledger-delete.sql", id)
            .execute(&self.pool)
            .await
            .unwrap()
            .rows_affected();

        Ok(rows_affected)
    }
}

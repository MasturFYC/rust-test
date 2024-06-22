use async_trait::async_trait;
use sqlx;

use super::{CreateRelationSchema, Relation, RelationType};
use crate::DBClient;

#[async_trait]
pub trait RelationExt {
    async fn get_relation(&self, id: uuid::Uuid) -> Result<Option<Relation>, sqlx::Error>;
    async fn get_relations(&self, page: usize, limit: usize) -> Result<Vec<Relation>, sqlx::Error>;
    async fn get_relations_by_type(
        &self,
        page: usize,
        limit: usize,
        rels: Vec<RelationType>,
    ) -> Result<Vec<Relation>, sqlx::Error>;
    async fn relation_create<T: Into<CreateRelationSchema> + Send>(
        &self,
        data: T,
    ) -> Result<Option<Relation>, sqlx::Error>;
    async fn relation_update<T: Into<CreateRelationSchema> + Send>(
        &self,
        id: uuid::Uuid,
        data: T,
    ) -> Result<Option<Relation>, sqlx::Error>;
    async fn relation_delete(&self, id: uuid::Uuid) -> Result<u64, sqlx::Error>;
}

#[async_trait]
impl RelationExt for DBClient {
    async fn get_relation(&self, id: uuid::Uuid) -> Result<Option<Relation>, sqlx::Error> {
        let relation = sqlx::query_file_as!(Relation, "sql/relation-get-by-id.sql", id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(relation)
    }

    async fn get_relations(&self, page: usize, limit: usize) -> Result<Vec<Relation>, sqlx::Error> {
        let offset = (page - 1) * limit;
        let relations = sqlx::query_file_as!(
            Relation,
            "sql/relation-get-all.sql",
            limit as i64,
            offset as i64
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(relations)
    }

    async fn get_relations_by_type(
        &self,
        page: usize,
        limit: usize,
        rels: Vec<RelationType>,
    ) -> Result<Vec<Relation>, sqlx::Error> {
        let offset: usize = (page - 1) * limit;
        // let mut params: Vec<String>;

        let test: Vec<String> = rels
            .into_iter()
            .map(|item| item.to_str().to_string())
            .collect();

        // for i in rels {
        //     params.push(i.to_str().to_string());
        // }

        let relations = sqlx::query_file_as!(
            Relation,
            "sql/relation-get-all-by-type.sql",
            &test,
            limit as i64,
            offset as i64,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(relations)
    }

    async fn relation_create<T: Into<CreateRelationSchema> + Send>(
        &self,
        t: T,
    ) -> Result<Option<Relation>, sqlx::Error> {
        let data: CreateRelationSchema = t.try_into().unwrap();
        let test: Vec<String> = data
            .relation_type
            .into_iter()
            .map(|item| item.to_str().to_string())
            .collect();

        let relation = sqlx::query_file_as!(
            Relation,
            "sql/relation-insert.sql",
            data.name.to_owned(),
            data.city.to_owned(),
            data.street.to_owned(),
            data.phone.to_owned(),
            data.is_active.to_owned(),
            data.is_special.to_owned(),
            &test as _
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(relation)
    }

    async fn relation_update<T: Into<CreateRelationSchema> + Send>(
        &self,
        id: uuid::Uuid,
        t: T,
    ) -> Result<Option<Relation>, sqlx::Error> {
        let data: CreateRelationSchema = t.try_into().unwrap();

        let test: Vec<String> = data
            .relation_type
            .into_iter()
            .map(|item| item.to_str().to_string())
            .collect();

        let relation = sqlx::query_file_as!(
            Relation,
            "sql/relation-update.sql",
            id,
            data.name.to_owned(),
            data.city.to_owned(),
            data.street.to_owned(),
            data.phone.to_owned(),
            data.is_active.to_owned(),
            data.is_special.to_owned(),
            &test as _,
            chrono::offset::Utc::now()
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(relation)
    }
    async fn relation_delete(&self, id: uuid::Uuid) -> Result<u64, sqlx::Error> {
        let rows_affected: u64 = sqlx::query_file!("sql/relation-delete.sql", id)
            .execute(&self.pool)
            .await
            .unwrap()
            .rows_affected();

        Ok(rows_affected)
    }
}

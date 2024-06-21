use async_trait::async_trait;
use sqlx;

use super::{Relation, RelationType};
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
    // async fn relation_create(&self, data: CreateRelationSchema) -> Result<Option<Relation>, sqlx::Error>;
    // async fn relation_update(&self, id: uuid::Uuid, data: CreateRelationSchema) -> Result<Option<Relation>, sqlx::Error>;
    // async fn relation_delete(&self, id: uuid::Uuid) -> Result<u64, sqlx::Error>;
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

        let test: Vec<String> = rels.into_iter().map(|item| item.to_str().to_string()).collect();

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
}

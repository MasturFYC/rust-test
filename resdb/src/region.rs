pub mod model {
    use serde::{Deserialize, Serialize};
    use validator::Validate;

    #[derive(Validate, Debug, Deserialize, Serialize, Clone)]
    pub struct Region {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<i16>,
	#[validate(length(min = 1, message = "Name is required"))]
	pub name: String,
    }
}

pub mod db {
    use super::model::Region;
    use async_trait::async_trait;
    use crate::{db::DBClient};
    
    #[async_trait]
    pub trait RegionExt {
	
	async fn get_region(&self, id: i16) ->
	    Result<Option<Region>, sqlx::Error>;

	async fn get_regions(&self) ->
	    Result<Vec<Region>, sqlx::Error>;

	async fn region_create<T>(&self, data: T) ->
	    Result<i16, sqlx::Error>
	where T: Into<Region> + Send;
	
	async fn region_update<T>(&self, data: T) ->
	    Result<bool, sqlx::Error>
	where T: Into<Region> + Send;
	
	async fn region_delete(&self, id: i16) ->
	    Result<Option<u64>, sqlx::Error>;
    }

    #[async_trait]
    impl RegionExt for DBClient {
	async fn get_region( &self, id: i16) ->
	    Result<Option<Region>, sqlx::Error> {
		let query_result = sqlx::query_file_as!(
		    Region, "sql/region/get-by-id.sql", id)
		    .fetch_optional(&self.pool)
		    .await?;
		Ok(query_result)
	    }

	async fn get_regions(&self) -> Result<Vec<Region>, sqlx::Error> {
	    let query_result = sqlx::query_file_as!(
		Region,
		"sql/region/get-all.sql"
	    )
		.fetch_all(&self.pool)
		.await?;

	    Ok(query_result)
	}

	async fn region_create<T>(
	    &self,
	    data: T,
	) -> Result<i16, sqlx::Error>
	where
	    T: Into<Region> + Send,
	{
	    let r: Region = data.into();
	    let rec = sqlx::query_file!(
		"sql/region/insert.sql",
		r.name,
	    )
		.fetch_one(&self.pool)
		.await?;

	    Ok(rec.id)
	}


	async fn region_update<T>(
	    &self,
	    data: T,
	) -> Result<bool, sqlx::Error>
	where
	    T: Into<Region> + Send,
	{
	    let r: Region = data.into();
	    let rows_affected = sqlx::query_file!(
		"sql/region/update.sql",
		r.id,
		r.name,
	    )
		.execute(&self.pool)
		.await?
		.rows_affected();

	    Ok(rows_affected > 0)
	}

	async fn region_delete(
	    &self,
	    id: i16,
	) -> Result<Option<u64>, sqlx::Error> {
	    //print!("REGION TO BE DELETED: {}", id);
	    let rows_affected = sqlx::query_file!("sql/region/delete.sql", id)
		.execute(&self.pool)
		.await?
		.rows_affected();

	    Ok(Some(rows_affected))
	}
    }

}

pub mod db {
	use async_trait::async_trait;
	use sqlx::{self, Acquire};

	use crate::model::{CreateRelationSchema, RelationProperty, Relation, RelationType};
	use crate::db::DBClient;

	#[async_trait]
	pub trait RelationExt {
		async fn get_relation(&self, id: uuid::Uuid) -> Result<Option<Relation>, sqlx::Error>;
		async fn get_relations(
			&self,
			page: usize,
			limit: usize,
			opt: Option<i8>,
			txt: Option<String>,
			reltype: Option<RelationType>,
		) -> Result<(Vec<Relation>, i64), sqlx::Error>;
		async fn get_relations_by_type(
			&self,
			page: usize,
			limit: usize,
			rels: Vec<RelationType>,
		) -> Result<(Vec<Relation>, i64), sqlx::Error>;
		async fn get_relation_property(
			&self,
			rels: Vec<RelationType>,
		) -> Result<Vec<RelationProperty>, sqlx::Error>;
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

		async fn get_relations(
			&self,
			page: usize,
			limit: usize,
			opt: Option<i8>,
			txt: Option<String>,
			reltype: Option<RelationType>,
		) -> Result<(Vec<Relation>, i64), sqlx::Error> {
			let offset = (page - 1) * limit;
			// acquire pg connection from current pool
			let mut conn = self.pool.acquire().await?;

			// get transaction pool from pg connection
			let mut tx = conn.begin().await?;

			let op = opt.unwrap_or(0);

			let result = match op {
				1_i8 => {
					let search_text = txt
						.expect("No search text was defined")
						.trim()
						.to_lowercase();
					let relations = sqlx::query_file_as!(
						Relation,
						"sql/relation-get-search.sql",
						search_text,
						limit as i64,
						offset as i64
					)
					.fetch_all(&mut *tx)
					.await?;

					let row = sqlx::query_scalar!(
						r#"
					SELECT 
						COUNT(*)
					FROM
						relations
					WHERE 
						POSITION($1 IN LOWER(name||' '||city||' '||COALESCE(phone,' ')||' '||COALESCE(street,' '))) > 0
					"#,
						search_text
					)
					.fetch_one(&mut *tx)
					.await?
					.unwrap_or(0);
					(relations, row)
				}
				2_i8 => {
					let rel_type: [String; 1]  = [reltype.expect("No relation type was defined!").to_str().to_string()];
					let relations = sqlx::query_file_as!(
						Relation,
						"sql/relation-get-all-byone-type.sql",
						&rel_type,
						limit as i64,
						offset as i64
					)
					.fetch_all(&mut *tx)
					.await?;

					let row = sqlx::query_scalar!(
						r#"
					SELECT 
						COUNT(*)
					FROM
						relations					
					WHERE
						relation_type::TEXT[] = $1
					"#,
						&rel_type
					)
					.fetch_one(&mut *tx)
					.await?
					.unwrap_or(0);
					(relations, row)
				}
				_ => {
					let relations = sqlx::query_file_as!(
						Relation,
						"sql/relation-get-all.sql",
						limit as i64,
						offset as i64
					)
					.fetch_all(&mut *tx)
					.await?;

					let row = sqlx::query_scalar!(
						r#"
					SELECT 
						COUNT(*)
					FROM
						relations
					"#
					)
					.fetch_one(&mut *tx)
					.await?
					.unwrap_or(0);
					(relations, row)
				}
			};

			tx.commit().await?;
			Ok(result)
		}

		async fn get_relations_by_type(
			&self,
			page: usize,
			limit: usize,
			rels: Vec<RelationType>,
		) -> Result<(Vec<Relation>, i64), sqlx::Error> {
			let offset: usize = (page - 1) * limit;
			// let mut params: Vec<String>;

			let test: Vec<String> = rels
				.into_iter()
				.map(|item| item.to_str().to_string())
				.collect();

			// acquire pg connection from current pool
			let mut conn = self.pool.acquire().await?;

			// get transaction pool from pg connection
			let mut tx = conn.begin().await?;

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
			.fetch_all(&mut *tx)
			.await?;

			let row = sqlx::query_scalar!(
				r#"
            SELECT 
                COUNT(*)
            FROM
                relations
            WHERE
                relation_type::TEXT[] && $1
            "#,
				&test
			)
			.fetch_one(&mut *tx)
			.await?;

			tx.commit().await?;

			Ok((relations, row.unwrap_or(0)))
		}

		async fn get_relation_property(
			&self,
			rels: Vec<RelationType>,
		) -> Result<Vec<RelationProperty>, sqlx::Error> {
			let prop: Vec<String> = rels
				.into_iter()
				.map(|item| item.to_str().to_string())
				.collect();

			let properties = sqlx::query_as!(
				RelationProperty,
				r#"
                SELECT 
                    id, 
                    name,
                    city,
                    street,
                    phone,
                    is_special,
                    photo
                FROM 
                    relations
                WHERE 
                    relation_type::TEXT[] && $1
                    AND is_active = true
                ORDER
                    BY name
               "#,
				&prop
			)
			.fetch_all(&self.pool)
			.await?;

			Ok(properties)
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
				&test as _,
				data.photo
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
				data.photo,
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
}

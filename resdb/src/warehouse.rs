pub mod model {
    use serde::{Deserialize, Serialize};
    use sqlx::{types::Json, FromRow, Row};
    use validator::Validate;

    #[derive(Validate, Debug, Deserialize, Serialize, Clone)]
    pub struct Warehouse {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<i16>,
	#[validate(length(min = 1, message = "Name is required"))]
	pub name: String,
	#[serde(rename = "employeeId")]
	pub employee_id: i16,
	#[serde(
	    rename = "employeeName",
	    skip_serializing_if = "Option::is_none"
	)]
	pub employee_name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub locate: Option<String>,
    }

    #[derive(Debug, Deserialize, Serialize, Validate)]
    pub struct Warehouses {
	pub id: i16,
	pub name: String,
	#[serde(rename = "employeeId")]
	pub employee_id: i16,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub locate: Option<String>,
    }

    #[derive(Default, Serialize, Deserialize, Debug, FromRow, sqlx::Type)]
    pub struct WarehouseProduct {
	pub id: i32,
	pub name: String,
    }

    #[derive(Default, Serialize, Deserialize, Debug)]
    pub struct WarehouseWithProduct {
	pub id: i16,
	pub name: String,
	#[serde(rename = "employeeId")]
	pub employee_id: i16,
	#[serde(rename = "employeeName")]
	pub employee_name: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub locate: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub products: Option<Json<Vec<WarehouseProduct>>>,
    }

    impl<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> for WarehouseWithProduct {
	fn from_row(
	    row: &'r sqlx::postgres::PgRow,
	) -> Result<Self, sqlx::Error> {
	    let id = row.get("id");
	    let name = row.get("name");
	    let employee_id = row.get("employee_id");
	    let employee_name = row.get("employee_name");
	    let locate = row.get("locate");
	    // let products = row
	    //     .try_get::<sqlx::types::Json<Vec<CategoryProduct>>, _>("products")
	    //     .map(|r| if r.is_empty() {None} else { Some (r.0) })
	    //     .unwrap_or(None);
	    let products = row
		.try_get::<sqlx::types::Json<Vec<WarehouseProduct>>, _>("products")
		.map(Some)
		.unwrap();
	    // serde_json::from_value(row.get("products")).unwrap_or(None);

	    Ok(Self {
		id,
		name,
		employee_id,
		employee_name,
		locate,
		products,
	    })
	}
    }
}

pub mod db {
    use super::model::{Warehouse, WarehouseProduct, WarehouseWithProduct};
    use crate::{db::DBClient, product::model::ProductIds};
    use async_trait::async_trait;
    use sqlx::{self, types::Json, Acquire};

    #[async_trait]
    pub trait WarehouseExt {
	async fn get_warehouse(
	    &self,
	    id: i16,
	) -> Result<Option<Warehouse>, sqlx::Error>;
	async fn get_warehouses(&self) -> Result<Vec<Warehouse>, sqlx::Error>;
	async fn get_warehouse_with_products(
	    &self,
	    id: i16,
	) -> Result<Option<WarehouseWithProduct>, sqlx::Error>;
	async fn warehouse_create<T>(
	    &self,
	    data: T,
	) -> Result<i16, sqlx::Error>
	where
	    T: Into<Warehouse> + Send;
	async fn warehouse_update<T>(
	    &self,
	    id: i16,
	    data: T,
	) -> Result<u64, sqlx::Error>
	where
	    T: Into<Warehouse> + Send;
	async fn warehouse_delete(
	    &self,
	    id: i16,
	) -> Result<Option<u64>, sqlx::Error>;
    }

    #[async_trait]
    impl WarehouseExt for DBClient {
	async fn get_warehouse(
	    &self,
	    id: i16,
	) -> Result<Option<Warehouse>, sqlx::Error> {
	    let query_result = sqlx::query_file_as!(
		Warehouse, //
		"sql/warehouse/get-by-id.sql",
		id
	    )
		.fetch_optional(&self.pool)
		.await?;
	    Ok(query_result)
	}

	async fn get_warehouses(&self) -> Result<Vec<Warehouse>, sqlx::Error> {
	    let query_result = sqlx::query_file_as!(
		Warehouse,
		"sql/warehouse/get-all-order-by-name.sql"
	    )
		.fetch_all(&self.pool)
		.await?;

	    Ok(query_result)
	}

	async fn warehouse_create<T>(
	    &self,
	    data: T,
	) -> Result<i16, sqlx::Error>
	where
	    T: Into<Warehouse> + Send,
	{
	    let mut cnn = self.pool.acquire().await?;
	    let mut tx = cnn.begin().await?;
	    let g: Warehouse = data.into();
	    let products = sqlx::query_as!(
		ProductIds,
		"SELECT id FROM products ORDER BY id;"
	    )
		.fetch_all(&mut *tx)
		.await?;
	    let rec = sqlx::query_file!(
		"sql/warehouse/insert.sql",
		g.name,
		g.employee_id,
		g.locate,
	    )
		.fetch_one(&mut *tx)
		.await?;
	    let warehouse_id = rec.id;
	    let ids_length = products.len();
	    if ids_length > 0 {
		// let new_warehouse_id = rec.id;
		let mut i: usize = 0;
		loop {
		    if let Some(x) = products.get(i) {
			let _ = sqlx::query!(
			    r#"
                    INSERT INTO stocks
                        (warehouse_id, product_id, qty)
                    VALUES
                        ($1, $2, 0)
                    "#,
			    warehouse_id,
			    x.id
			)
			    .execute(&mut *tx)
			    .await?;
		    }
		    i = i.checked_add(1).unwrap();
		    if i == ids_length {
			break;
		    }
		}
	    }
	    tx.commit().await?;
	    Ok(warehouse_id)
	}

	async fn warehouse_update<T>(
	    &self,
	    id: i16,
	    data: T,
	) -> Result<u64, sqlx::Error>
	where
	    T: Into<Warehouse> + Send,
	{
	    let g: Warehouse = data.into();
	    let rows_afftected = sqlx::query_file!(
		"sql/warehouse/update.sql",
		id,
		g.name,
		g.employee_id,
		g.locate,
	    )
		.execute(&self.pool)
		.await?
		.rows_affected();

	    Ok(rows_afftected)
	}

	async fn warehouse_delete(
	    &self,
	    id: i16,
	) -> Result<Option<u64>, sqlx::Error> {
	    //print!("WAREHOUSE TO BE DELETED: {}", id);
	    let rows_affected = sqlx::query_file!("sql/warehouse/delete.sql", id)
		.execute(&self.pool)
		.await?
		.rows_affected();

	    Ok(Some(rows_affected))
	}

	async fn get_warehouse_with_products(
	    &self,
	    id: i16,
	) -> Result<Option<WarehouseWithProduct>, sqlx::Error> {
	    let query_result = sqlx::query_file_as!(
		WarehouseWithProduct,
		"sql/warehouse/with-product.sql",
		id
	    )
		.fetch_optional(&self.pool)
		.await?;
	    Ok(query_result)
	}
    }
}

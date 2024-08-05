use serde::{Deserialize, Serialize};
use sqlx::{Pool, postgres::PgPoolOptions, Postgres};

pub mod account;
pub mod category;
pub mod db;
pub mod ledger;
pub mod model;
pub mod order;
pub mod stock;
pub mod order_payment;
pub mod product;
pub mod relation;
pub mod user;

#[derive(Debug, Deserialize, Serialize)]
pub struct PageOptions {
	pub page: Option<usize>,
	pub limit: Option<usize>,
}

pub async fn load_pool(
	database_url: &String,
) -> Result<Pool<Postgres>, sqlx::Error> {
	let pool = PgPoolOptions::new()
		.max_connections(10)
		.connect(database_url)
		.await;
	pool
}

pub async fn migrate(pool: &Pool<Postgres>) {
	match sqlx::migrate!("../migrations").run(pool).await {
		Ok(_) => println!("Migrations executed successfully."),
		Err(e) => eprintln!("Error executing migrations: {}", e),
	};
}

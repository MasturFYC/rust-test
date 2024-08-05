use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde_json::json;

use crate::{dtos::RequestQueryDto, extractors::auth::RequireAuth, AppState};

use resdb::{
	model::UserRole,
	stock::db::StockExt,
	stock::model::RequestQueryStock
};

pub fn stock_scope(conf: &mut web::ServiceConfig) {
	let scope = web::scope("/api/stocks")
		.wrap(RequireAuth::allowed_roles(vec![UserRole::Admin]))
		.service(get_stock)
		.service(get_stocks)
		// .service(get_relations_by_type)
		.service(create)
		.service(update)
		.service(delete);

	conf.service(scope);
}

#[get("/{id}")]
async fn get_stock(
	path: web::Path<i32>,
	app_state: web::Data<AppState>,
) -> impl Responder {
	let stock_id = path.into_inner();
	let query_result = app_state.db_client.get_stock(stock_id).await;

	match query_result {
		Ok(stock) => {
			match stock {
				None => {
					let message =
						format!("Stock with ID: {} not found", stock_id);
					return HttpResponse::NotFound().json(json!({
						"status": "fail",
						"message": message
					}));
				}
				Some(x) => {
					let stock_response = json!({"status": "success","data": x});
					return HttpResponse::Ok().json(stock_response);
				}
			};

		}
		Err(e) => {
			let message = format!("Error {:?}", e);
			return HttpResponse::InternalServerError().json(
				serde_json::json!({"status": "fail","message": message}),
			);
		}
	}
}

#[get("")]
async fn get_stocks(
	opts: web::Query<RequestQueryDto>,
	app_state: web::Data<AppState>,
) -> impl Responder {
	let query_params = opts.into_inner();

	// let map_err = query_params
	//     .validate()
	//     .map_err(|e| e);

	// if map_err.is_err() {
	//     return HttpResponse::BadRequest().json(json!({"status":"fail", "message": "Bad request"}));
	// }

	let limit = query_params.limit.unwrap_or(10);
	let page = query_params.page.unwrap_or(1);

	let query_result = app_state.db_client.get_stocks(page, limit).await;

	if query_result.is_err() {
		let message = "Something bad happened while fetching all stock items";
		return HttpResponse::InternalServerError().json(json!({
			"status": "error",
			"message": message
		}));
	}

	// let (stocks, length) = query_result.unwrap();
	let v = query_result.unwrap();
	// selected stocks by page and limit
	let stocks = v.0;
	// count all stocks in database
	let length = v.1;
	let lim = limit as i64;

	let json_response = json!({
		"status": "success",
		"totalPages": (length / lim) + (if length % lim == 0 {0} else {1}),
		"count": stocks.len(), // count of selected stocks
		"data": stocks, // selected stocks
		"totalItems": length, // all item stocks in database
	});

	HttpResponse::Ok().json(json_response)
}

#[post("")]
async fn create(
	body: web::Json<RequestQueryStock>,
	app_state: web::Data<AppState>,
) -> impl Responder {
	let data = body.into_inner();
	let query_result = app_state
		.db_client
		.stock_create(data.stock, data.details)
		.await;

	match query_result {
		Ok(o) => {
			let stock = o.0;
			let details = o.1;
			let detail_response = json!({
				"status": "success",
				"data": json!({
					"stock" : stock,
					"details" : details
				})
			});

			// println!("{:?}", v);

			return HttpResponse::Created().json(detail_response);
		}
		Err(e) => {
			if e.to_string()
				.contains("duplicate key value violates unique constraint")
			{
				return HttpResponse::BadRequest().json(json!({
					"status": "fail",
					"message": "Note with that name already exists"
				}));
			}

			return HttpResponse::InternalServerError().json(json!({
				"status": "error",
				"message": format!("{:?}", e)
			}));
		}
	}
}

#[put("/{id}")]
async fn update(
	path: web::Path<i32>,
	body: web::Json<RequestQueryStock>,
	app_state: web::Data<AppState>,
) -> impl Responder {
	let stock_id = path.into_inner();

	let query_result = app_state.db_client.get_stock(stock_id).await;

	if query_result.is_err() {
		return HttpResponse::BadRequest()
			.json(json!({"status": "fail","message": "Bad request"}));
	}
	// let old = ; //_or(None);

	if query_result.unwrap().is_none() {
		let message = format!("Stock with ID: {} not found", stock_id);
		return HttpResponse::NotFound()
			.json(json!({"status": "fail","message": message}));
	}

	let data = body.into_inner();
	let query_result = app_state
		.db_client
		.stock_update(stock_id, data.stock, data.details)
		.await;

	match query_result {
		Ok(stock) => {
			let stock_response = json!({
			"status": "success",
			"data": json!({
				"stock": stock.0,
				"details": stock.1
			})});

			return HttpResponse::Ok().json(stock_response);
		}
		Err(err) => {
			let message = format!("Error: {:?}", err);
			return HttpResponse::InternalServerError()
				.json(json!({"status": "error","message": message}));
		}
	}
}

#[delete("/{id}")]
async fn delete(
	path: web::Path<i32>,
	app_state: web::Data<AppState>,
) -> impl Responder {
	let stock_id = path.into_inner();

	let query_result = app_state.db_client.stock_delete(stock_id).await;

	match query_result {
		Ok(rows_affected) => {
			if rows_affected == 0 {
				let message = format!("Stock with ID: {} not found", stock_id);

				return HttpResponse::NotFound().json(json!({
					"status": "fail",
					"message": message
				}));
			}

			let json = HttpResponse::Ok().json(json!({
				"status": "success",
				"data": rows_affected
			}));
			return json;
		}
		Err(err) => {
			let message = format!("Error: {:?}", err);
			return HttpResponse::InternalServerError().json(json!({
				"status": "error",
				"message": message
			}));
		}
	}
}

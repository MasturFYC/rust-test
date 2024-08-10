use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde_json::json;
use validator::Validate;

use crate::{dtos::RequestQueryDto, error::HttpError, extractors::auth::RequireAuth, AppState};

use resdb::{
	model::UserRole,
	stock::{db::StockExt, model::RequestQueryStock},
};

pub fn stock_scope(conf: &mut web::ServiceConfig) {
	let scope = web::scope("/api/stocks")
		.wrap(RequireAuth::allowed_roles(vec![UserRole::Admin]))
		.service(get_stock)
		.service(get_stock_with_details)
		.service(get_stocks)
		// .service(get_relations_by_type)
		.service(create)
		.service(update)
		.service(delete);

	conf.service(scope);
}

#[get("/{id}")]
async fn get_stock(path: web::Path<i32>, app_state: web::Data<AppState>) -> impl Responder {
	let stock_id = path.into_inner();
	let query_result = app_state.db_client.get_stock(stock_id).await;

	match query_result {
		Ok(stock) => {
			match stock {
				None => {
					let message = format!("Stock with ID: {} not found", stock_id);
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
			return HttpResponse::InternalServerError()
				.json(serde_json::json!({"status": "fail","message": message}));
		}
	}
}

#[get("/details/{id}")]
async fn get_stock_with_details(
	path: web::Path<i32>,
	app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
	let stock_id = path.into_inner();
	let result = app_state
		.db_client
		.get_stock_with_details(stock_id)
		.await
		.map_err(|e| HttpError::server_error(e.to_string()))?;

	let (stock, details) = result;

	let response = json!(
		{"status": "success",
		"stock": stock,
		"details": details
	});

	return Ok(HttpResponse::Ok().json(response));
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
		"stocks": stocks, // selected stocks
		"currentPage": page,
		"totalItems": length, // all item stocks in database
	});

	HttpResponse::Ok().json(json_response)
}

#[post("")]
async fn create(
	body: web::Json<RequestQueryStock>,
	app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
	body.validate().map_err(|e| {
		println!("{:?}", e.errors());
		HttpError::bad_request(e.to_string())
	})?;

	// println!("WELCOME");
	let data = body.into_inner();
	// let response = json!({
	// 			"status": "success",
	// 			"stock" : data,
	// 			"details" : 200
	// });
	// return Ok(HttpResponse::Created().json(response));
	let query_result = app_state
		.db_client
		.stock_create(data.stock, data.details)
		.await;

	match query_result {
		Ok(o) => {
			let stock = o.0;
			let details = o.1;
			let response = json!({
				"status": "success",
				"stock" : stock,
				"details" : details
			});

			// println!("{:?}", v);

			return Ok(HttpResponse::Created().json(response));
		}
		// Err(e) => {
		Err(e) => Err(HttpError::server_error(e.to_string())),
		// if e.to_string()
		// 	.contains("duplicate key value violates unique constraint")
		// {
		// 	println!("{:?}", e);
		// 	return Err(HttpResponse::BadRequest().json(json!({
		// 		"status": "fail",
		// 		"message": "Note with that name already exists"
		// 	})));
		// }

		// return Err(HttpResponse::InternalServerError().json(json!({
		// 	"status": "error",
		// 	"message": format!("{:?}", e)
		// })));
		// }
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
		return HttpResponse::BadRequest().json(json!({"status": "fail-1","message": "Bad request"}));
	}
	// let old = ; //_or(None);

	if query_result.unwrap().is_none() {
		let message = format!("Stock with ID: {} not found", stock_id);
		return HttpResponse::NotFound().json(json!({"status": "fail-2","message": message}));
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
			"stock": stock.0,
			"details": stock.1
			});

			return HttpResponse::Ok().json(stock_response);
		}
		Err(err) => {
			let message = format!("Error: {:?}", err);
			return HttpResponse::InternalServerError()
				.json(json!({"status": "error1","message": message}));
		}
	}
}

#[delete("")]
async fn delete(
	body: web::Json<Vec<i32>>,
	// path: web::Path<Vec<i32>>, 
	app_state: web::Data<AppState>) -> impl Responder {
	let ids = body.into_inner();

	let query_result = app_state.db_client.stock_delete(ids).await;

	match query_result {
		Ok(rows_affected) => {
			if rows_affected == 0 {
				let message = "Stock with ID: those ids  not found".to_string();

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

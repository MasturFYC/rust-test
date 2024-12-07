use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde_json::json;

use crate::{
	dtos::RequestQueryDto, extractors::auth::RequireAuth, AppState,
	error::HttpError,
};

use resdb::{
	model::{OrderDtos, RequestQueryOrderDtos, UserRole},
	order::db::OrderExt,
};

pub fn order_scope(conf: &mut web::ServiceConfig) {
	let scope = web::scope("/api/orders")
		.wrap(RequireAuth::allowed_roles(vec![UserRole::Admin]))
		.service(get_order)
		.service(get_orders)
		.service(get_order_with_details)
		// .service(get_relations_by_type)
		.service(create)
		.service(update_only_order)
		.service(update)
		.service(delete);
	conf.service(scope);
}

#[get("/{id}")]
async fn get_order(
	path: web::Path<i32>,
	app_state: web::Data<AppState>,
) -> impl Responder {
	let order_id = path.into_inner();
	let query_result = app_state.db_client.get_order(order_id).await;
	match query_result {
		Ok(order) => {
			match order {
				#![allow(non_snake_case)]
				None => {
					let message =
						format!("Order with ID: {} not found", order_id);
					HttpResponse::NotFound().json(json!({
						"status": "failed",
						"message": message
					}))
				}
				Some(x) => {
					let order_response = json!({
						"status": "success",
						"data": x
					});
					HttpResponse::Ok().json(order_response)
				}
			}
			// if order.is_none() {
			//     let message = format!("Order with ID: {} not found", order_id);
			//         return HttpResponse::NotFound()
			//             .json(serde_json::json!({"status": "failed","message": message}));
			// }
			// let order_response = serde_json::json!({"status": "success","data": order});
			// return HttpResponse::Ok().json(order_response);
		}
		Err(e) => {
			let message = format!("Error {:?}", e);
			HttpResponse::InternalServerError().json(
				serde_json::json!({"status": "failed","message": message}),
			)
		}
	}
}

#[get("/details/{id}")]
async fn get_order_with_details(
	path: web::Path<i32>,
	app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
	let order_id = path.into_inner();
	let result = app_state
		.db_client
		.get_order_with_details(order_id)
		.await
		.map_err(|e| HttpError::server_error(e.to_string()))?;
	let (stock, details) = result;
	let response = json!(
		{"status": "success",
		"stock": stock,
		"details": details
	});
	Ok(HttpResponse::Ok().json(response))
}

#[put("/update-only-order/{id}")]
async fn update_only_order(
	path: web::Path<i32>,
	body: web::Json<OrderDtos>,
	app_state: web::Data<AppState>,
) -> impl Responder {
	let order_id = path.into_inner();
	let query_result = app_state
		.db_client //
		.get_order(order_id)
		.await;
	if query_result.is_err() {
		return HttpResponse::BadRequest().json(json!({
			"status": "failed-1",
			"message": "Bad request"
		}));
	}
	// let old = ; //_or(None);
	if query_result.unwrap().is_none() {
		let message = format!("Order with ID: {} not found", order_id);
		return HttpResponse::NotFound().json(json!({
			"status": "failed-2",
			"message": message
		}));
	}

	let data = body.into_inner();
	let query_result = app_state //
		.db_client
		.order_update_only(order_id, data)
		.await;
	match query_result {
		Ok(result) => {
			let stock_response = json!({
			"status": "success",
			"id": result,
			});
			HttpResponse::Ok().json(stock_response)
		}
		Err(err) => {
			let message = format!("Error: {:?}", err);
			HttpResponse::InternalServerError().json(json!({
				"status": "error1",
				"message": message
			}))
		}
	}
}

#[get("")]
async fn get_orders(
	qp: web::Query<RequestQueryDto>,
	app_state: web::Data<AppState>,
) -> impl Responder {
	let query_params = qp.into_inner();
	let limit = query_params.limit.unwrap_or(10);
	let page = query_params.page.unwrap_or(1);
	let query_result = app_state.db_client.get_orders(page, limit).await;
	if query_result.is_err() {
		let message = "Something bad happened while fetching all order items";
		return HttpResponse::InternalServerError().json(json!({
			"status": "error",
			"message": message
		}));
	}
	// let (orders, length) = query_result.unwrap();
	let v = query_result.unwrap();
	// selected orders by page and limit
	let orders = v.0;
	// count all orders in database
	let length = v.1;
	let lim = limit as i64;
	let p: i64 = if length > lim { length / lim } else { 1 };
	let r: i64 = if length > lim { length % lim } else { 0 };
	let remain: i64 = if r > 0 { 1 } else { 0 };
	let json_response = json!({
		"status": "success",
		"totalPages": p + remain,
		"count": orders.len(), // count of selected orders
		"data": orders, // selected orders
		"currentPage": page,
		"totalItems": length, // all item orders in database
	});
	HttpResponse::Ok().json(json_response)
}

#[post("")]
async fn create(
	body: web::Json<RequestQueryOrderDtos>,
	app_state: web::Data<AppState>,
) -> impl Responder {
	let data = body.into_inner();
	// let param = OrderDtos::set_default(&data);
	let query_result = app_state
		.db_client
		.order_create(data.order, data.details)
		.await;
	match query_result {
		Ok(o) => {
			let response = json!({
				"status": "success",
				"id" : o.0,
				"length" : o.1
			});
			// println!("{:?}", v);
			HttpResponse::Created().json(response)
		}
		Err(e) => {
			if e.to_string()
				.contains("duplicate key value violates unique constraint")
			{
				return HttpResponse::BadRequest().json(json!({
					"status": "failed",
					"message": "Note with that name already exists"
				}));
			}
			HttpResponse::InternalServerError().json(json!({
				"status": "error",
				"message": format!("{:?}", e)
			}))
		}
	}
}

#[put("/{id}")]
async fn update(
	path: web::Path<i32>,
	body: web::Json<RequestQueryOrderDtos>,
	app_state: web::Data<AppState>,
) -> impl Responder {
	let order_id = path.into_inner();
	let query_result = app_state.db_client.get_order(order_id).await;
	if query_result.is_err() {
		return HttpResponse::BadRequest()
			.json(json!({"status": "failed","message": "Bad request"}));
	}
	// let old = ; //_or(None);
	if query_result.unwrap().is_none() {
		let message = format!("Order with ID: {} not found", order_id);
		return HttpResponse::NotFound()
			.json(json!({"status": "failed","message": message}));
	}
	let data = body.into_inner();
	let query_result = app_state
		.db_client
		.order_update(order_id, data.order, data.details)
		.await;
	match query_result {
		Ok(order) => {
			let order_response = json!({
			"status": "success",
			"data": {
				"order": order.0,
				"details": order.1
			}});
			HttpResponse::Ok().json(order_response)
		}
		Err(err) => {
			let message = format!("Error: {:?}", err);
			HttpResponse::InternalServerError()
				.json(json!({"status": "error","message": message}))
		}
	}
}

#[delete("/{id}")]
async fn delete(
	body: web::Json<Vec<i32>>,
	app_state: web::Data<AppState>,
) -> impl Responder {
	let ids = body.into_inner();
	let query_result = app_state.db_client.order_delete(ids).await;
	match query_result {
		Ok(rows_affected) => {
			if rows_affected == 0 {
				let message = "Order with those ids not found".to_string();
				return HttpResponse::NotFound().json(json!({
					"status": "failed",
					"message": message
				}));
			}
			HttpResponse::Ok().json(json!({
				"status": "success",
				"data": rows_affected
			}))
		}
		Err(err) => {
			let message = format!("Error: {:?}", err);
			HttpResponse::InternalServerError().json(json!({
				"status": "error",
				"message": message
			}))
		}
	}
}

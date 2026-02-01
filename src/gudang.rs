use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde_json::json;
use validator::Validate;
use crate::{error::HttpError, extractors::auth::RequireAuth, AppState};
use resdb::{
    model::UserRole,
    warehouse::{db::WarehouseExt, model::Warehouse},
};

pub fn warehouse_scope(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api/warehouses")
	.wrap(RequireAuth::allowed_roles(vec![UserRole::Admin]))
	.service(get_warehouse)
	.service(get_warehouses)
	.service(create)
	.service(update)
	.service(delete);
    conf.service(scope);
}

#[get("/{id}")]
async fn get_warehouse(
    path: web::Path<i16>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let warehouse_id = path.into_inner();
    let query_result = app_state //
	.db_client
	.get_warehouse(warehouse_id)
	.await;
    match query_result {
	Ok(warehouse) => match warehouse {
	    None => {
		let message = format!(
		    //
		    "Warehouse with ID: {} not found",
		    warehouse_id
		);
		HttpResponse::NotFound().json(json!({
		    "status": "fail",
		    "message": message
		}))
	    }
	    Some(x) => {
		let response = json!({
		    "status": "success",
		    "data": x
		});
		HttpResponse::Ok().json(response)
	    }
	},
	Err(e) => {
	    let message = format!("Error {:?}", e);
	    let response = serde_json::json!({
		"status": "fail",
		"message": message
	    });
	    HttpResponse::InternalServerError().json(response)
	}
    }
}

#[get("")]
async fn get_warehouses(app_state: web::Data<AppState>) -> impl Responder {
    let query_result = app_state
	.db_client //
	.get_warehouses()
	.await;
    if query_result.is_err() {
	let message = "Something bad happened while fetching all stock items";
	return HttpResponse::InternalServerError().json(json!({
	    "status": "error",
	    "message": message
	}));
    }
    // let (stocks, length) = query_result.unwrap();
    let data = query_result.unwrap();
    let json_response = json!({
	"status": "success",
	"data": data, // all item stocks in database
	"count": data.len()
    });

    HttpResponse::Ok().json(json_response)
}

#[post("")]
async fn create(
    body: web::Json<Warehouse>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    body.validate().map_err(|e| {
	println!("{:?}", e.errors());
	HttpError::bad_request(e.to_string())
    })?;
    let data = body.into_inner();
    let query_result = app_state //
	.db_client
	.warehouse_create(data)
	.await;
    match query_result {
	Ok(o) => {
	    let response = json!({
		"status": "success",
		"id" : o,
	    });
	    Ok(HttpResponse::Created().json(response))
	}
	Err(e) => Err(HttpError::server_error(e.to_string())),
    }
}

#[put("/{id}")]
async fn update(
    path: web::Path<i16>,
    body: web::Json<Warehouse>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let id = path.into_inner();
    let query_result = app_state //
	.db_client
	.get_warehouse(id)
	.await;
    if query_result.is_err() {
	return HttpResponse::BadRequest().json(json!({
	    "status": "fail-1",
	    "message": "Bad request"
	}));
    }
    // let old = ; //_or(None);
    if query_result.unwrap().is_none() {
	let message = format!("Stock with ID: {} not found", id);
	return HttpResponse::NotFound().json(json!({
	    "status": "fail-2",
	    "message": message
	}));
    }
    let data = body.into_inner();
    let query_result = app_state.db_client.warehouse_update(id, data).await;
    match query_result {
	Ok(result) => {
	    // let r = result.unwrap();
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

#[delete("/{id}")]
async fn delete(
    path: web::Path<i16>,
    // path: web::Path<Vec<i32>>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let id = path.into_inner();
    let query_result = app_state
	.db_client
	.warehouse_delete(id) //
	.await;
    match query_result {
	Ok(rows_affected) => {
	    let x = rows_affected.unwrap();
	    if x == 0 {
		let message = format!("Warehouse with ID: {} not found", id);

		return HttpResponse::NotFound().json(json!({
		    "status": "fail",
		    "message": message
		}));
	    }
	    HttpResponse::Ok().json(json!({
		"status": "success",
		"data": x
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

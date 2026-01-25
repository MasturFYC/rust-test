use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde_json::json;
use validator::Validate;
use crate::{error::HttpError, extractors::auth::RequireAuth, AppState};
use resdb::{
    model::UserRole,
    gudang::{db::GudangExt, model::Gudang},
};

pub fn gudang_scope(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api/gudangs")
	.wrap(RequireAuth::allowed_roles(vec![UserRole::Admin]))
	.service(get_gudang)
	.service(get_gudangs)
	.service(create)
	.service(update)
	.service(delete);
    conf.service(scope);
}

#[get("/{id}")]
async fn get_gudang(
    path: web::Path<i16>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let gudang_id = path.into_inner();
    let query_result = app_state //
	.db_client
	.get_gudang(gudang_id)
	.await;
    match query_result {
	Ok(gudang) => match gudang {
	    None => {
		let message = format!(
		    //
		    "Gudang with ID: {} not found",
		    gudang_id
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
async fn get_gudangs(app_state: web::Data<AppState>) -> impl Responder {
    let query_result = app_state
	.db_client //
	.get_gudangs()
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
    body: web::Json<Gudang>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    body.validate().map_err(|e| {
	println!("{:?}", e.errors());
	HttpError::bad_request(e.to_string())
    })?;
    let data = body.into_inner();
    let query_result = app_state //
	.db_client
	.gudang_create(data)
	.await;
    match query_result {
	Ok(o) => {
	    let r = o.unwrap();
	    let response = json!({
		"status": "success",
		"id" : r.id,
	    });
	    Ok(HttpResponse::Created().json(response))
	}
	Err(e) => Err(HttpError::server_error(e.to_string())),
    }
}

#[put("/{id}")]
async fn update(
    path: web::Path<i16>,
    body: web::Json<Gudang>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let id = path.into_inner();
    let query_result = app_state //
	.db_client
	.get_gudang(id)
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
    let query_result = app_state.db_client.gudang_update(id, data).await;
    match query_result {
	Ok(result) => {
	    let r = result.unwrap();
	    let stock_response = json!({
		"status": "success",
		"id": r.id,
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
	.gudang_delete(id) //
	.await;
    match query_result {
	Ok(rows_affected) => {
	    let x = rows_affected.unwrap();
	    if x == 0 {
		let message = format!("Gudang with ID: {} not found", id);

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

use crate::{extractors::auth::RequireAuth, AppState};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use resdb::{
    model::UserRole,
    region::{db::RegionExt, model::Region},
};
use serde_json::json;

pub fn scope(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api/regions")
    // .service(health_checker_handler)
	.wrap(RequireAuth::allowed_roles(vec![UserRole::Admin]))
	.service(get_region)
	.service(get_regions)
	.service(create)
	.service(update)
	.service(delete);
/*	.service(get_region_with_products) */
/*	.service(get_region_property) */

    conf.service(scope);
}

#[get("/{id}")]
async fn get_region(path: web::Path<i16>, app_state: web::Data<AppState>) -> impl Responder {
    let region_id = path.into_inner();
    let query_result = app_state
	.db_client
	.get_region(region_id)
	.await;
    
    match query_result {
	Ok(reg) => {
	    let note_response = json!(reg);
	    HttpResponse::Ok().json(note_response)
	}
	Err(_) => {
	    let message = format!("Region with ID: {} not found", region_id);
	    HttpResponse::NotFound().json(json!({"status": "fail","message": message}))
	}
    }
}

#[get("")]
async fn get_regions(
    // opts: web::Query<PageOptions>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    // let limit = opts.limit.unwrap_or(10);
    // let offset = (opts.page.unwrap_or(1) - 1) * limit;
    let query_result = app_state
	.db_client
	.get_regions()
	.await;
    if query_result.is_err() {
	let message = "Something bad happened while fetching all region items";
	return HttpResponse::InternalServerError()
	    .json(json!({"status": "error","message": message}));
    }
    let regs = query_result.unwrap();
    let response = json!(regs);
    HttpResponse::Ok().json(response)
}

// #[get("/property/list")]
// async fn get_region_property(app_state: web::Data<AppState>) -> impl Responder {
//     let query_result = app_state.db_client.get_region_property().await;
//     if query_result.is_err() {
// 	let message = "Something bad happened while fetching all region items";
// 	return HttpResponse::InternalServerError()
// 	    .json(json!({"status": "error","message": message}));
//     }
//     let cats = query_result.unwrap();
//     let json_response = json!({
// 	"status": "success",
// 	"count": cats.len(),
// 	"data": cats
//     });
//     HttpResponse::Ok().json(json_response)
// }

#[post("")]
async fn create(body: web::Json<Region>, app_state: web::Data<AppState>) -> impl Responder {
    let data = body.into_inner();
    let query_result = app_state
	.db_client
	.region_create(data)
	.await;
    match query_result {
	Ok(reg) => {
	    let response = json!(reg);

	    HttpResponse::Ok().json(response)
	}
	Err(e) => {
	    if e.to_string()
		.contains("duplicate key value violates unique constraint")
	    {
		return HttpResponse::BadRequest().json(
		    json!({"status": "fail","message": "Note with that title already exists"}),
		);
	    }
	    HttpResponse::InternalServerError()
		.json(json!({"status": "error","message": format!("{:?}", e)}))
	}
    }
}

#[put("/{id}")]
async fn update(
    path: web::Path<i16>,
    body: web::Json<Region>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let region_id = path.into_inner();
    let query_result = app_state.db_client.get_region(region_id).await;
    if query_result.is_err() {
	let message = format!("Region with ID: {} not found", region_id);
	return HttpResponse::NotFound()
	    .json(serde_json::json!({"status": "fail","message": message}));
    }
    let data = body.into_inner();
    let query_result = app_state
	.db_client
	.region_update(data)
	.await;
    match query_result {
	Ok(reg) => {
	    let response = serde_json::json!(reg);
	    HttpResponse::Ok().json(response)
	}
	Err(err) => {
	    if err
		.to_string()
		.contains("duplicate key value violates unique constraint")
	    {
		return HttpResponse::BadRequest().json(
		    json!({"status": "fail","message": "Note with that title already exists"}),
		);
	    }
	    HttpResponse::InternalServerError()
		.json(json!({"status": "error","message": format!("{:?}", err)}))
	}
    }
}

#[delete("/{id}")]
async fn delete(path: web::Path<i16>, app_state: web::Data<AppState>) -> impl Responder {
    let region_id = path.into_inner();
    let query_result = app_state.db_client.region_delete(region_id).await;
    if query_result.is_err() {
	let message = format!("Region with ID: {} cannot be deleted", region_id);
	return HttpResponse::BadRequest().json(json!({"status": "fail","message": message}));
    }
    let rows_affected = query_result.unwrap().unwrap();
    if rows_affected == 0 {
	let message = format!("Region with ID: {} not found", region_id);
	return HttpResponse::NotFound().json(json!({"status": "fail","message": message}));
    }
    HttpResponse::Ok().json(json!({
	"status" : "success",
	"affected": rows_affected
    }))
}

/*
#[get("/products/{id}")]
pub async fn get_region_with_products(
    path: web::Path<i16>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let cat_id = path.into_inner();

    let row = app_state.db_client.get_region_with_products(cat_id).await;

    match row {
	Ok(data) => HttpResponse::Ok().json(serde_json::json!({
	    "status": "success",
	    "data": data
	})),
	Err(e) => {
	    println!("{:?}", e);
	    let message = format!("Region with ID: {} not found", cat_id);
	    HttpResponse::NotFound().json(serde_json::json!({"status": "fail","message": message}))
	}
    }
}
*/

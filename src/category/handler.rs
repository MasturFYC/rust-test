use crate::{extractors::auth::RequireAuth, AppState};

use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use resdb::{
	model::UserRole,
	category::{db::CategoryExt, Category},
};
use serde_json::json;

pub fn category_scope(conf: &mut web::ServiceConfig) {
	let scope = web::scope("/api/categories")
		// .service(health_checker_handler)
		.wrap(RequireAuth::allowed_roles(vec![UserRole::Admin]))
		.service(get_category)
		.service(get_categories)
		.service(create)
		.service(update)
		.service(get_category_with_products)
		.service(delete);

	conf.service(scope);
}

#[get("/{id}")]
async fn get_category(
	path: web::Path<i16>,
	app_state: web::Data<AppState>,
) -> impl Responder {
	let cat_id = path.into_inner();

	let query_result = app_state.db_client.get_category(cat_id).await;

	match query_result {
		Ok(cat) => {
			let note_response = json!({"status": "success","data": serde_json::json!({
				"category": cat
			})});

			return HttpResponse::Ok().json(note_response);
		}
		Err(_) => {
			let message = format!("Category with ID: {} not found", cat_id);
			return HttpResponse::NotFound()
				.json(json!({"status": "fail","message": message}));
		}
	}
}

#[get("")]
async fn get_categories(
	// opts: web::Query<PageOptions>,
	app_state: web::Data<AppState>,
) -> impl Responder {
	// let limit = opts.limit.unwrap_or(10);
	// let offset = (opts.page.unwrap_or(1) - 1) * limit;

	let query_result = app_state.db_client.get_categories().await;

	if query_result.is_err() {
		let message =
			"Something bad happened while fetching all category items";
		return HttpResponse::InternalServerError()
			.json(json!({"status": "error","message": message}));
	}

	let cats = query_result.unwrap();

	let json_response = json!({
		"status": "success",
		"count": cats.len(),
		"data": cats
	});
	HttpResponse::Ok().json(json_response)
}

#[post("")]
async fn create(
	body: web::Json<Category>,
	app_state: web::Data<AppState>,
) -> impl Responder {
	let data = body.into_inner();

	let query_result = app_state.db_client.category_create(data).await;

	match query_result {
		Ok(cat) => {
			let cat_response = json!({"status": "success", "data": cat});

			return HttpResponse::Ok().json(cat_response);
		}
		Err(e) => {
			if e.to_string()
				.contains("duplicate key value violates unique constraint")
			{
				return HttpResponse::BadRequest().json(
					json!({"status": "fail","message": "Note with that title already exists"}),
				);
			}

			return HttpResponse::InternalServerError().json(
				json!({"status": "error","message": format!("{:?}", e)}),
			);
		}
	}
}

#[put("/{id}")]
async fn update(
	path: web::Path<i16>,
	body: web::Json<Category>,
	app_state: web::Data<AppState>,
) -> impl Responder {
	let cat_id = path.into_inner();

	let query_result = app_state.db_client.get_category(cat_id).await;

	if query_result.is_err() {
		let message = format!("Category with ID: {} not found", cat_id);
		return HttpResponse::NotFound()
			.json(serde_json::json!({"status": "fail","message": message}));
	}

	let data = body.into_inner();
	// let cat = query_result.unwrap();

	let query_result = app_state.db_client.category_update(cat_id, data).await;

	match query_result {
		Ok(cat) => {
			let cat_response = serde_json::json!({
                "status": "success",
                "data": serde_json::json!(cat)
            });

			return HttpResponse::Ok().json(cat_response);
		}
		Err(err) => {
		    if err.to_string()
				.contains("duplicate key value violates unique constraint")
			{
				return HttpResponse::BadRequest().json(
					json!({"status": "fail","message": "Note with that title already exists"}),
				);
			}

			return HttpResponse::InternalServerError().json(
				json!({"status": "error","message": format!("{:?}", err)}),
			);
		}
	}
}

#[delete("/{id}")]
async fn delete(
	path: web::Path<i16>,
	app_state: web::Data<AppState>,
) -> impl Responder {
	let cat_id = path.into_inner();
	let query_result = app_state.db_client.category_delete(cat_id).await;

    if query_result.is_err() {
		let message = format!("Category with ID: {} cannot be deleted", cat_id);
		return HttpResponse::BadRequest()
			.json(json!({"status": "fail","message": message}));

    }

	let rows_affected = query_result.unwrap().unwrap();
	if rows_affected == 0 {
		let message = format!("Category with ID: {} not found", cat_id);
		return HttpResponse::NotFound()
			.json(json!({"status": "fail","message": message}));
	}

    return HttpResponse::Ok().json(json!({
        "status" : "success",
        "affected": rows_affected
    }));

    //	HttpResponse::NoContent().finish()
}

#[get("/products/{id}")]
pub async fn get_category_with_products(
	path: web::Path<i16>,
	app_state: web::Data<AppState>,
) -> impl Responder {
	let cat_id = path.into_inner();

	let row = app_state.db_client.get_category_with_products(cat_id).await;

	match row {
		Ok(data) => {
			let json = HttpResponse::Ok().json(serde_json::json!({
				"status": "success",
				"data": data
			}));
			return json;
		}
		Err(e) => {
			println!("{:?}", e);
			let message = format!("Category with ID: {} not found", cat_id);
			return HttpResponse::NotFound().json(
				serde_json::json!({"status": "fail","message": message}),
			);
		}
	}
}
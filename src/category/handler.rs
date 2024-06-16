use crate::{
    category::{Category, CreateCategorySchema, PageOptions, UpdateCategorySchema}, extractors::auth::RequireAuth, models::UserRole, AppState
};

use actix_web::{delete, get, put, post, web, HttpResponse, Responder};
use serde_json::json;

#[get("/categories/{id}")]
async fn get_category (path: web::Path<i16>, data: web::Data<AppState>) -> impl Responder {
    let cat_id = path.into_inner();
    let query_result  = sqlx::query_file_as!(
        Category, "sql/category-get-by-id.sql", cat_id
    )
    .fetch_one(&data.db_client.pool)
    .await;

    match query_result {
        Ok(cat) => {
            let note_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "category": cat
            })});

            return HttpResponse::Ok().json(note_response);
        }
        Err(_) => {
            let message = format!("Category with ID: {} not found", cat_id);
            return HttpResponse::NotFound()
                .json(serde_json::json!({"status": "fail","message": message}));
        }
    }
}

#[get("/categories")]
async fn get_categories(opts: web::Query<PageOptions>, data: web::Data<AppState>) -> impl Responder {

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let query_result = sqlx::query_file_as!(
        Category,
        "sql/category-get-all.sql",
        limit as i64,
        offset as i64
    )
    .fetch_all(&data.db_client.pool)
    .await;

    if query_result.is_err() {
        let message = "Something bad happened while fetching all category items";
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error","message": message}));
    }

    let cats = query_result.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "results": cats.len(),
        "data": cats
    });
    HttpResponse::Ok().json(json_response)
}

#[post("/categories")]
async fn create(body: web::Json<CreateCategorySchema>, data: web::Data<AppState>) -> impl Responder {
    let query_result = sqlx::query_file_as!(
        Category,
        "sql/category-insert.sql",
        body.name.to_string()
    )
    .fetch_one(&data.db_client.pool)
    .await;

    match query_result {
        Ok(cat) => {
            let cat_response = serde_json::json!({"status": "success", "data": cat});

            return HttpResponse::Ok().json(cat_response);
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                return HttpResponse::BadRequest()
                .json(serde_json::json!({"status": "fail","message": "Note with that title already exists"}));
            }

            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
        }
    }
}

#[put("/categories/{id}")]
async fn update(path: web::Path<i16>, body: web::Json<UpdateCategorySchema>, data: web::Data<AppState>) -> impl Responder {

    let cat_id = path.into_inner();
    let query_result = sqlx::query_file_as!(
        Category, "sql/category-get-by-id.sql", cat_id)
    .fetch_one(&data.db_client.pool)
    .await;

    if query_result.is_err() {
        let message = format!("Category with ID: {} not found", cat_id);
        return HttpResponse::NotFound().json(serde_json::json!({"status": "fail","message": message}));
    }

    let cat = query_result.unwrap();

    let query_result = sqlx::query_file_as!(
        Category,
        "sql/category-update.sql",
        body.name.to_owned().unwrap_or(cat.name),
        cat_id
    )
    .fetch_one(&data.db_client.pool)
    .await;

    match query_result {
        Ok(cat) => {
            let cat_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "category": cat
            })});

            return HttpResponse::Ok().json(cat_response);
        }
        Err(err) => {
            let message = format!("Error: {:?}", err);
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": message}));
        }
    }
}

#[delete("/categories/{id}")]
async fn delete (path: web::Path<i16>, data: web::Data<AppState>) -> impl Responder {
    let cat_id = path.into_inner();
    let rows_affected = sqlx::query_file!("sql/category-delete.sql", cat_id)
    .execute(&data.db_client.pool)
    .await
    .unwrap()
    .rows_affected();

    if rows_affected == 0 {
        let message = format!("Category with ID: {} not found", cat_id);
        return HttpResponse::NotFound().json(json!({"status": "fail","message": message}));
    }

    HttpResponse::NoContent().finish()
}

pub fn category_scope(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        // .service(health_checker_handler)
        .wrap(RequireAuth::allowed_roles(vec![UserRole::Admin]))
        .service(get_category)
        .service(get_categories)
        .service(create)
        .service(update)
        .service(delete);

    conf.service(scope);
}
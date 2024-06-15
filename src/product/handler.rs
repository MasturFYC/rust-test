use crate::{
    product::Product, extractors::auth::RequireAuth, models::UserRole, AppState
};

use actix_web::{get, web, HttpResponse, Responder};
// use serde_json::json;

#[get("/products/{id}")]
async fn get_product (path: web::Path<i32>, data: web::Data<AppState>) -> impl Responder {
    let prod_id = path.into_inner();
    let query_result  = sqlx::query_as!(
        Product, 
        "SELECT * FROM products WHERE id = $1", prod_id
    )
    .fetch_one(&data.db_client.pool)
    .await;

    match query_result {
        Ok(prod) => {
            let prod_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "product": prod
            })});

            return HttpResponse::Ok().json(prod_response);
        }
        Err(_) => {
            let message = format!("Product with ID: {} not found", prod_id);
            return HttpResponse::NotFound()
                .json(serde_json::json!({"status": "fail","message": message}));
        }
    }
}

// #[get("/products")]
// async fn get_products(opts: web::Query<PageOptions>, data: web::Data<AppState>) -> impl Responder {

//     let limit = opts.limit.unwrap_or(10);
//     let offset = (opts.page.unwrap_or(1) - 1) * limit;

//     let query_result = sqlx::query_as!(
//         Product,
//         r#"SELECT * FROM products LIMIT $1 OFFSET $2"#,
//         limit as i64,
//         offset as i64
//     )
//     .fetch_all(&data.db_client.pool)
//     .await;

//     if query_result.is_err() {
//         let message = "Something bad happened while fetching all product items";
//         return HttpResponse::InternalServerError()
//             .json(json!({"status": "error","message": message}));
//     }

//     let cats = query_result.unwrap();

//     let json_response = serde_json::json!({
//         "status": "success",
//         "results": cats.len(),
//         "data": cats
//     });
//     HttpResponse::Ok().json(json_response)
// }

// #[post("/products")]
// async fn create(body: web::Json<CreateCategorySchema>, data: web::Data<AppState>) -> impl Responder {
//     let query_result = sqlx::query_as!(
//         Product,
//         "INSERT INTO products (name) VALUES ($1) RETURNING id, name",
//         body.name.to_string()
//     )
//     .fetch_one(&data.db_client.pool)
//     .await;

//     match query_result {
//         Ok(cat) => {
//             let cat_response = serde_json::json!({"status": "success", "data": cat});

//             return HttpResponse::Ok().json(cat_response);
//         }
//         Err(e) => {
//             if e.to_string()
//                 .contains("duplicate key value violates unique constraint")
//             {
//                 return HttpResponse::BadRequest()
//                 .json(serde_json::json!({"status": "fail","message": "Note with that title already exists"}));
//             }

//             return HttpResponse::InternalServerError()
//                 .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
//         }
//     }
// }

// #[put("/products/{id}")]
// async fn update(path: web::Path<i32>, body: web::Json<CreateProductSchema>, data: web::Data<AppState>) -> impl Responder {

//     let prod_id = path.into_inner();
//     let query_result = sqlx::query_as!(Product,"SELECT id, name FROM products WHERE id=$1", prod_id)
//     .fetch_one(&data.db_client.pool)
//     .await;

//     if query_result.is_err() {
//         let message = format!("Product with ID: {} not found", prod_id);
//         return HttpResponse::NotFound().json(serde_json::json!({"status": "fail","message": message}));
//     }

//     let cat = query_result.unwrap();

//     let query_result = sqlx::query_as!(
//         Product,
//         "UPDATE products SET name = $1 WHERE id = $2 RETURNING *",
//         body.name.to_owned().unwrap_or(cat.name),
//         prod_id
//     )
//     .fetch_one(&data.db_client.pool)
//     .await;

//     match query_result {
//         Ok(cat) => {
//             let cat_response = serde_json::json!({"status": "success","data": serde_json::json!({
//                 "product": cat
//             })});

//             return HttpResponse::Ok().json(cat_response);
//         }
//         Err(err) => {
//             let message = format!("Error: {:?}", err);
//             return HttpResponse::InternalServerError()
//                 .json(serde_json::json!({"status": "error","message": message}));
//         }
//     }
// }

// #[delete("/products/{id}")]
// async fn delete (path: web::Path<i32>, data: web::Data<AppState>) -> impl Responder {
//     let prod_id = path.into_inner();
//     let rows_affected = sqlx::query!("DELETE FROM products WHERE id = $1", prod_id)
//     .execute(&data.db_client.pool)
//     .await
//     .unwrap()
//     .rows_affected();

//     if rows_affected == 0 {
//         let message = format!("Product with ID: {} not found", prod_id);
//         return HttpResponse::NotFound().json(json!({"status": "fail","message": message}));
//     }

//     HttpResponse::NoContent().finish()
// }

pub fn product_scope(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        // .service(health_checker_handler)
        .wrap(RequireAuth::allowed_roles(vec![UserRole::Admin]))
        .service(get_product);
        // .service(get_products)
        // .service(create)
        // .service(update)
        // .service(delete);

    conf.service(scope);
}
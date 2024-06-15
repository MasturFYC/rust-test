use actix_web::{web, HttpResponse, Scope};
use validator::Validate;

use super::{db::ProductExt, CreateResponseDto, ProductListResponseDto, ProductResponseDto};
use crate::{
    dtos::RequestQueryDto, error::{ErrorMessage, HttpError}, product::CreateProductSchema, AppState    
};

pub fn product_scope() -> Scope {
    web::scope("/api/products")
        .route("/{id}", web::get().to(get_product))
        .route("", web::get().to(get_products))
        .route("", web::post().to(create))
        // .route("/login", web::post().to(login))
        // .route(
        //     "/logout",
        //     web::post().to(logout).wrap(RequireAuth::allowed_roles(vec![
        //         UserRole::User,
        //         UserRole::Moderator,
        //         UserRole::Admin,
        //     ])),
        // )
}

#[utoipa::path(
    get,
    path = "/api/products/{id}",
    tag = "Get product by ID endpoint",
    responses(
        (status=200, description="Get product by ID", body=ProductFull),
        (status=404, description= "Product not found", body=Response),
        (status=500, description= "Internal Server Error", body=Response ),
    )
)]
pub async fn get_product (path: web::Path<i32>, app_state: web::Data<AppState>) -> Result<HttpResponse, HttpError> {
    let prod_id = path.into_inner();

    let result = app_state
        .db_client
        .get_product(prod_id)
        .await;

    match result {
        Ok(product) => Ok(HttpResponse::Created().json(ProductResponseDto {
            status: "success".to_string(),
            data: product.unwrap(),
        })),
        Err(sqlx::Error::Database(db_err)) => {
            if db_err.is_unique_violation() {
                Err(HttpError::unique_constraint_voilation(
                    ErrorMessage::EmailExist,
                ))
            } else {
                Err(HttpError::server_error(db_err.to_string()))
            }
        }
        Err(e) => Err(HttpError::server_error(e.to_string())),
    }

}

#[utoipa::path(
    get,
    path = "/api/products",
    tag = "Get all products endpoint",
    params (RequestQueryDto),
    responses(
        (status=200, description="Get all product by page and limit", body=[ProductListResponseDto]),
        (status=500, description= "Internal Server Error", body=Response ),
    )
)]
pub async fn get_products(query: web::Query<RequestQueryDto>, app_state: web::Data<AppState>) -> Result<HttpResponse, HttpError> {

    // println!("{}", "Test");

    let query_params: RequestQueryDto = query.into_inner();

    query_params
        .validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;
        
    let page = query_params.page.unwrap_or(1);
    let limit = query_params.limit.unwrap_or(10);
    
    let products = app_state
        .db_client
        .get_products(page as u32, limit)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let length = products.len();
    Ok(HttpResponse::Ok().json(ProductListResponseDto {
        status: "success".to_string(),
        data: products,
        results: length,
    }))
}

#[utoipa::path(
    post,
    path = "/api/products",
    tag = "Create product Endpoint",
    request_body(content = CreateProductSchema, description = "Credentials to create product"), // example = json!({"email": "johndoe@example.com","name": "John Doe","password": "password123","passwordConfirm": "password123"})),
    responses(
        (status=201, description="Product created successfully", body=CreateResponseDto),
        (status=400, description="Validation Errors", body=Response),
        (status=409, description="Product with barcode already exists", body=Response),
        (status=500, description="Internal Server Error", body=Response ),
    )
)]
pub async fn create(body: web::Json<CreateProductSchema>, app_state: web::Data<AppState>) -> Result<HttpResponse, HttpError> {
    body.validate().map_err(|e| HttpError::bad_request(e.to_string()))?;

    let data = body.into_inner();

    let result = app_state
        .db_client
        .create_product(data)
        .await;

    match result {
        Ok(product) => Ok(HttpResponse::Created().json(CreateResponseDto {
            status: "success".to_string(),
            data: product,
        })),
        Err(sqlx::Error::Database(db_err)) => {
            if db_err.is_unique_violation() {
                Err(HttpError::unique_constraint_voilation(
                    ErrorMessage::EmailExist,
                ))
            } else {
                Err(HttpError::server_error(db_err.to_string()))
            }
        }
        Err(e) => Err(HttpError::server_error(e.to_string())),
    }
}

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

// pub fn product_scope(conf: &mut web::ServiceConfig) {
//     let scope = web::scope("/api")
//         // .service(health_checker_handler)
//         .wrap(RequireAuth::allowed_roles(vec![UserRole::Admin]))
//         .service(get_product);
//         // .service(get_products)
//         // .service(create)
//         // .service(update)
//         // .service(delete);

//     conf.service(scope);
// }
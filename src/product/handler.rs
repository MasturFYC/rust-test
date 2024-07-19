use actix_web::{web, HttpResponse, Scope};
use serde_json::json;
use uuid::Uuid;
use validator::Validate;
use database::product::{db::ProductExt, DeleteResponseDto, ProductResponseDto};
use database::model::UserRole;

use crate::{
    dtos::RequestQueryDto,
    error::{ErrorMessage, HttpError},
    extractors::auth::RequireAuth,
    AppState,
};

pub fn product_scope() -> Scope {
    web::scope("/api/products")
        .route("/{id}", web::get().to(get_product))
        .route("", web::get().to(get_products))
        .route(
            "",
            web::post()
                .to(create_product)
                .wrap(RequireAuth::allowed_roles(vec![
                    UserRole::Admin,
                    UserRole::Moderator,
                ])),
        )
        .route(
            "/{id}",
            web::put()
                .to(update_product)
                .wrap(RequireAuth::allowed_roles(vec![
                    UserRole::Admin,
                    UserRole::Moderator,
                ])),
        )
        .route(
            "/{id}",
            web::delete()
                .to(delete_product)
                .wrap(RequireAuth::allowed_roles(vec![UserRole::Admin])),                
        )
        .route(
            "/category/{id}",
            web::get()
                .to(get_by_category)
                .wrap(RequireAuth::allowed_roles(vec![
                    UserRole::Admin,
                    UserRole::Moderator,
                ]))
        )
        .route(
            "/supplier/{id}",
            web::get()
                .to(get_by_supplier)
                .wrap(RequireAuth::allowed_roles(vec![
                    UserRole::Admin,
                    UserRole::Moderator,
                ]))
        )
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
pub async fn get_product(
    path: web::Path<Uuid>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    let prod_id = path.into_inner();

    let result = app_state.db_client.get_product(prod_id).await;
    // .map_err(|e| HttpError::server_error(e.to_string()));

    //let product = result.ok_or(HttpError::bad_request(ErrorMessage::UserNoLongerExist));

    match result {
        Ok(product) => {
            if product.is_none() {
                Err(HttpError::new(ErrorMessage::DataNotFound, 404))
            } else {
                Ok(HttpResponse::Ok().json(ProductResponseDto {
                    status: "success".to_string(),
                    data: product.unwrap(),
                }))
            }
        }
        // Err(sqlx::Error::Database(db_err)) => {
        //     if db_err.is_unique_violation() {
        //         Err(HttpError::unique_constraint_voilation(
        //             ErrorMessage::UserNoLongerExist,
        //         ))
        //     } else {
        //         Err(HttpError::server_error(db_err.to_string()))
        //     }
        // }

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
pub async fn get_products(
    query: web::Query<RequestQueryDto>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    // println!("{}", "Test");

    let query_params: RequestQueryDto = query.into_inner();

    query_params
        .validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let page = query_params.page.unwrap_or(1);
    let limit = query_params.limit.unwrap_or(10);
    let lim = limit as i64;

    let (products, count) = app_state
        .db_client
        .get_products(page as u32, limit)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    // let length = products.len();
    let response = json!({
        "status": "success",
        "count": products.len(),
        "data": products,
        "totalItems" : count,
        "totalPages": (count / lim) + (if count % lim == 0 {0} else {1}),
    });

    Ok(HttpResponse::Ok().json(response))
}


#[utoipa::path(
    get,
    path = "/api/products/category/{id}",
    tag = "Get all products endpoint",
    params (RequestQueryDto),
    responses(
        (status=200, description="Get all product by category, page and limit", body=[ProductListResponseDto]),
        (status=500, description= "Internal Server Error", body=Response ),
    )
)]
pub async fn get_by_category(
    id: web::Path<i16>,
    query: web::Query<RequestQueryDto>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    // println!("{}", "Test");

    let query_params: RequestQueryDto = query.into_inner();
    let cat_id = id.to_owned();

    query_params
        .validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let page = query_params.page.unwrap_or(1);
    let limit = query_params.limit.unwrap_or(10);
    let lim = limit as i64;

    let (products, count) = app_state
        .db_client
        .get_products_by_category(cat_id, page as u32, limit)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    // let length = products.len();
    let response = json!({
        "status": "success",
        "count": products.len(),
        "data": products,
        "totalItems" : count,
        "totalPages": (count / lim) + (if count % lim == 0 {0} else {1}),
    });

    Ok(HttpResponse::Ok().json(response))
}


#[utoipa::path(
    get,
    path = "/api/products/supplier/{id}",
    tag = "Get all products endpoint",
    params (RequestQueryDto),
    responses(
        (status=200, description="Get all product by category, page and limit", body=[ProductListResponseDto]),
        (status=500, description= "Internal Server Error", body=Response ),
    )
)]
pub async fn get_by_supplier(
    id: web::Path<Uuid>,
    query: web::Query<RequestQueryDto>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    // println!("{}", "Test");

    let query_params: RequestQueryDto = query.into_inner();
    let sup_id = id.to_owned();

    query_params
        .validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let page = query_params.page.unwrap_or(1);
    let limit = query_params.limit.unwrap_or(10);
    let lim = limit as i64;

    let (products, count) = app_state
        .db_client
        .get_products_by_supplier(sup_id, page as u32, limit)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    // let length = products.len();
    let response = json!({
        "status": "success",
        "count": products.len(),
        "data": products,
        "totalItems" : count,
        "totalPages": (count / lim) + (if count % lim == 0 {0} else {1}),
    });

    Ok(HttpResponse::Ok().json(response))
}


#[utoipa::path(
    post,
    path = "/api/products",
    tag = "Create product Endpoint",
    request_body(content = database::product::Product, description = "Credentials to create product"), // example = json!({"email": "johndoe@example.com","name": "John Doe","password": "password123","passwordConfirm": "password123"})),
    responses(
        (status=201, description="Product created successfully", body=CreateResponseDto),
        (status=400, description="Validation Errors", body=Response),
        (status=409, description="Product with barcode already exists", body=Response),
        (status=500, description="Internal Server Error", body=Response ),
    )
)]

pub async fn create_product(
    body: web::Json<database::product::Product>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let new_product = body.into_inner();

    let result = app_state.db_client.product_create(new_product).await;

    match result {
        Ok(product) => Ok(HttpResponse::Created().json(json!({
            "status": "success".to_string(),
            "data": product,
        }))),
        // Err(sqlx::Error::Database(db_err)) => {
        //     if db_err.is_unique_violation() {
        //         Err(HttpError::unique_constraint_voilation(
        //             ErrorMessage::ProductExist,
        //         ))
        //     } else {
        //         Err(HttpError::server_error(db_err.to_string()))
        //     }
        // }
        Err(e) => Err(HttpError::server_error(e.to_string())),
    }
}

#[utoipa::path(
    put,
    path = "/api/products/{id}",
    tag = "Create product Endpoint",
    request_body(content = ProductOriginal, description = "Schema data to update product"), // example = json!({"email": "johndoe@example.com","name": "John Doe","password": "password123","passwordConfirm": "password123"})),
    responses(
        (status=201, description="Product udpated successfully", body=CreateResponseDto),
        (status=400, description="Validation Errors", body=Response),
        (status=409, description="Product with barcode already exists", body=Response),
        (status=500, description="Internal Server Error", body=Response ),
    )
)]
async fn update_product(
    path: web::Path<Uuid>,
    body: web::Json<database::product::Product>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let prod_id = path.into_inner();
    let data = body.into_inner();

    let result = app_state
        .db_client
        .get_product_origin(prod_id)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let product = result.ok_or(HttpError::bad_request(ErrorMessage::UserNoLongerExist))?;

    let result = app_state
        .db_client
        .product_update(prod_id, data, product)
        .await;

    match result {
        Ok(product) => Ok(HttpResponse::Ok().json(json!({
            "status": "success".to_string(),
            "data": product,
        }))),
        // Err(sqlx::Error::Database(db_err)) => {
        //     if db_err.is_unique_violation() {
        //         Err(HttpError::unique_constraint_voilation(
        //             ErrorMessage::ProductExist,
        //         ))
        //     } else {
        //         Err(HttpError::server_error(db_err.to_string()))
        //     }
        // }
        Err(e) => Err(HttpError::server_error(e.to_string())),
    }
}

#[utoipa::path(
    delete,
    path = "/api/products/{id}",
    tag = "Delete product Endpoint",
    responses(
        (status=201, description="Product deleted successfully", body=DeleteResponseDto),
        (status=400, description="Validation Errors", body=Response),
        (status=409, description="Product with barcode already exists", body=Response),
        (status=500, description="Internal Server Error", body=Response ),
    )
)]
async fn delete_product(
    path: web::Path<Uuid>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, HttpError> {
    let prod_id = path.into_inner();

    let result = app_state.db_client.product_delete(prod_id).await;

    match result {
        Ok(rows_affected) => Ok(HttpResponse::Ok().json(DeleteResponseDto {
            status: if rows_affected as i32 == 0 {
                "No data tobe deleted.".to_string()
            } else {
                "success".to_string()
            },
            data: rows_affected,
        })),
        // Err(sqlx::Error::Database(db_err)) => {
        //     if db_err.is_foreign_key_violation() {
        //         Err(HttpError::unique_constraint_voilation(
        //             ErrorMessage::DataCannotBeDeleted,
        //         ))
        //     } else {
        //         Err(HttpError::server_error(db_err.to_string()))
        //     }
        // }
        Err(e) => Err(HttpError::server_error(e.to_string())),
    }
}

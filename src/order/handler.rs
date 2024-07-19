use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde_json::json;

use crate::{dtos::RequestQueryDto, extractors::auth::RequireAuth, AppState};

use database::{model::{RequestQueryOrderDtos, UserRole}, order::db::OrderExt};

pub fn order_scope(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api/orders")
        .wrap(RequireAuth::allowed_roles(vec![UserRole::Admin]))
        .service(get_order)
        .service(get_orders)
        // .service(get_relations_by_type)
        .service(create)
        .service(update)
        .service(delete);

    conf.service(scope);
}

#[get("/{id}")]
async fn get_order(path: web::Path<uuid::Uuid>, app_state: web::Data<AppState>) -> impl Responder {
    let order_id = path.into_inner();
    let query_result = app_state.db_client.get_order(order_id).await;

    match query_result {
        Ok(order) => {
            match order {
                None => {
                    let message = format!("Order with ID: {} not found", order_id);
                    return HttpResponse::NotFound().json(json!({
                        "status": "fail",
                        "message": message
                    }));
                }
                Some(x) => {
                    let order_response = json!({"status": "success","data": x});
                    return HttpResponse::Ok().json(order_response);
                }
            };

            // if order.is_none() {
            //     let message = format!("Order with ID: {} not found", order_id);
            //         return HttpResponse::NotFound()
            //             .json(serde_json::json!({"status": "fail","message": message}));
            // }

            // let order_response = serde_json::json!({"status": "success","data": order});
            // return HttpResponse::Ok().json(order_response);
        }
        Err(e) => {
            let message = format!("Error {:?}", e);
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "fail","message": message}));
        }
    }
}

#[get("")]
async fn get_orders(
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

    let json_response = json!({
        "status": "success",
        "totalPages": (length / lim) + (if length % lim == 0 {0} else {1}),
        "count": orders.len(), // count of selected orders
        "data": orders, // selected orders
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
            let order = o.0;
            let details = o.1;
            let detail_response = json!({
                "status": "success",
                "data": json!({
                    "order" : order,
                    "details" : details
                })
            });

            // println!("{:?}", v);

            return HttpResponse::Created().json(detail_response);
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                return HttpResponse::BadRequest().json(json!({
                    "status": "fail",
                    "message": "Note with that name already exists"
                }));
            }

            return HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": format!("{:?}", e)
            }));
        }
    }
}

#[put("/{id}")]
async fn update(
    path: web::Path<uuid::Uuid>,
    body: web::Json<RequestQueryOrderDtos>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let order_id = path.into_inner();

    let query_result = app_state.db_client.get_order(order_id).await;

    if query_result.is_err() {
        return HttpResponse::BadRequest().json(json!({"status": "fail","message": "Bad request"}));
    }
    // let old = ; //_or(None);

    if query_result.unwrap().is_none() {
        let message = format!("Order with ID: {} not found", order_id);
        return HttpResponse::NotFound().json(json!({"status": "fail","message": message}));
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
            "data": json!({
                "account": order
            })});

            return HttpResponse::Ok().json(order_response);
        }
        Err(err) => {
            let message = format!("Error: {:?}", err);
            return HttpResponse::InternalServerError()
                .json(json!({"status": "error","message": message}));
        }
    }
}

#[delete("/{id}")]
async fn delete(path: web::Path<uuid::Uuid>, app_state: web::Data<AppState>) -> impl Responder {
    let order_id = path.into_inner();

    let query_result = app_state.db_client.order_delete(order_id).await;

    match query_result {
        Ok(rows_affected) => {
            if rows_affected == 0 {
                let message = format!("Order with ID: {} not found", order_id);

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

use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde_json::json;
// use validator::Validate;

use crate::{
    dtos::RequestQueryDto, extractors::auth::RequireAuth, models::UserRole,
    AppState,
};

use super::{CreateOrderSchema, db::OrderExt};

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
async fn get_order(
    path: web::Path<uuid::Uuid>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let order_id = path.into_inner();
    let query_result = app_state
        .db_client
        .get_order(order_id)
        .await;

    match query_result {
        Ok(order) => {

            match order {
                None => {
                    let message = format!("Order with ID: {} not found", order_id);
                    return HttpResponse::NotFound()
                        .json(serde_json::json!({"status": "fail","message": message}))

                },
                Some(x) => {
                    let order_response = serde_json::json!({"status": "success","data": x});
                    return HttpResponse::Ok().json(order_response)
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
    let query_params: RequestQueryDto = opts.into_inner();

    // let map_err = query_params
    //     .validate()
    //     .map_err(|e| e);

    // if map_err.is_err() {
    //     return HttpResponse::BadRequest().json(json!({"status":"fail", "message": "Bad request"}));
    // }

    let limit = query_params.limit.unwrap_or(10);
    let page = query_params.page.unwrap_or(1);
    

    let query_result = app_state
        .db_client
        .get_orders(page, limit)
        .await;

    if query_result.is_err() {
        let message = "Something bad happened while fetching all order items";
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error","message": message}));
    }


    let (orders, length) = query_result.unwrap();
    let lim = limit as i64;

    let json_response = serde_json::json!({
        "status": "success",
        "totalPages": (length / lim) + (if length % lim == 0 {0} else {1}),
        "count": orders.len(),
        "data": orders,
        "totalItems": length,
    });

    HttpResponse::Ok().json(json_response)
}


#[post("")]
async fn create(
    body: web::Json<CreateOrderSchema>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let data = body.into_inner();
    let query_result = app_state.db_client
        .order_create(data)
        .await;

    match query_result {
        Ok(order) => {
            if order.is_none() {
                return HttpResponse::NotAcceptable().json(
                    serde_json::json!({"status": "fail","message": "Cannot create new order"}),
                );
            }

            let acc_response = serde_json::json!({"status": "success", "data": order.unwrap()});

            return HttpResponse::Created().json(acc_response);
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                return HttpResponse::BadRequest()
                .json(serde_json::json!({"status": "fail","message": "Note with that name already exists"}));
            }

            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
        }
    }
}

#[put("/{id}")]
async fn update(
    path: web::Path<uuid::Uuid>,
    body: web::Json<CreateOrderSchema>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let order_id = path.into_inner();

    let query_result = app_state.db_client.get_order(order_id).await;

    if query_result.is_err() {
        return HttpResponse::BadRequest()
            .json(serde_json::json!({"status": "fail","message": "Bad request"}));
    }

    // let old = ; //_or(None);

    if query_result.unwrap().is_none() {
        let message = format!("Order with ID: {} not found", order_id);
        return HttpResponse::NotFound()
            .json(serde_json::json!({"status": "fail","message": message}));
    }

    let data = body.into_inner();
    let query_result = app_state
        .db_client
        .order_update(order_id, data)
        .await;

    match query_result {
        Ok(order) => {
            let order_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "account": order
            })});

            return HttpResponse::Ok().json(order_response);
        }
        Err(err) => {
            let message = format!("Error: {:?}", err);
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": message}));
        }
    }
}

#[delete("/{id}")]
async fn delete(path: web::Path<uuid::Uuid>, app_state: web::Data<AppState>) -> impl Responder {
    let order_id = path.into_inner();

    let query_result = app_state.db_client
    .order_delete(order_id)
    .await;

    match query_result {
        Ok(rows_affected) => {
            if rows_affected == 0 {
                let message = format!("Order with ID: {} not found", order_id);

                return HttpResponse::NotFound()
                    .json(serde_json::json!({"status": "fail","message": message}));
            }

            let json = HttpResponse::Ok()
                .json(serde_json::json!({"status": "success","data": rows_affected}));
            return json;
        }
        Err(err) => {
            let message = format!("Error: {:?}", err);
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": message}));
        }
    }
}

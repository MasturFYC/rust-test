use actix_web::{
    //delete,
    get,
    post,
    //put,
    web,
    HttpResponse,
    Responder,
};
// use bigdecimal::BigDecimal;
use serde_json::json;
use uuid::Uuid;
// use validator::Validate;

use crate::{
    // dtos::RequestQueryDto,
    extractors::auth::RequireAuth,
    models::UserRole,
    // order::{CreateOrderSchema, Order},
    AppState,
};

use super::{
    db::OrderDetailExt, CreateOrder, CreateOrderDetailSchema, OrderDetail, OrderDetailCreateReturn,
};

pub fn order_detail_scope(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api/order-details")
        .wrap(RequireAuth::allowed_roles(vec![UserRole::Admin]))
        .service(get_order_details)
        //  .service(get_orders)
        // .service(get_relations_by_type)
        .service(create)
        .service(creates);
    //  .service(update)
    //  .service(delete);

    conf.service(scope);
}

#[get("/{id}")]
async fn get_order_details(
    path: web::Path<uuid::Uuid>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let order_id: Uuid = path.into_inner();
    let query_result: Result<Vec<OrderDetail>, sqlx::Error> =
        app_state.db_client.get_details(order_id).await;

    if query_result.is_err() {
        let message = "Something bad happened while fetching all order detail items";
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error","message": message}));
    }

    let v: Vec<OrderDetail> = query_result.unwrap();
    let json_response = serde_json::json!({
       "status": "success",
       "count": v.len(),
       "data": v,
    });

    HttpResponse::Ok().json(json_response)
}

#[post("")]
async fn create(
    body: web::Json<CreateOrderDetailSchema>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let data = body.into_inner();
    let query_result = app_state.db_client
    .order_detail_create(data.clone())
    .await;

    match query_result {
        Ok(result) => {
            // let detail = result.0;
            if result.is_none() {
                return HttpResponse::MethodNotAllowed().json(
                    serde_json::json!({"status": "fail","message": "Cannot create new order detail"}),
                );
            }

            let v: OrderDetailCreateReturn = result.unwrap();
            // let t = result.1;

            // let now = chrono::Utc::now();

            let detail_response = json!({
               "status": "success",
               "data": json!(OrderDetail {
                  order_id: data.order_id.unwrap(),
                  id: v.id,
                  product_id: data.product_id,
                  qty: data.qty,
                  direction: data.direction,
                  unit: data.unit,
                  price: data.price,
                  discount: data.discount,
                  hpp: data.hpp,
                  created_at: v.created_at,
                  updated_at: v.updated_at,
               })
            });

            // println!("{:?}", v);

            return HttpResponse::Created().json(detail_response);
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

#[post("/order")]
async fn creates(body: web::Json<CreateOrder>, app_state: web::Data<AppState>) -> impl Responder {
    let data = body.into_inner();
    let query_result = app_state
        .db_client
        .create_order_with_details(data.order, data.details)
        .await;

    match query_result {
        Ok(o) => {
            let order = o.0.unwrap();
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
                return HttpResponse::BadRequest()
               .json(serde_json::json!({"status": "fail","message": "Note with that name already exists"}));
            }

            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
        }
    }
}

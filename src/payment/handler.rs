use crate::{dtos::RequestQueryDto, extractors::auth::RequireAuth, AppState};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use database::{model::{OrderPayment,UserRole}, order_payment::db::OrderPaymentExt};
use serde_json::json;

pub fn payment_scope(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api/payments")
        .wrap(RequireAuth::allowed_roles(vec![UserRole::Admin]))
        .service(get_payment)
        .service(get_payments)
        .service(create)
        .service(update)
        .service(delete);

    conf.service(scope);
}

#[get("/{id}")]
async fn get_payment(
    path: web::Path<uuid::Uuid>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let id = path.into_inner();
    let query_result = app_state.db_client.get_order_payment(id).await;

    match query_result {
        Ok(payment) => {
            let response = json!({
                "status": "success",
                "data": payment
            });

            return HttpResponse::Ok().json(response);
        }
        Err(_) => {
            let message = format!("Payment with ID: {} not found", id);
            return HttpResponse::NotFound().json(json!({"status": "fail","message": message}));
        }
    }
}

#[get("/{id}")]
async fn get_payments(
    opts: web::Query<RequestQueryDto>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let query_params: RequestQueryDto = opts.into_inner();

    let limit = query_params.limit.unwrap_or(10);
    let page = query_params.page.unwrap_or(1);

    let query_result = app_state.db_client.get_order_payments(page, limit).await;

    if query_result.is_err() {
        let message = "Something bad happened while fetching all order payments items";
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error","message": message}));
    }

    let (payments, count) = query_result.unwrap();
    let lim = limit as i64;

    let json_response = json!({
        "status": "success",
        "totalPages": (count / lim) + (if count % lim == 0 {0} else {1}),
        "count": payments.len(),
        "data": json!(payments),
        "totalItems": count,
    });

    HttpResponse::Ok().json(json_response)
}

#[post("")]
async fn create(body: web::Json<OrderPayment>, app_state: web::Data<AppState>) -> impl Responder {
    let data = body.into_inner();
    let query_result = app_state.db_client.order_payment_create(data).await;

    match query_result {
        Ok(o) => {
            let op = o.unwrap();
            let response = json!({
                "status": "success",
                "data": json!(op)
            });

            return HttpResponse::Created().json(response);
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
    body: web::Json<OrderPayment>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let pid = path.into_inner();
    let query_result = app_state.db_client.get_order_payment(pid).await;

    if query_result.is_err() {
        return HttpResponse::BadRequest().json(json!({"status": "fail","message": "Bad request"}));
    }
    // let old = ; //_or(None);

    if query_result.unwrap().is_none() {
        let message = format!("Order payment with ID: {} not found", pid);
        return HttpResponse::NotFound().json(json!({"status": "fail","message": message}));
    }

    let data = body.into_inner();
    let query_result = app_state.db_client.order_payment_update(pid, data).await;

    match query_result {
        Ok(payment) => {
            let order_response = json!({
            "status": "success",
            "data": json!({
                "account": payment
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
    let pid = path.into_inner();

    let query_result = app_state.db_client.order_payment_delete(pid).await;

    match query_result {
        Ok(rows_affected) => {
            if rows_affected == 0 {
                let message = format!("Order payment with ID: {} not found", pid);

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

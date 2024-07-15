use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde_json::json;

use crate::{dtos::RequestQueryDto, extractors::auth::RequireAuth, models::UserRole, AppState};

use super::{db::LedgerExt, LedgerSchema};

#[get("/{id}")]
async fn get_ledger(path: web::Path<uuid::Uuid>, app_state: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();
    let query_result = app_state.db_client.get_ledger(id).await;

    match query_result {
        Ok(ledger) => {
            let ledger_response = serde_json::json!({"status": "success","data": ledger});

            return HttpResponse::Ok().json(ledger_response);
        }
        Err(_) => {
            let message = format!("Ledger with ID: {} not found", id);
            return HttpResponse::NotFound()
                .json(serde_json::json!({"status": "fail","message": message}));
        }
    }
}

#[get("")]
async fn get_ledgers(
    opts: web::Query<RequestQueryDto>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let query_params: RequestQueryDto = opts.into_inner();

    let limit = query_params.limit.unwrap_or(10);
    let page = query_params.page.unwrap_or(1);

    let get_ledgers = app_state.db_client.get_ledgers(page, limit);
    let query_result = get_ledgers.await;

    if query_result.is_err() {
        let message = "Something bad happened while fetching all ledger items";
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error","message": message}));
    }

    //let result = query_result;//.unwrap();

    let (ledgers, count) = query_result.unwrap(); //.expect("No data found");
    // let ledgers = result.0;
    // let count = result.1;
    let lim = limit as i64;

    let json_response = json!({
        "status": "success",
        "totalPages": (count / lim) + (if count % lim == 0 {0} else {1}),
        "count": ledgers.len(), // count of selected ledgers
        "data": ledgers, // selected ledgers
        "totalItems": count, // all item ledgers in database
    });

    HttpResponse::Ok().json(json_response)
}

#[post("")]
async fn create(
    body: web::Json<LedgerSchema>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let data = body.into_inner();
    let query_result = app_state.db_client.ledger_create(data).await;

    match query_result {
        Ok(ledger) => {
            if ledger.is_none() {
                return HttpResponse::BadRequest().json(
                    serde_json::json!({"status": "fail","message": "Cannot create new ledger"}),
                );
            }

            let ledger_response = json!({"status": "success", "data": ledger});

            return HttpResponse::Created().json(ledger_response);
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
    body: web::Json<LedgerSchema>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let ledger_id = path.into_inner();

    let query_result = app_state.db_client.get_ledger(ledger_id).await;

    if query_result.is_err() {
        return HttpResponse::BadRequest()
            .json(serde_json::json!({"status": "fail","message": "Bad request"}));
    }

    let old = query_result.unwrap_or(None);

    if old.is_none() {
        let message = format!("Ledger  with ID: {} not found", ledger_id);
        return HttpResponse::NotFound()
            .json(serde_json::json!({"status": "fail","message": message}));
    }

    let data = body.into_inner();
    let query_result = app_state.db_client.ledger_update(ledger_id, data).await;

    match query_result {
        Ok(ledger) => {
            let ledger_response = serde_json::json!({
            "status": "success",
            "data": serde_json::json!({
                "ledger": ledger
            })});

            return HttpResponse::Ok().json(ledger_response);
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
    let ledger_id = path.into_inner();

    let query_result = app_state.db_client.ledger_delete(ledger_id).await;

    match query_result {
        Ok(rows_affected) => {
            if rows_affected == 0 {
                let message = format!("Ledger  with ID: {} not found", ledger_id);

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

pub fn ledger_scope(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api/ledgers")
        .wrap(RequireAuth::allowed_roles(vec![UserRole::Admin]))
        .service(get_ledger)
        .service(get_ledgers)
        .service(create)
        .service(update)
        .service(delete);
    conf.service(scope);
}
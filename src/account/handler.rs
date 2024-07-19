use crate::{extractors::auth::RequireAuth, AppState};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use resdb::{account::db::AccountExt, PageOptions, model::{UserRole, Account}};
use serde_json::json;


#[get("/{id}")]
async fn get_account(path: web::Path<i16>, app_state: web::Data<AppState>) -> impl Responder {
    let acc_id = path.into_inner();

    let query_result = app_state.db_client.get_account(acc_id).await;

    match query_result {
        Ok(acc) => {
            let message = format!("Account with ID: {} not found", acc_id);
            if acc.is_none() {
                return HttpResponse::NotFound()
                    .json(serde_json::json!({"status": "fail","message": message}));
            }

            let acc_response = serde_json::json!({
                "status": "success",
                "data": acc
            });

            return HttpResponse::Ok().json(acc_response);
        }
        Err(_) => {
            let message = format!("Account with ID: {} not found", acc_id);
            return HttpResponse::NotFound()
                .json(serde_json::json!({"status": "fail","message": message}));
        }
    }
}

#[get("")]
async fn get_accounts(
    opts: web::Query<PageOptions>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let page = opts.page.unwrap_or(1) as u32;

    let query_result = app_state.db_client.get_accounts(page, limit).await;

    if query_result.is_err() {
        let message = "Something bad happened while fetching all account items";
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error","message": message}));
    }

    let accs = query_result.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "count": accs.len(),
        "data": accs
    });
    HttpResponse::Ok().json(json_response)
}

#[post("")]
async fn create(
    body: web::Json<Account>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let data = body.into_inner();

    let query_result = app_state.db_client.account_create(data).await;

    match query_result {
        Ok(acc) => {
            if acc.is_none() {
                return HttpResponse::NotFound().json(
                    serde_json::json!({"status": "fail","message": "Cannot create new account"}),
                );
            }

            let acc_response = serde_json::json!({"status": "success", "data": acc});

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
    path: web::Path<i16>,
    body: web::Json<Account>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let acc_id = path.into_inner();

    let query_result = app_state.db_client.get_account(acc_id).await;

    if query_result.is_err() {
        return HttpResponse::BadRequest()
            .json(serde_json::json!({"status": "fail","message": "Bad request"}));
    }

    let old = query_result.unwrap_or(None);

    if old.is_none() {
        let message = format!("Account with ID: {} not found", acc_id);
        return HttpResponse::NotFound()
            .json(serde_json::json!({"status": "fail","message": message}));
    }

    let data = body.into_inner();
    let query_result = app_state.db_client.account_update(acc_id, data).await;

    match query_result {
        Ok(acc) => {
            let acc_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "account": acc
            })});

            return HttpResponse::Ok().json(acc_response);
        }
        Err(err) => {
            let message = format!("Error: {:?}", err);
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": message}));
        }
    }
}

#[delete("/{id}")]
async fn delete(path: web::Path<i16>, app_state: web::Data<AppState>) -> impl Responder {
    let acc_id = path.into_inner();

    let query_result = app_state.db_client.account_delete(acc_id).await;

    match query_result {
        Ok(rows_affected) => {
            if rows_affected == 0 {
                let message = format!("Account with ID: {} not found", acc_id);

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

pub fn account_scope(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api/accounts")
        // .service(health_checker_handler)
        .wrap(RequireAuth::allowed_roles(vec![UserRole::Admin]))
        .service(get_account)
        .service(get_accounts)
        .service(create)
        .service(update)
        // .service(get_category_with_products)
        .service(delete);

    conf.service(scope);
}

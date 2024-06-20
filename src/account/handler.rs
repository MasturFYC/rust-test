use crate::{
    account::{Account, CreateAccountSchema, UpdateAccountSchema},
    category::PageOptions,
    extractors::auth::RequireAuth,
    models::UserRole,
    AppState,
};

use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde_json::json;

#[get("/{id}")]
async fn get_account(path: web::Path<i16>, data: web::Data<AppState>) -> impl Responder {
    let acc_id = path.into_inner();
    let query_result = sqlx::query_file_as!(Account, "sql/account-get-by-id.sql", acc_id)
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
            let message = format!("Category with ID: {} not found", acc_id);
            return HttpResponse::NotFound()
                .json(serde_json::json!({"status": "fail","message": message}));
        }
    }
}

#[get("")]
async fn get_accounts(opts: web::Query<PageOptions>, data: web::Data<AppState>) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let query_result = sqlx::query_file_as!(
        Account,
        "sql/account-get-all.sql",
        limit as i64,
        offset as i64
    )
    .fetch_all(&data.db_client.pool)
    .await;

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
async fn create(body: web::Json<CreateAccountSchema>, data: web::Data<AppState>) -> impl Responder {
    let query_result =
        sqlx::query_file_as!(
            Account,
            "sql/account-insert.sql",
            body.id.to_owned(),
            body.name.to_string(),
            body.root.to_owned().unwrap_or_default(),
            body.normal.to_owned(),
            body.en_name.to_owned().unwrap_or("".to_string()),
            body.descriptions.to_owned().unwrap_or("".to_string()),            
            body.is_active.to_owned(),
            body.payable.to_owned(),
        )
        .fetch_one(&data.db_client.pool)
        .await;

    match query_result {
        Ok(acc) => {
            let acc_response = serde_json::json!({"status": "success", "data": acc});

            return HttpResponse::Ok().json(acc_response);
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

#[put("/{id}")]
async fn update(
    path: web::Path<i16>,
    body: web::Json<UpdateAccountSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let acc_id = path.into_inner();
    let query_result = sqlx::query_file_as!(Account, "sql/account-get-by-id.sql", acc_id)
        .fetch_one(&data.db_client.pool)
        .await;

    if query_result.is_err() {
        let message = format!("Account with ID: {} not found", acc_id);
        return HttpResponse::NotFound()
            .json(serde_json::json!({"status": "fail","message": message}));
    }

    let acc = query_result.unwrap();

    let query_result = sqlx::query_file_as!(
        Account,
        "sql/account-update.sql",
        acc_id,
        body.name.to_owned(),
        body.root.to_owned().unwrap_or(acc.root.unwrap_or_default()),
        body.normal.to_owned(),
        body.en_name.to_owned().unwrap_or(acc.en_name.unwrap_or("".to_string())),
        body.descriptions.to_owned().unwrap_or(acc.descriptions.unwrap_or("".to_string())),
        body.is_active.to_owned(),
        body.payable.to_owned(),        
    )
    .fetch_one(&data.db_client.pool)
    .await;

    match query_result {
        Ok(acc) => {
            let cat_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "account": acc
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

#[delete("/{id}")]
async fn delete(path: web::Path<i16>, data: web::Data<AppState>) -> impl Responder {
    let acc_id = path.into_inner();
    let rows_affected = sqlx::query_file!("sql/account-delete.sql", acc_id)
        .execute(&data.db_client.pool)
        .await
        .unwrap()
        .rows_affected();

    if rows_affected == 0 {
        let message = format!("Account with ID: {} not found", acc_id);
        return HttpResponse::NotFound().json(json!({"status": "fail","message": message}));
    }

    HttpResponse::NoContent().finish()
}

// #[get("/products/{id}")]
// pub async fn get_category_with_products(
//     path: web::Path<i16>,
//     data: web::Data<AppState>,
// ) -> impl Responder {
//     let cat_id = path.into_inner();
//     let row = sqlx::query_file_as!(CategoryWithProduct, "sql/category-with-product.sql", cat_id)
//         .fetch_one(&data.db_client.pool)
//         .await;

//     match row {
//         Ok(data) => {
//             let json = HttpResponse::Ok().json(serde_json::json!({
//                 "status": "success",
//                 "data": data
//             }));
//             return json;
//         }
//         Err(e) => {
//             println!("{:?}", e);
//             let message = format!("Category with ID: {} not found", cat_id);
//             return HttpResponse::NotFound()
//                 .json(serde_json::json!({"status": "fail","message": message}));
//         }
//     }
// }

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

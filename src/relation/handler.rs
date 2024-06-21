use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde_json::json;
// use validator::Validate;

use crate::{
    dtos::RequestQueryDto, extractors::auth::RequireAuth, models::UserRole,
    relation::db::RelationExt, AppState,
};

use super::{CreateRelationSchema, RelationType};

pub fn relation_scope(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api/relations")
        .wrap(RequireAuth::allowed_roles(vec![UserRole::Admin]))
        .service(get_relation)
        .service(get_relations)
        .service(get_relations_by_type)
        .service(create)
        .service(update)
        .service(delete);

    conf.service(scope);
}

#[get("/{id}")]
async fn get_relation(
    path: web::Path<uuid::Uuid>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let rel_id = path.into_inner();
    let query_result = app_state.db_client.get_relation(rel_id).await;

    match query_result {
        Ok(relation) => {
            let relation_response = serde_json::json!({"status": "success","data": relation});

            return HttpResponse::Ok().json(relation_response);
        }
        Err(_) => {
            let message = format!("Category with ID: {} not found", rel_id);
            return HttpResponse::NotFound()
                .json(serde_json::json!({"status": "fail","message": message}));
        }
    }
}

#[get("")]
async fn get_relations(
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

    let query_result = app_state.db_client.get_relations(page, limit).await;

    if query_result.is_err() {
        let message = "Something bad happened while fetching all relation items";
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error","message": message}));
    }

    let rels = query_result.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "count": rels.len(),
        "data": rels
    });
    HttpResponse::Ok().json(json_response)
}

#[post("/type")]
async fn get_relations_by_type(
    opts: web::Query<RequestQueryDto>,
    body: web::Json<Vec<RelationType>>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let query_params: RequestQueryDto = opts.into_inner();

    // let map_err = query_params
    //     .validate()
    //     .map_err(|e| e);

    // if map_err.is_err() {
    //     return HttpResponse::BadRequest().json(json!({"status":"fail", "message": "Bad request"}));
    // }

    let rels = body.into_inner();

    let limit = query_params.limit.unwrap_or(10);
    let page = query_params.page.unwrap_or(1);

    let query_result = app_state
        .db_client
        .get_relations_by_type(page, limit, rels)
        .await;

    if query_result.is_err() {
        let message = "Something bad happened while fetching all relation items";
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error","message": message}));
    }

    let rels = query_result.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "count": rels.len(),
        "data": rels
    });
    HttpResponse::Ok().json(json_response)
}

#[post("")]
async fn create(
    body: web::Json<CreateRelationSchema>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let data = body.into_inner();
    let query_result = app_state.db_client.relation_create(data).await;

    match query_result {
        Ok(rel) => {
            if rel.is_none() {
                return HttpResponse::BadRequest().json(
                    serde_json::json!({"status": "fail","message": "Cannot create new account"}),
                );
            }

            let acc_response = serde_json::json!({"status": "success", "data": rel.unwrap()});

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
    body: web::Json<CreateRelationSchema>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let rel_id = path.into_inner();

    let query_result = app_state.db_client.get_relation(rel_id).await;

    if query_result.is_err() {
        return HttpResponse::BadRequest()
            .json(serde_json::json!({"status": "fail","message": "Bad request"}));
    }

    let old = query_result.unwrap_or(None);

    if old.is_none() {
        let message = format!("Relation with ID: {} not found", rel_id);
        return HttpResponse::NotFound()
            .json(serde_json::json!({"status": "fail","message": message}));
    }

    let data = body.into_inner();
    let query_result = app_state.db_client.relation_update(rel_id, data).await;

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
async fn delete(path: web::Path<uuid::Uuid>, app_state: web::Data<AppState>) -> impl Responder {
    let rel_id = path.into_inner();

    let query_result = app_state.db_client.relation_delete(rel_id).await;

    match query_result {
        Ok(rows_affected) => {
            if rows_affected == 0 {
                let message = format!("Relation with ID: {} not found", rel_id);

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

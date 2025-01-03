mod config;
// mod db;
mod dtos;
mod error;
mod extractors;
// mod models;
mod account;
mod category;
mod gudang;
mod ledger;
mod order;
mod payment;
mod product;
mod relation;
mod scopes;
mod stock;
mod utils;

use actix_cors::Cors;
use actix_web::{
    get, http::header, middleware::Logger, web, App, HttpResponse, HttpServer, Responder,
};

use config::Config;
use dotenv::dotenv;
use dtos::{
    FilterUserDto, LoginUserDto, RegisterUserDto, Response, UserData, UserListResponseDto,
    UserLoginResponseDto, UserResponseDto,
};
use resdb::db::DBClient;

use scopes::{auth, users};
// use sqlx::postgres::PgPoolOptions;
use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

// use product::handler;

#[derive(Debug, Clone)]
pub struct AppState {
    pub env: Config,
    pub db_client: DBClient,
}

#[derive(OpenApi)]
#[openapi(
	paths(
		auth::login, auth::logout, auth::register,
		users::get_me, users::get_users,
		health_checker_handler,
		product::handler::create_product,
		product::handler::get_product,
		product::handler::get_products,
	),
	components(
		schemas(
			UserData,FilterUserDto,LoginUserDto,RegisterUserDto,UserResponseDto,
			UserLoginResponseDto,Response,UserListResponseDto
		)
	),
	tags(
		(name = "Rust REST API", description = "Authentication in Rust Endpoints")
	),
	modifiers(&SecurityAddon)
)]
struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "token",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        )
    }
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    openssl_probe::init_ssl_cert_env_vars();
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    dotenv().ok();
    env_logger::init();

    let config = Config::init();

    // let pool = PgPoolOptions::new()
    //     .max_connections(10)
    //     .connect(&config.database_url)
    //     .await?;
    let pool = resdb::load_pool(&config.database_url).await?;
    resdb::migrate(&pool).await;

    // match sqlx::migrate!("./migrations").run(&pool).await {
    //     Ok(_) => println!("Migrations executed successfully."),
    //     Err(e) => eprintln!("Error executing migrations: {}", e),
    // };

    let db_client = DBClient::new(pool);
    let app_state: AppState = AppState {
        env: config.clone(),
        db_client,
    };

    println!(
        "{}",
        format_args!("Server is running on http://localhost:{}", config.port)
    );

    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        let cors = Cors::default()
            //			.allowed_origin("http://localhost:3000")
            .allowed_origin("http://localhost:8000")
            //			.allowed_origin("http://localhost:8080")
            .allowed_origin("https://arch.sapulidi.site")
            //			.allowed_origin("http://localhost")
            .allowed_origin("http://localhost:5173")
            .allowed_origin("http://localhost")
            .allowed_origin("http://127.0.0.1:8080")
            //			.allowed_origin("https://rust.codevoweb.com")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();

        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(cors)
            .wrap(Logger::default())
            .service(product::handler::product_scope())
            .service(scopes::auth::auth_scope())
            .service(scopes::users::users_scope())
            .service(health_checker_handler)
            .configure(category::handler::category_scope)
            .configure(account::handler::account_scope)
            .configure(relation::handler::relation_scope)
            .configure(order::handler::order_scope)
            .configure(stock::stock_scope)
            .configure(gudang::gudang_scope)
            .configure(ledger::handler::ledger_scope)
            .configure(payment::handler::payment_scope)
            .service(Redoc::with_url("/redoc", openapi.clone()))
            .service(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
            .service(SwaggerUi::new("/{_:.*}").url("/api-docs/openapi.json", openapi.clone()))
    })
    .bind(("0.0.0.0", config.port))?
    .run()
    .await?;

    Ok(())
}

#[utoipa::path(
    get,
    path = "/api/healthchecker",
    tag = "Health Checker Endpoint",
    responses(
        (status = 200, description= "Authenticated User", body = Response),
    )
)]
#[get("/api/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Complete Restful API in Rust";

    HttpResponse::Ok().json(serde_json::json!({"status": "success", "message": MESSAGE}))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::StatusCode, test, App};

    #[actix_web::test]
    async fn test_health_checker_handler() {
        let app = test::init_service(App::new().service(health_checker_handler)).await;

        let req = test::TestRequest::get()
            .uri("/api/healthchecker")
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);

        let body = test::read_body(resp).await;
        let expected_json =
            serde_json::json!({"status": "success", "message": "Complete Restful API in Rust"});

        assert_eq!(body, serde_json::to_string(&expected_json).unwrap());
    }
}

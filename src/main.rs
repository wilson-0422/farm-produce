mod config;
mod handlers;
mod middleware;
mod models;
mod services;

use std::sync::Arc;
use axum::{Router, routing::{get, post}};
use tower_http::services::ServeDir;

use config::app::AppState;

#[tokio::main]
async fn main() {
    let conn = config::database::init_db();
    config::seed::seed_data(&conn);

    let tera = tera::Tera::new("templates/**/*").expect("模板初始化失败");

    let state = Arc::new(AppState {
        db: std::sync::Mutex::new(conn),
        tera,
    });

    let app = Router::new()
        .route("/", get(handlers::home::index))
        .route("/dashboard", get(handlers::home::dashboard))
        .route("/auth/login", get(handlers::auth::login_page).post(handlers::auth::login))
        .route("/auth/register", get(handlers::auth::register_page).post(handlers::auth::register))
        .route("/auth/logout", get(handlers::auth::logout))
        .route("/harvests", get(handlers::harvest::list).post(handlers::harvest::create))
        .route("/harvests/create", get(handlers::harvest::create_page))
        .route("/harvests/:id", get(handlers::harvest::detail))
        .route("/harvests/:id/edit", get(handlers::harvest::edit_page).post(handlers::harvest::edit))
        .route("/warehouse", get(handlers::warehouse::list))
        .route("/warehouse/intake", get(handlers::warehouse::intake_page).post(handlers::warehouse::intake))
        .route("/warehouse/:id", get(handlers::warehouse::detail))
        .route("/distributions", get(handlers::distribution::list).post(handlers::distribution::create))
        .route("/distributions/create", get(handlers::distribution::create_page))
        .route("/distributions/:id", get(handlers::distribution::detail))
        .route("/logistics", get(handlers::logistics::list).post(handlers::logistics::create))
        .route("/logistics/track", get(handlers::logistics::track_page))
        .route("/logistics/:id", get(handlers::logistics::detail))
        .route("/settlements", get(handlers::settlement::list).post(handlers::settlement::create))
        .route("/settlements/create", get(handlers::settlement::create_page))
        .route("/settlements/:id", get(handlers::settlement::detail))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("产地农产品产销协同平台已启动: http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}

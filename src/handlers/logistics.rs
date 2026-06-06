use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse, Redirect},
    Form,
};
use axum_extra::extract::cookie::CookieJar;
use std::sync::Arc;
use tera::Context;

use crate::config::app::AppState;
use crate::middleware::auth::get_current_user;
use crate::models::logistics::LogisticsForm;
use crate::services::{distribution_service, logistics_service};

pub async fn list(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
) -> impl IntoResponse {
    let user = get_current_user(&jar, &state);
    let db = state.db.lock().unwrap();
    let entries = logistics_service::list(&db).unwrap_or_default();
    drop(db);
    let mut ctx = Context::new();
    ctx.insert("logistics", &entries);
    ctx.insert("user", &user);
    ctx.insert("current_page", &"logistics");
    let html = state.tera.render("logistics/list.html", &ctx).unwrap();
    Html(html)
}

pub async fn detail(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let user = get_current_user(&jar, &state);
    let db = state.db.lock().unwrap();
    let entry = logistics_service::find(&db, id);
    drop(db);
    match entry {
        Ok(e) => {
            let mut ctx = Context::new();
            ctx.insert("logistic", &e);
            ctx.insert("user", &user);
            ctx.insert("current_page", &"logistics");
            let html = state.tera.render("logistics/detail.html", &ctx).unwrap();
            Html(html).into_response()
        }
        Err(_) => Redirect::to("/logistics").into_response(),
    }
}

pub async fn track_page(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
) -> impl IntoResponse {
    let user = get_current_user(&jar, &state);
    let db = state.db.lock().unwrap();
    let entries = logistics_service::list(&db).unwrap_or_default();
    drop(db);
    let mut ctx = Context::new();
    ctx.insert("logistics", &entries);
    ctx.insert("user", &user);
    ctx.insert("current_page", &"logistics");
    let html = state.tera.render("logistics/track.html", &ctx).unwrap();
    Html(html)
}

pub async fn create(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    Form(form): Form<LogisticsForm>,
) -> impl IntoResponse {
    let user = get_current_user(&jar, &state);
    if user.is_none() {
        return Redirect::to("/auth/login").into_response();
    }
    let db = state.db.lock().unwrap();
    match logistics_service::create(&db, &form) {
        Ok(_) => {
            drop(db);
            Redirect::to("/logistics").into_response()
        }
        Err(msg) => {
            let distributions = distribution_service::list(&db).unwrap_or_default();
            drop(db);
            let mut ctx = Context::new();
            ctx.insert("error", &msg);
            ctx.insert("distributions", &distributions);
            ctx.insert("user", &user);
            ctx.insert("current_page", &"logistics");
            let html = state.tera.render("logistics/track.html", &ctx).unwrap();
            Html(html).into_response()
        }
    }
}

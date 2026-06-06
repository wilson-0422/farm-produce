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
use crate::models::warehouse::WarehouseIntakeForm;
use crate::services::{harvest_service, warehouse_service};

pub async fn list(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
) -> impl IntoResponse {
    let user = get_current_user(&jar, &state);
    let db = state.db.lock().unwrap();
    let entries = warehouse_service::list(&db).unwrap_or_default();
    drop(db);
    let mut ctx = Context::new();
    ctx.insert("entries", &entries);
    ctx.insert("user", &user);
    ctx.insert("current_page", &"warehouse");
    let html = state.tera.render("warehouse/list.html", &ctx).unwrap();
    Html(html)
}

pub async fn detail(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let user = get_current_user(&jar, &state);
    let db = state.db.lock().unwrap();
    let entry = warehouse_service::find(&db, id);
    drop(db);
    match entry {
        Ok(e) => {
            let mut ctx = Context::new();
            ctx.insert("entry", &e);
            ctx.insert("user", &user);
            ctx.insert("current_page", &"warehouse");
            let html = state.tera.render("warehouse/detail.html", &ctx).unwrap();
            Html(html).into_response()
        }
        Err(_) => Redirect::to("/warehouse").into_response(),
    }
}

pub async fn intake_page(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
) -> impl IntoResponse {
    let user = get_current_user(&jar, &state);
    if user.is_none() {
        return Redirect::to("/auth/login").into_response();
    }
    let db = state.db.lock().unwrap();
    let harvests = harvest_service::list(&db).unwrap_or_default();
    drop(db);
    let mut ctx = Context::new();
    ctx.insert("harvests", &harvests);
    ctx.insert("user", &user);
    ctx.insert("current_page", &"warehouse");
    let html = state.tera.render("warehouse/intake.html", &ctx).unwrap();
    Html(html).into_response()
}

pub async fn intake(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    Form(form): Form<WarehouseIntakeForm>,
) -> impl IntoResponse {
    let user = get_current_user(&jar, &state);
    let user = match user {
        Some(u) => u,
        None => return Redirect::to("/auth/login").into_response(),
    };
    let db = state.db.lock().unwrap();
    match warehouse_service::create(&db, user.id, &form) {
        Ok(_) => {
            drop(db);
            Redirect::to("/warehouse").into_response()
        }
        Err(msg) => {
            let harvests = harvest_service::list(&db).unwrap_or_default();
            drop(db);
            let mut ctx = Context::new();
            ctx.insert("error", &msg);
            ctx.insert("harvests", &harvests);
            ctx.insert("user", &Some(user));
            ctx.insert("current_page", &"warehouse");
            let html = state.tera.render("warehouse/intake.html", &ctx).unwrap();
            Html(html).into_response()
        }
    }
}

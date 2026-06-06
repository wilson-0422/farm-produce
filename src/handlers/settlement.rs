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
use crate::models::settlement::SettlementForm;
use crate::services::{distribution_service, harvest_service, settlement_service};

pub async fn list(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
) -> impl IntoResponse {
    let user = get_current_user(&jar, &state);
    let db = state.db.lock().unwrap();
    let settlements = settlement_service::list(&db).unwrap_or_default();
    drop(db);
    let mut ctx = Context::new();
    ctx.insert("settlements", &settlements);
    ctx.insert("user", &user);
    ctx.insert("current_page", &"settlements");
    let html = state.tera.render("settlements/list.html", &ctx).unwrap();
    Html(html)
}

pub async fn detail(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let user = get_current_user(&jar, &state);
    let db = state.db.lock().unwrap();
    let settlement = settlement_service::find(&db, id);
    drop(db);
    match settlement {
        Ok(s) => {
            let mut ctx = Context::new();
            ctx.insert("settlement", &s);
            ctx.insert("user", &user);
            ctx.insert("current_page", &"settlements");
            let html = state.tera.render("settlements/detail.html", &ctx).unwrap();
            Html(html).into_response()
        }
        Err(_) => Redirect::to("/settlements").into_response(),
    }
}

pub async fn create_page(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
) -> impl IntoResponse {
    let user = get_current_user(&jar, &state);
    if user.is_none() {
        return Redirect::to("/auth/login").into_response();
    }
    let db = state.db.lock().unwrap();
    let distributions = distribution_service::list(&db).unwrap_or_default();
    let harvests = harvest_service::list(&db).unwrap_or_default();
    drop(db);
    let mut ctx = Context::new();
    ctx.insert("distributions", &distributions);
    ctx.insert("harvests", &harvests);
    ctx.insert("user", &user);
    ctx.insert("current_page", &"settlements");
    let html = state.tera.render("settlements/create.html", &ctx).unwrap();
    Html(html).into_response()
}

pub async fn create(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    Form(form): Form<SettlementForm>,
) -> impl IntoResponse {
    let user = get_current_user(&jar, &state);
    if user.is_none() {
        return Redirect::to("/auth/login").into_response();
    }
    let db = state.db.lock().unwrap();
    match settlement_service::create(&db, &form) {
        Ok(_) => {
            drop(db);
            Redirect::to("/settlements").into_response()
        }
        Err(msg) => {
            let distributions = distribution_service::list(&db).unwrap_or_default();
            let harvests = harvest_service::list(&db).unwrap_or_default();
            drop(db);
            let mut ctx = Context::new();
            ctx.insert("error", &msg);
            ctx.insert("distributions", &distributions);
            ctx.insert("harvests", &harvests);
            ctx.insert("user", &user);
            ctx.insert("current_page", &"settlements");
            let html = state.tera.render("settlements/create.html", &ctx).unwrap();
            Html(html).into_response()
        }
    }
}

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
use crate::models::distribution::DistributionForm;
use crate::services::distribution_service;

pub async fn list(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
) -> impl IntoResponse {
    let user = get_current_user(&jar, &state);
    let db = state.db.lock().unwrap();
    let distributions = distribution_service::list(&db).unwrap_or_default();
    drop(db);
    let mut ctx = Context::new();
    ctx.insert("distributions", &distributions);
    ctx.insert("user", &user);
    ctx.insert("current_page", &"distributions");
    let html = state.tera.render("distributions/list.html", &ctx).unwrap();
    Html(html)
}

pub async fn detail(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let user = get_current_user(&jar, &state);
    let db = state.db.lock().unwrap();
    let dist = distribution_service::find(&db, id);
    drop(db);
    match dist {
        Ok(d) => {
            let mut ctx = Context::new();
            ctx.insert("distribution", &d);
            ctx.insert("user", &user);
            ctx.insert("current_page", &"distributions");
            let html = state.tera.render("distributions/detail.html", &ctx).unwrap();
            Html(html).into_response()
        }
        Err(_) => Redirect::to("/distributions").into_response(),
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
    let mut ctx = Context::new();
    ctx.insert("user", &user);
    ctx.insert("current_page", &"distributions");
    let html = state.tera.render("distributions/create.html", &ctx).unwrap();
    Html(html).into_response()
}

pub async fn create(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    Form(form): Form<DistributionForm>,
) -> impl IntoResponse {
    let user = get_current_user(&jar, &state);
    if user.is_none() {
        return Redirect::to("/auth/login").into_response();
    }
    let db = state.db.lock().unwrap();
    match distribution_service::create(&db, &form) {
        Ok(_) => {
            drop(db);
            Redirect::to("/distributions").into_response()
        }
        Err(msg) => {
            drop(db);
            let mut ctx = Context::new();
            ctx.insert("error", &msg);
            ctx.insert("user", &user);
            ctx.insert("current_page", &"distributions");
            let html = state.tera.render("distributions/create.html", &ctx).unwrap();
            Html(html).into_response()
        }
    }
}

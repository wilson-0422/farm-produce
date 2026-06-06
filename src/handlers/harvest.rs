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
use crate::models::harvest::HarvestForm;
use crate::services::harvest_service;

pub async fn list(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
) -> impl IntoResponse {
    let user = get_current_user(&jar, &state);
    let db = state.db.lock().unwrap();
    let harvests = harvest_service::list(&db).unwrap_or_default();
    drop(db);
    let mut ctx = Context::new();
    ctx.insert("harvests", &harvests);
    ctx.insert("user", &user);
    ctx.insert("current_page", &"harvests");
    let html = state.tera.render("harvests/list.html", &ctx).unwrap();
    Html(html)
}

pub async fn detail(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let user = get_current_user(&jar, &state);
    let db = state.db.lock().unwrap();
    let harvest = harvest_service::find(&db, id);
    drop(db);
    match harvest {
        Ok(h) => {
            let mut ctx = Context::new();
            ctx.insert("harvest", &h);
            ctx.insert("user", &user);
            ctx.insert("current_page", &"harvests");
            let html = state.tera.render("harvests/detail.html", &ctx).unwrap();
            Html(html).into_response()
        }
        Err(_) => Redirect::to("/harvests").into_response(),
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
    ctx.insert("current_page", &"harvests");
    let html = state.tera.render("harvests/create.html", &ctx).unwrap();
    Html(html).into_response()
}

pub async fn create(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    Form(form): Form<HarvestForm>,
) -> impl IntoResponse {
    let user = get_current_user(&jar, &state);
    let user = match user {
        Some(u) => u,
        None => return Redirect::to("/auth/login").into_response(),
    };
    let db = state.db.lock().unwrap();
    match harvest_service::create(&db, user.id, &form) {
        Ok(_) => {
            drop(db);
            Redirect::to("/harvests").into_response()
        }
        Err(msg) => {
            drop(db);
            let mut ctx = Context::new();
            ctx.insert("error", &msg);
            ctx.insert("user", &Some(user));
            ctx.insert("current_page", &"harvests");
            let html = state.tera.render("harvests/create.html", &ctx).unwrap();
            Html(html).into_response()
        }
    }
}

pub async fn edit_page(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let user = get_current_user(&jar, &state);
    if user.is_none() {
        return Redirect::to("/auth/login").into_response();
    }
    let db = state.db.lock().unwrap();
    let harvest = harvest_service::find(&db, id);
    drop(db);
    match harvest {
        Ok(h) => {
            let mut ctx = Context::new();
            ctx.insert("harvest", &h);
            ctx.insert("user", &user);
            ctx.insert("current_page", &"harvests");
            let html = state.tera.render("harvests/edit.html", &ctx).unwrap();
            Html(html).into_response()
        }
        Err(_) => Redirect::to("/harvests").into_response(),
    }
}

pub async fn edit(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    Path(id): Path<i64>,
    Form(form): Form<HarvestForm>,
) -> impl IntoResponse {
    let user = get_current_user(&jar, &state);
    if user.is_none() {
        return Redirect::to("/auth/login").into_response();
    }
    let db = state.db.lock().unwrap();
    match harvest_service::update(&db, id, &form) {
        Ok(_) => {
            drop(db);
            Redirect::to(&format!("/harvests/{}", id)).into_response()
        }
        Err(msg) => {
            let harvest = harvest_service::find(&db, id).ok();
            drop(db);
            let mut ctx = Context::new();
            ctx.insert("error", &msg);
            ctx.insert("harvest", &harvest);
            ctx.insert("user", &user);
            ctx.insert("current_page", &"harvests");
            let html = state.tera.render("harvests/edit.html", &ctx).unwrap();
            Html(html).into_response()
        }
    }
}

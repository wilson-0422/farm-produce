use axum::{
    extract::State,
    response::{Html, IntoResponse, Redirect},
    Form,
};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use std::sync::Arc;
use tera::Context;

use crate::config::app::AppState;
use crate::middleware::auth::get_current_user;
use crate::models::user::{LoginForm, RegisterForm};
use crate::services::user_service;

pub async fn login_page(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
) -> impl IntoResponse {
    let user = get_current_user(&jar, &state);
    if user.is_some() {
        return Redirect::to("/").into_response();
    }
    let mut ctx = Context::new();
    ctx.insert("current_page", &"auth");
    let html = state.tera.render("auth/login.html", &ctx).unwrap();
    Html(html).into_response()
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    Form(form): Form<LoginForm>,
) -> impl IntoResponse {
    let db = state.db.lock().unwrap();
    match user_service::verify_login(&db, &form) {
        Ok(user) => {
            match user_service::create_session(&db, user.id) {
                Ok(token) => {
                    drop(db);
                    let cookie = Cookie::build(("session_token", token))
                        .path("/")
                        .http_only(true)
                        .finish();
                    (jar.add(cookie), Redirect::to("/")).into_response()
                }
                Err(_) => {
                    drop(db);
                    let mut ctx = Context::new();
                    ctx.insert("error", &"创建会话失败");
                    ctx.insert("current_page", &"auth");
                    let html = state.tera.render("auth/login.html", &ctx).unwrap();
                    Html(html).into_response()
                }
            }
        }
        Err(msg) => {
            drop(db);
            let mut ctx = Context::new();
            ctx.insert("error", &msg);
            ctx.insert("current_page", &"auth");
            let html = state.tera.render("auth/login.html", &ctx).unwrap();
            Html(html).into_response()
        }
    }
}

pub async fn register_page(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
) -> impl IntoResponse {
    let user = get_current_user(&jar, &state);
    if user.is_some() {
        return Redirect::to("/").into_response();
    }
    let mut ctx = Context::new();
    ctx.insert("current_page", &"auth");
    let html = state.tera.render("auth/register.html", &ctx).unwrap();
    Html(html).into_response()
}

pub async fn register(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    Form(form): Form<RegisterForm>,
) -> impl IntoResponse {
    let db = state.db.lock().unwrap();
    match user_service::create_user(&db, &form) {
        Ok(_) => {
            drop(db);
            let mut ctx = Context::new();
            ctx.insert("success", &"注册成功，请登录");
            ctx.insert("current_page", &"auth");
            let html = state.tera.render("auth/login.html", &ctx).unwrap();
            Html(html).into_response()
        }
        Err(msg) => {
            drop(db);
            let mut ctx = Context::new();
            ctx.insert("error", &msg);
            ctx.insert("current_page", &"auth");
            let html = state.tera.render("auth/register.html", &ctx).unwrap();
            Html(html).into_response()
        }
    }
}

pub async fn logout(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
) -> impl IntoResponse {
    if let Some(cookie) = jar.get("session_token") {
        let token = cookie.value().to_string();
        let db = state.db.lock().unwrap();
        user_service::delete_session(&db, &token).ok();
        drop(db);
    }
    let cookie = Cookie::build(("session_token", ""))
        .path("/")
        .http_only(true)
        .finish();
    (jar.remove(cookie), Redirect::to("/auth/login")).into_response()
}

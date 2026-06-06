use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use axum_extra::extract::cookie::CookieJar;
use std::sync::Arc;
use tera::Context;

use crate::config::app::AppState;
use crate::middleware::auth::get_current_user;
use crate::services::{harvest_service, warehouse_service, distribution_service, logistics_service, settlement_service};

pub async fn index(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
) -> impl IntoResponse {
    let user = get_current_user(&jar, &state);
    let db = state.db.lock().unwrap();
    let harvest_count = harvest_service::count(&db).unwrap_or(0);
    let warehouse_count = warehouse_service::count(&db).unwrap_or(0);
    let dist_count = distribution_service::count(&db).unwrap_or(0);
    let logistics_count = logistics_service::count(&db).unwrap_or(0);
    let settlement_count = settlement_service::count(&db).unwrap_or(0);
    let pending_settlements = settlement_service::count_pending(&db).unwrap_or(0);
    let pending_amount = settlement_service::total_pending_amount(&db).unwrap_or(0.0);
    let total_quantity = harvest_service::total_quantity(&db).unwrap_or(0.0);
    let in_transit = logistics_service::count_in_transit(&db).unwrap_or(0);
    let total_sales = distribution_service::total_amount(&db).unwrap_or(0.0);
    drop(db);

    let mut ctx = Context::new();
    ctx.insert("user", &user);
    ctx.insert("current_page", &"home");
    ctx.insert("harvest_count", &harvest_count);
    ctx.insert("warehouse_count", &warehouse_count);
    ctx.insert("dist_count", &dist_count);
    ctx.insert("logistics_count", &logistics_count);
    ctx.insert("settlement_count", &settlement_count);
    ctx.insert("pending_settlements", &pending_settlements);
    ctx.insert("pending_amount", &pending_amount);
    ctx.insert("total_quantity", &total_quantity);
    ctx.insert("in_transit", &in_transit);
    ctx.insert("total_sales", &total_sales);
    let html = state.tera.render("index.html", &ctx).unwrap();
    Html(html)
}

pub async fn dashboard(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
) -> impl IntoResponse {
    let user = get_current_user(&jar, &state);
    let db = state.db.lock().unwrap();
    let harvest_count = harvest_service::count(&db).unwrap_or(0);
    let warehouse_count = warehouse_service::count(&db).unwrap_or(0);
    let dist_count = distribution_service::count(&db).unwrap_or(0);
    let logistics_count = logistics_service::count(&db).unwrap_or(0);
    let settlement_count = settlement_service::count(&db).unwrap_or(0);
    let pending_settlements = settlement_service::count_pending(&db).unwrap_or(0);
    let pending_amount = settlement_service::total_pending_amount(&db).unwrap_or(0.0);
    let total_quantity = harvest_service::total_quantity(&db).unwrap_or(0.0);
    let in_transit = logistics_service::count_in_transit(&db).unwrap_or(0);
    let total_sales = distribution_service::total_amount(&db).unwrap_or(0.0);
    let recent_harvests = harvest_service::list(&db).unwrap_or_default();
    drop(db);

    let mut ctx = Context::new();
    ctx.insert("user", &user);
    ctx.insert("current_page", &"dashboard");
    ctx.insert("harvest_count", &harvest_count);
    ctx.insert("warehouse_count", &warehouse_count);
    ctx.insert("dist_count", &dist_count);
    ctx.insert("logistics_count", &logistics_count);
    ctx.insert("settlement_count", &settlement_count);
    ctx.insert("pending_settlements", &pending_settlements);
    ctx.insert("pending_amount", &pending_amount);
    ctx.insert("total_quantity", &total_quantity);
    ctx.insert("in_transit", &in_transit);
    ctx.insert("total_sales", &total_sales);
    ctx.insert("recent_harvests", &recent_harvests);
    let html = state.tera.render("dashboard/overview.html", &ctx).unwrap();
    Html(html)
}

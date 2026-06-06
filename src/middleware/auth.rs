use axum_extra::extract::cookie::CookieJar;
use crate::config::app::AppState;
use crate::models::user::User;
use crate::services::user_service;

pub fn get_current_user(jar: &CookieJar, state: &AppState) -> Option<User> {
    let cookie = jar.get("session_token")?;
    let token = cookie.value();
    let db = state.db.lock().ok()?;
    user_service::find_by_session_token(&db, token).ok()
}

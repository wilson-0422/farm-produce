use rusqlite::Connection;
use std::sync::Mutex;
use tera::Tera;

pub struct AppState {
    pub db: Mutex<Connection>,
    pub tera: Tera,
}

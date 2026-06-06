use rusqlite::{params, Connection};
use crate::models::user::{User, LoginForm, RegisterForm};
use bcrypt::{verify, hash, DEFAULT_COST};
use uuid::Uuid;

pub fn find_by_username(conn: &Connection, username: &str) -> Result<User, rusqlite::Error> {
    conn.query_row(
        "SELECT id, username, password_hash, role, name, phone, created_at FROM users WHERE username = ?1",
        params![username],
        |row| Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            password_hash: row.get(2)?,
            role: row.get(3)?,
            name: row.get(4)?,
            phone: row.get(5)?,
            created_at: row.get(6)?,
        }),
    )
}

pub fn find_by_id(conn: &Connection, id: i64) -> Result<User, rusqlite::Error> {
    conn.query_row(
        "SELECT id, username, password_hash, role, name, phone, created_at FROM users WHERE id = ?1",
        params![id],
        |row| Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            password_hash: row.get(2)?,
            role: row.get(3)?,
            name: row.get(4)?,
            phone: row.get(5)?,
            created_at: row.get(6)?,
        }),
    )
}

pub fn find_by_session_token(conn: &Connection, token: &str) -> Result<User, rusqlite::Error> {
    conn.query_row(
        "SELECT u.id, u.username, u.password_hash, u.role, u.name, u.phone, u.created_at FROM users u JOIN sessions s ON u.id = s.user_id WHERE s.token = ?1",
        params![token],
        |row| Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            password_hash: row.get(2)?,
            role: row.get(3)?,
            name: row.get(4)?,
            phone: row.get(5)?,
            created_at: row.get(6)?,
        }),
    )
}

pub fn verify_login(conn: &Connection, form: &LoginForm) -> Result<User, String> {
    let user = find_by_username(conn, &form.username).map_err(|_| "用户名或密码错误".to_string())?;
    let valid = verify(&form.password, &user.password_hash).map_err(|_| "验证失败".to_string())?;
    if valid {
        Ok(user)
    } else {
        Err("用户名或密码错误".to_string())
    }
}

pub fn create_session(conn: &Connection, user_id: i64) -> Result<String, rusqlite::Error> {
    let token = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    conn.execute(
        "INSERT INTO sessions (user_id, token, created_at) VALUES (?1, ?2, ?3)",
        params![user_id, token, now],
    )?;
    Ok(token)
}

pub fn delete_session(conn: &Connection, token: &str) -> Result<(), rusqlite::Error> {
    conn.execute("DELETE FROM sessions WHERE token = ?1", params![token])?;
    Ok(())
}

pub fn create_user(conn: &Connection, form: &RegisterForm) -> Result<i64, String> {
    let existing = find_by_username(conn, &form.username);
    if existing.is_ok() {
        return Err("用户名已存在".to_string());
    }
    let ph = hash(&form.password, DEFAULT_COST).map_err(|e| e.to_string())?;
    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    conn.execute(
        "INSERT INTO users (username, password_hash, role, name, phone, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![form.username, ph, form.role, form.name, form.phone, now],
    ).map_err(|e| e.to_string())?;
    Ok(conn.last_insert_rowid())
}

pub fn list_farmers(conn: &Connection) -> Result<Vec<User>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT id, username, password_hash, role, name, phone, created_at FROM users WHERE role = 'farmer' ORDER BY name",
    )?;
    let users = stmt.query_map([], |row| Ok(User {
        id: row.get(0)?,
        username: row.get(1)?,
        password_hash: row.get(2)?,
        role: row.get(3)?,
        name: row.get(4)?,
        phone: row.get(5)?,
        created_at: row.get(6)?,
    }))?.collect::<Result<Vec<_>, _>>()?;
    Ok(users)
}

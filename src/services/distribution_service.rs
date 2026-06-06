use rusqlite::{params, Connection};
use crate::models::distribution::{Distribution, DistributionForm};

pub fn list(conn: &Connection) -> Result<Vec<Distribution>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT id, channel_name, contact_person, contact_phone, product_name, quantity, unit_price, total_amount, status, created_at FROM distributions ORDER BY created_at DESC"
    )?;
    let rows = stmt.query_map([], |row| Ok(Distribution {
        id: row.get(0)?,
        channel_name: row.get(1)?,
        contact_person: row.get(2)?,
        contact_phone: row.get(3)?,
        product_name: row.get(4)?,
        quantity: row.get(5)?,
        unit_price: row.get(6)?,
        total_amount: row.get(7)?,
        status: row.get(8)?,
        created_at: row.get(9)?,
    }))?;
    let mut result = Vec::new();
    for r in rows {
        if let Ok(d) = r {
            result.push(d);
        }
    }
    Ok(result)
}

pub fn find(conn: &Connection, id: i64) -> Result<Distribution, rusqlite::Error> {
    conn.query_row(
        "SELECT id, channel_name, contact_person, contact_phone, product_name, quantity, unit_price, total_amount, status, created_at FROM distributions WHERE id = ?1",
        params![id],
        |row| Ok(Distribution {
            id: row.get(0)?,
            channel_name: row.get(1)?,
            contact_person: row.get(2)?,
            contact_phone: row.get(3)?,
            product_name: row.get(4)?,
            quantity: row.get(5)?,
            unit_price: row.get(6)?,
            total_amount: row.get(7)?,
            status: row.get(8)?,
            created_at: row.get(9)?,
        }),
    )
}

pub fn create(conn: &Connection, form: &DistributionForm) -> Result<i64, String> {
    let quantity: f64 = form.quantity.parse().map_err(|_| "数量格式错误".to_string())?;
    let unit_price: f64 = form.unit_price.parse().map_err(|_| "单价格式错误".to_string())?;
    let total = quantity * unit_price;
    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    conn.execute(
        "INSERT INTO distributions (channel_name, contact_person, contact_phone, product_name, quantity, unit_price, total_amount, status, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 'pending', ?8)",
        params![form.channel_name, form.contact_person, form.contact_phone, form.product_name, quantity, unit_price, total, now],
    ).map_err(|e| e.to_string())?;
    Ok(conn.last_insert_rowid())
}

pub fn update_status(conn: &Connection, id: i64, status: &str) -> Result<(), rusqlite::Error> {
    conn.execute("UPDATE distributions SET status = ?1 WHERE id = ?2", params![status, id])?;
    Ok(())
}

pub fn count(conn: &Connection) -> Result<i64, rusqlite::Error> {
    conn.query_row("SELECT COUNT(*) FROM distributions", [], |r| r.get(0))
}

pub fn total_amount(conn: &Connection) -> Result<f64, rusqlite::Error> {
    conn.query_row("SELECT COALESCE(SUM(total_amount), 0) FROM distributions", [], |r| r.get(0))
}

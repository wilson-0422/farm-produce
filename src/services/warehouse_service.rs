use rusqlite::{params, Connection};
use crate::models::warehouse::{WarehouseEntry, WarehouseIntakeForm};

pub fn list(conn: &Connection) -> Result<Vec<WarehouseEntry>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT w.id, w.harvest_id, h.product_name, w.keeper_id, u.name as keeper_name, w.location, w.quantity_stored, w.intake_date, w.status, w.notes, w.created_at FROM warehouse_entries w JOIN harvests h ON w.harvest_id = h.id JOIN users u ON w.keeper_id = u.id ORDER BY w.created_at DESC"
    )?;
    let rows = stmt.query_map([], |row| Ok(WarehouseEntry {
        id: row.get(0)?,
        harvest_id: row.get(1)?,
        product_name: row.get(2)?,
        keeper_id: row.get(3)?,
        keeper_name: row.get(4)?,
        location: row.get(5)?,
        quantity_stored: row.get(6)?,
        intake_date: row.get(7)?,
        status: row.get(8)?,
        notes: row.get(9)?,
        created_at: row.get(10)?,
    }))?;
    let mut result = Vec::new();
    for r in rows {
        if let Ok(w) = r {
            result.push(w);
        }
    }
    Ok(result)
}

pub fn find(conn: &Connection, id: i64) -> Result<WarehouseEntry, rusqlite::Error> {
    conn.query_row(
        "SELECT w.id, w.harvest_id, h.product_name, w.keeper_id, u.name as keeper_name, w.location, w.quantity_stored, w.intake_date, w.status, w.notes, w.created_at FROM warehouse_entries w JOIN harvests h ON w.harvest_id = h.id JOIN users u ON w.keeper_id = u.id WHERE w.id = ?1",
        params![id],
        |row| Ok(WarehouseEntry {
            id: row.get(0)?,
            harvest_id: row.get(1)?,
            product_name: row.get(2)?,
            keeper_id: row.get(3)?,
            keeper_name: row.get(4)?,
            location: row.get(5)?,
            quantity_stored: row.get(6)?,
            intake_date: row.get(7)?,
            status: row.get(8)?,
            notes: row.get(9)?,
            created_at: row.get(10)?,
        }),
    )
}

pub fn create(conn: &Connection, keeper_id: i64, form: &WarehouseIntakeForm) -> Result<i64, String> {
    let harvest_id: i64 = form.harvest_id.parse().map_err(|_| "收成ID格式错误".to_string())?;
    let qty: f64 = form.quantity_stored.parse().map_err(|_| "入库数量格式错误".to_string())?;
    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    conn.execute(
        "INSERT INTO warehouse_entries (harvest_id, keeper_id, location, quantity_stored, intake_date, status, notes, created_at) VALUES (?1, ?2, ?3, ?4, ?5, 'stored', ?6, ?7)",
        params![harvest_id, keeper_id, form.location, qty, form.intake_date, form.notes, now],
    ).map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE harvests SET status = 'warehoused' WHERE id = ?1",
        params![harvest_id],
    ).map_err(|e| e.to_string())?;
    Ok(conn.last_insert_rowid())
}

pub fn update_status(conn: &Connection, id: i64, status: &str) -> Result<(), rusqlite::Error> {
    conn.execute("UPDATE warehouse_entries SET status = ?1 WHERE id = ?2", params![status, id])?;
    Ok(())
}

pub fn count(conn: &Connection) -> Result<i64, rusqlite::Error> {
    conn.query_row("SELECT COUNT(*) FROM warehouse_entries", [], |r| r.get(0))
}

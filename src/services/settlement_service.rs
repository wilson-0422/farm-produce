use rusqlite::{params, Connection};
use crate::models::settlement::{Settlement, SettlementForm};

pub fn list(conn: &Connection) -> Result<Vec<Settlement>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT s.id, s.distribution_id, s.harvest_id, s.farmer_id, u.name as farmer_name, s.amount, s.status, s.payment_method, COALESCE(s.paid_at, '') as paid_at, s.notes, s.created_at FROM settlements s JOIN users u ON s.farmer_id = u.id ORDER BY s.created_at DESC"
    )?;
    let rows = stmt.query_map([], |row| Ok(Settlement {
        id: row.get(0)?,
        distribution_id: row.get(1)?,
        harvest_id: row.get(2)?,
        farmer_id: row.get(3)?,
        farmer_name: row.get(4)?,
        amount: row.get(5)?,
        status: row.get(6)?,
        payment_method: row.get(7)?,
        paid_at: row.get(8)?,
        notes: row.get(9)?,
        created_at: row.get(10)?,
    }))?;
    let mut result = Vec::new();
    for r in rows {
        if let Ok(s) = r {
            result.push(s);
        }
    }
    Ok(result)
}

pub fn find(conn: &Connection, id: i64) -> Result<Settlement, rusqlite::Error> {
    conn.query_row(
        "SELECT s.id, s.distribution_id, s.harvest_id, s.farmer_id, u.name as farmer_name, s.amount, s.status, s.payment_method, COALESCE(s.paid_at, '') as paid_at, s.notes, s.created_at FROM settlements s JOIN users u ON s.farmer_id = u.id WHERE s.id = ?1",
        params![id],
        |row| Ok(Settlement {
            id: row.get(0)?,
            distribution_id: row.get(1)?,
            harvest_id: row.get(2)?,
            farmer_id: row.get(3)?,
            farmer_name: row.get(4)?,
            amount: row.get(5)?,
            status: row.get(6)?,
            payment_method: row.get(7)?,
            paid_at: row.get(8)?,
            notes: row.get(9)?,
            created_at: row.get(10)?,
        }),
    )
}

pub fn create(conn: &Connection, form: &SettlementForm) -> Result<i64, String> {
    let dist_id: i64 = form.distribution_id.parse().map_err(|_| "分销单ID格式错误".to_string())?;
    let harvest_id: i64 = form.harvest_id.parse().map_err(|_| "收成ID格式错误".to_string())?;
    let amount: f64 = form.amount.parse().map_err(|_| "金额格式错误".to_string())?;
    let farmer_id: i64 = conn.query_row(
        "SELECT farmer_id FROM harvests WHERE id = ?1",
        params![harvest_id],
        |r| r.get(0),
    ).map_err(|_| "收成记录不存在".to_string())?;
    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    conn.execute(
        "INSERT INTO settlements (distribution_id, harvest_id, farmer_id, amount, status, payment_method, notes, created_at) VALUES (?1, ?2, ?3, ?4, 'pending', ?5, ?6, ?7)",
        params![dist_id, harvest_id, farmer_id, amount, form.payment_method, form.notes, now],
    ).map_err(|e| e.to_string())?;
    Ok(conn.last_insert_rowid())
}

pub fn mark_paid(conn: &Connection, id: i64) -> Result<(), rusqlite::Error> {
    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    conn.execute(
        "UPDATE settlements SET status = 'paid', paid_at = ?1 WHERE id = ?2",
        params![now, id],
    )?;
    Ok(())
}

pub fn count(conn: &Connection) -> Result<i64, rusqlite::Error> {
    conn.query_row("SELECT COUNT(*) FROM settlements", [], |r| r.get(0))
}

pub fn count_pending(conn: &Connection) -> Result<i64, rusqlite::Error> {
    conn.query_row("SELECT COUNT(*) FROM settlements WHERE status = 'pending'", [], |r| r.get(0))
}

pub fn total_pending_amount(conn: &Connection) -> Result<f64, rusqlite::Error> {
    conn.query_row("SELECT COALESCE(SUM(amount), 0) FROM settlements WHERE status = 'pending'", [], |r| r.get(0))
}

use rusqlite::{params, Connection};
use crate::models::logistics::{Logistics, LogisticsForm};

pub fn list(conn: &Connection) -> Result<Vec<Logistics>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT l.id, l.distribution_id, d.channel_name, l.driver_name, l.driver_phone, l.vehicle_plate, l.departure_time, l.estimated_arrival, COALESCE(l.actual_arrival, '') as actual_arrival, l.status, l.notes, l.created_at FROM logistics l JOIN distributions d ON l.distribution_id = d.id ORDER BY l.created_at DESC"
    )?;
    let rows = stmt.query_map([], |row| Ok(Logistics {
        id: row.get(0)?,
        distribution_id: row.get(1)?,
        channel_name: row.get(2)?,
        driver_name: row.get(3)?,
        driver_phone: row.get(4)?,
        vehicle_plate: row.get(5)?,
        departure_time: row.get(6)?,
        estimated_arrival: row.get(7)?,
        actual_arrival: row.get(8)?,
        status: row.get(9)?,
        notes: row.get(10)?,
        created_at: row.get(11)?,
    }))?;
    let mut result = Vec::new();
    for r in rows {
        if let Ok(l) = r {
            result.push(l);
        }
    }
    Ok(result)
}

pub fn find(conn: &Connection, id: i64) -> Result<Logistics, rusqlite::Error> {
    conn.query_row(
        "SELECT l.id, l.distribution_id, d.channel_name, l.driver_name, l.driver_phone, l.vehicle_plate, l.departure_time, l.estimated_arrival, COALESCE(l.actual_arrival, '') as actual_arrival, l.status, l.notes, l.created_at FROM logistics l JOIN distributions d ON l.distribution_id = d.id WHERE l.id = ?1",
        params![id],
        |row| Ok(Logistics {
            id: row.get(0)?,
            distribution_id: row.get(1)?,
            channel_name: row.get(2)?,
            driver_name: row.get(3)?,
            driver_phone: row.get(4)?,
            vehicle_plate: row.get(5)?,
            departure_time: row.get(6)?,
            estimated_arrival: row.get(7)?,
            actual_arrival: row.get(8)?,
            status: row.get(9)?,
            notes: row.get(10)?,
            created_at: row.get(11)?,
        }),
    )
}

pub fn create(conn: &Connection, form: &LogisticsForm) -> Result<i64, String> {
    let dist_id: i64 = form.distribution_id.parse().map_err(|_| "分销单ID格式错误".to_string())?;
    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    conn.execute(
        "INSERT INTO logistics (distribution_id, driver_name, driver_phone, vehicle_plate, departure_time, estimated_arrival, actual_arrival, status, notes, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, '', 'pending', ?7, ?8)",
        params![dist_id, form.driver_name, form.driver_phone, form.vehicle_plate, form.departure_time, form.estimated_arrival, form.notes, now],
    ).map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE distributions SET status = 'shipping' WHERE id = ?1",
        params![dist_id],
    ).map_err(|e| e.to_string())?;
    Ok(conn.last_insert_rowid())
}

pub fn update_status(conn: &Connection, id: i64, status: &str) -> Result<(), rusqlite::Error> {
    conn.execute("UPDATE logistics SET status = ?1 WHERE id = ?2", params![status, id])?;
    Ok(())
}

pub fn mark_delivered(conn: &Connection, id: i64) -> Result<(), rusqlite::Error> {
    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    conn.execute(
        "UPDATE logistics SET status = 'delivered', actual_arrival = ?1 WHERE id = ?2",
        params![now, id],
    )?;
    let dist_id: i64 = conn.query_row(
        "SELECT distribution_id FROM logistics WHERE id = ?1",
        params![id],
        |r| r.get(0),
    )?;
    conn.execute(
        "UPDATE distributions SET status = 'completed' WHERE id = ?1",
        params![dist_id],
    )?;
    Ok(())
}

pub fn count(conn: &Connection) -> Result<i64, rusqlite::Error> {
    conn.query_row("SELECT COUNT(*) FROM logistics", [], |r| r.get(0))
}

pub fn count_in_transit(conn: &Connection) -> Result<i64, rusqlite::Error> {
    conn.query_row("SELECT COUNT(*) FROM logistics WHERE status = 'in_transit'", [], |r| r.get(0))
}

use rusqlite::{params, Connection};
use crate::models::harvest::{Harvest, HarvestForm};

pub fn list(conn: &Connection) -> Result<Vec<Harvest>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT h.id, h.farmer_id, u.name as farmer_name, h.product_name, h.quantity, h.unit, h.price_per_unit, h.harvest_date, h.status, h.description, h.created_at FROM harvests h JOIN users u ON h.farmer_id = u.id ORDER BY h.created_at DESC"
    )?;
    let rows = stmt.query_map([], |row| Ok(Harvest {
        id: row.get(0)?,
        farmer_id: row.get(1)?,
        farmer_name: row.get(2)?,
        product_name: row.get(3)?,
        quantity: row.get(4)?,
        unit: row.get(5)?,
        price_per_unit: row.get(6)?,
        harvest_date: row.get(7)?,
        status: row.get(8)?,
        description: row.get(9)?,
        created_at: row.get(10)?,
    }))?;
    let mut result = Vec::new();
    for r in rows {
        if let Ok(h) = r {
            result.push(h);
        }
    }
    Ok(result)
}

pub fn find(conn: &Connection, id: i64) -> Result<Harvest, rusqlite::Error> {
    conn.query_row(
        "SELECT h.id, h.farmer_id, u.name as farmer_name, h.product_name, h.quantity, h.unit, h.price_per_unit, h.harvest_date, h.status, h.description, h.created_at FROM harvests h JOIN users u ON h.farmer_id = u.id WHERE h.id = ?1",
        params![id],
        |row| Ok(Harvest {
            id: row.get(0)?,
            farmer_id: row.get(1)?,
            farmer_name: row.get(2)?,
            product_name: row.get(3)?,
            quantity: row.get(4)?,
            unit: row.get(5)?,
            price_per_unit: row.get(6)?,
            harvest_date: row.get(7)?,
            status: row.get(8)?,
            description: row.get(9)?,
            created_at: row.get(10)?,
        }),
    )
}

pub fn create(conn: &Connection, farmer_id: i64, form: &HarvestForm) -> Result<i64, String> {
    let quantity: f64 = form.quantity.parse().map_err(|_| "数量格式错误".to_string())?;
    let price: f64 = form.price_per_unit.parse().map_err(|_| "单价格式错误".to_string())?;
    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    conn.execute(
        "INSERT INTO harvests (farmer_id, product_name, quantity, unit, price_per_unit, harvest_date, status, description, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, 'pending', ?7, ?8)",
        params![farmer_id, form.product_name, quantity, form.unit, price, form.harvest_date, form.description, now],
    ).map_err(|e| e.to_string())?;
    Ok(conn.last_insert_rowid())
}

pub fn update(conn: &Connection, id: i64, form: &HarvestForm) -> Result<(), String> {
    let quantity: f64 = form.quantity.parse().map_err(|_| "数量格式错误".to_string())?;
    let price: f64 = form.price_per_unit.parse().map_err(|_| "单价格式错误".to_string())?;
    conn.execute(
        "UPDATE harvests SET product_name = ?1, quantity = ?2, unit = ?3, price_per_unit = ?4, harvest_date = ?5, description = ?6 WHERE id = ?7",
        params![form.product_name, quantity, form.unit, price, form.harvest_date, form.description, id],
    ).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn update_status(conn: &Connection, id: i64, status: &str) -> Result<(), rusqlite::Error> {
    conn.execute("UPDATE harvests SET status = ?1 WHERE id = ?2", params![status, id])?;
    Ok(())
}

pub fn list_by_farmer(conn: &Connection, farmer_id: i64) -> Result<Vec<Harvest>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT h.id, h.farmer_id, u.name as farmer_name, h.product_name, h.quantity, h.unit, h.price_per_unit, h.harvest_date, h.status, h.description, h.created_at FROM harvests h JOIN users u ON h.farmer_id = u.id WHERE h.farmer_id = ?1 ORDER BY h.created_at DESC"
    )?;
    let rows = stmt.query_map(params![farmer_id], |row| Ok(Harvest {
        id: row.get(0)?,
        farmer_id: row.get(1)?,
        farmer_name: row.get(2)?,
        product_name: row.get(3)?,
        quantity: row.get(4)?,
        unit: row.get(5)?,
        price_per_unit: row.get(6)?,
        harvest_date: row.get(7)?,
        status: row.get(8)?,
        description: row.get(9)?,
        created_at: row.get(10)?,
    }))?;
    let mut result = Vec::new();
    for r in rows {
        if let Ok(h) = r {
            result.push(h);
        }
    }
    Ok(result)
}

pub fn count(conn: &Connection) -> Result<i64, rusqlite::Error> {
    conn.query_row("SELECT COUNT(*) FROM harvests", [], |r| r.get(0))
}

pub fn total_quantity(conn: &Connection) -> Result<f64, rusqlite::Error> {
    conn.query_row("SELECT COALESCE(SUM(quantity), 0) FROM harvests", [], |r| r.get(0))
}

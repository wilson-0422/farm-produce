use rusqlite::Connection;

pub fn init_db() -> Connection {
    let conn = Connection::open("farm_produce.db").expect("failed to open database");
    conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;").ok();
    create_tables(&conn);
    conn
}

fn create_tables(conn: &Connection) {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            role TEXT NOT NULL,
            name TEXT NOT NULL,
            phone TEXT NOT NULL,
            created_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS sessions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            token TEXT NOT NULL UNIQUE,
            created_at TEXT NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users(id)
        );
        CREATE TABLE IF NOT EXISTS harvests (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            farmer_id INTEGER NOT NULL,
            product_name TEXT NOT NULL,
            quantity REAL NOT NULL,
            unit TEXT NOT NULL,
            price_per_unit REAL NOT NULL,
            harvest_date TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'pending',
            description TEXT NOT NULL DEFAULT '',
            created_at TEXT NOT NULL,
            FOREIGN KEY (farmer_id) REFERENCES users(id)
        );
        CREATE TABLE IF NOT EXISTS warehouse_entries (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            harvest_id INTEGER NOT NULL,
            keeper_id INTEGER NOT NULL,
            location TEXT NOT NULL,
            quantity_stored REAL NOT NULL,
            intake_date TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'stored',
            notes TEXT NOT NULL DEFAULT '',
            created_at TEXT NOT NULL,
            FOREIGN KEY (harvest_id) REFERENCES harvests(id),
            FOREIGN KEY (keeper_id) REFERENCES users(id)
        );
        CREATE TABLE IF NOT EXISTS distributions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            channel_name TEXT NOT NULL,
            contact_person TEXT NOT NULL,
            contact_phone TEXT NOT NULL,
            product_name TEXT NOT NULL,
            quantity REAL NOT NULL,
            unit_price REAL NOT NULL,
            total_amount REAL NOT NULL,
            status TEXT NOT NULL DEFAULT 'pending',
            created_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS logistics (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            distribution_id INTEGER NOT NULL,
            driver_name TEXT NOT NULL,
            driver_phone TEXT NOT NULL,
            vehicle_plate TEXT NOT NULL,
            departure_time TEXT NOT NULL,
            estimated_arrival TEXT NOT NULL,
            actual_arrival TEXT,
            status TEXT NOT NULL DEFAULT 'pending',
            notes TEXT NOT NULL DEFAULT '',
            created_at TEXT NOT NULL,
            FOREIGN KEY (distribution_id) REFERENCES distributions(id)
        );
        CREATE TABLE IF NOT EXISTS settlements (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            distribution_id INTEGER NOT NULL,
            harvest_id INTEGER NOT NULL,
            farmer_id INTEGER NOT NULL,
            amount REAL NOT NULL,
            status TEXT NOT NULL DEFAULT 'pending',
            payment_method TEXT NOT NULL DEFAULT '',
            paid_at TEXT,
            notes TEXT NOT NULL DEFAULT '',
            created_at TEXT NOT NULL,
            FOREIGN KEY (distribution_id) REFERENCES distributions(id),
            FOREIGN KEY (harvest_id) REFERENCES harvests(id),
            FOREIGN KEY (farmer_id) REFERENCES users(id)
        );"
    ).expect("failed to create tables");
}

use rusqlite::{params, Connection};
use bcrypt::{hash, DEFAULT_COST};

pub fn seed_data(conn: &Connection) {
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM users", [], |r| r.get(0)).unwrap_or(0);
    if count > 0 {
        return;
    }

    let users = vec![
        ("zhangdashan", "农户1", "farmer", "张大山", "13800001001"),
        ("lixiuying", "农户2", "farmer", "李秀英", "13800001002"),
        ("wangjianguo", "仓管1", "warehouse", "王建国", "13800002001"),
        ("chenminghua", "分销1", "distribution", "陈明华", "13800003001"),
        ("zhaodelong", "物流1", "logistics", "赵德龙", "13800004001"),
        ("admin", "管理员", "admin", "系统管理员", "13800000000"),
    ];

    for (username, pwd, role, name, phone) in &users {
        let ph = hash(*pwd, DEFAULT_COST).unwrap();
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        conn.execute(
            "INSERT INTO users (username, password_hash, role, name, phone, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![username, ph, role, name, phone, now],
        ).unwrap();
    }

    let harvests = vec![
        (1, "红富士苹果", 5000.0, "公斤", 6.5, "2025-09-15", "pending", "山东烟台优质红富士，果大色红，甜度高"),
        (1, "有机番茄", 3000.0, "公斤", 4.0, "2025-10-01", "pending", "有机种植，无农药残留，口感鲜美"),
        (2, "绿茶毛尖", 800.0, "公斤", 120.0, "2025-04-20", "warehoused", "明前毛尖，高山云雾茶，品质上乘"),
        (2, "高山稻米", 10000.0, "公斤", 8.5, "2025-10-10", "warehoused", "海拔800米以上梯田种植，一年一季"),
        (1, "黄心土豆", 8000.0, "公斤", 3.2, "2025-11-05", "pending", "黄心土豆，淀粉含量高，适合炖煮"),
        (2, "紫薯", 2000.0, "公斤", 5.0, "2025-11-10", "pending", "富硒紫薯，花青素含量高，营养丰富"),
    ];

    for (farmer_id, product_name, quantity, unit, price, date, status, desc) in &harvests {
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        conn.execute(
            "INSERT INTO harvests (farmer_id, product_name, quantity, unit, price_per_unit, harvest_date, status, description, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![farmer_id, product_name, quantity, unit, price, date, status, desc, now],
        ).unwrap();
    }

    let warehouse_entries = vec![
        (3, 3, "A区-01号冷库", 800.0, "2025-04-25", "stored", "恒温4度保存，湿度65%"),
        (4, 4, "B区-03号常温库", 10000.0, "2025-10-15", "stored", "干燥通风保存"),
    ];

    for (keeper_id, harvest_id, location, qty, date, status, notes) in &warehouse_entries {
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        conn.execute(
            "INSERT INTO warehouse_entries (harvest_id, keeper_id, location, quantity_stored, intake_date, status, notes, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![harvest_id, keeper_id, location, qty, date, status, notes, now],
        ).unwrap();
    }

    let distributions = vec![
        ("永辉超市采购部", "刘经理", "13900001001", "绿茶毛尖", 200.0, 150.0, "confirmed"),
        ("盒马鲜生华东区", "王采购", "13900002001", "高山稻米", 5000.0, 9.5, "shipping"),
        ("大润发物流中心", "张主管", "13900003001", "红富士苹果", 2000.0, 8.0, "pending"),
    ];

    for (channel, contact, phone, product, qty, price, status) in &distributions {
        let total = qty * price;
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        conn.execute(
            "INSERT INTO distributions (channel_name, contact_person, contact_phone, product_name, quantity, unit_price, total_amount, status, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![channel, contact, phone, product, qty, price, total, status, now],
        ).unwrap();
    }

    let logistics_entries = vec![
        (2, "赵德龙", "13800004001", "鲁A12345", "2025-11-01 08:00", "2025-11-02 18:00", "", "in_transit", "预计明日到达"),
    ];

    for (dist_id, driver, phone, plate, depart, arrive, actual, status, notes) in &logistics_entries {
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        conn.execute(
            "INSERT INTO logistics (distribution_id, driver_name, driver_phone, vehicle_plate, departure_time, estimated_arrival, actual_arrival, status, notes, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![dist_id, driver, phone, plate, depart, arrive, actual, status, notes, now],
        ).unwrap();
    }

    let settlements = vec![
        (1, 3, 2, 30000.0, "pending", "银行转账", ""),
        (2, 4, 2, 47500.0, "pending", "银行转账", ""),
    ];

    for (dist_id, harvest_id, farmer_id, amount, status, method, notes) in &settlements {
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        conn.execute(
            "INSERT INTO settlements (distribution_id, harvest_id, farmer_id, amount, status, payment_method, notes, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![dist_id, harvest_id, farmer_id, amount, status, method, notes, now],
        ).unwrap();
    }
}

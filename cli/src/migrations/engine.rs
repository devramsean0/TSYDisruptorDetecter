use log::info;

struct Migration {
    name: &'static str,
    sql: &'static str,
}
fn migrations() -> Vec<Migration> {
    vec![
        Migration {
            name: "create_disruptions_table",
            sql: "CREATE TABLE IF NOT EXISTS disruptions (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                dates TEXT NOT NULL,
                posted_on TEXT NOT NULL,
                reason TEXT NOT NULL,
                description TEXT NOT NULL,
                impact TEXT NOT NULL,
                details TEXT NOT NULL
            )"
        },
        Migration {
            name: "create_complete_hash_on_disruptions",
            sql: "ALTER TABLE disruptions ADD COLUMN hash TEXT NOT NULL DEFAULT ''"
        },
        Migration {
            name: "create_migration_diff_state_table",
            sql: "CREATE TABLE IF NOT EXISTS disruptions_diffs (
                id INTEGER PRIMARY KEY,
                disruption_id TEXT,
                disruption_name TEXT,
                state INTEGER NOT NULL
            )"
        }
    ]
}
pub fn run_migrations(con: &rusqlite::Connection) {
    let _ = con.execute(
        "CREATE TABLE IF NOT EXISTS migrations (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            hash TEXT NOT NULL,
            UNIQUE(id, hash)
        )",
        [],
    );
    for migration in migrations() {
        let hash = format!("{:x}", md5::compute(migration.sql));
        let str_hash = hash.as_str();
        let mut stmt = con.prepare("SELECT COUNT(*) FROM migrations WHERE name = ? OR hash = ?").unwrap();
        let count: i64 = stmt.query_row(&[&migration.name, str_hash], |row| row.get(0)).unwrap();
        if count == 0 {
            con.execute(migration.sql, []).unwrap();
            con.execute("INSERT INTO migrations (name, hash) VALUES (?, ?)", &[&migration.name, str_hash]).unwrap();
            info!("Applied migration: {}", migration.name);
        }
    }
}
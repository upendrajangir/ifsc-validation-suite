use crate::models::BankData;
use sqlx::{Pool, Sqlite, Row};
use std::env;

pub type DatabasePool = Pool<Sqlite>;

pub async fn init_pool() -> anyhow::Result<DatabasePool> {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:ifsc_database.db".to_string());

    let pool = sqlx::SqlitePool::connect(&database_url).await?;
    Ok(pool)
}

pub async fn run_migrations(pool: &DatabasePool) -> anyhow::Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS bank_data (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            ifsc TEXT UNIQUE NOT NULL,
            bank TEXT NOT NULL,
            branch TEXT NOT NULL,
            address TEXT NOT NULL,
            contact TEXT,
            city TEXT NOT NULL,
            rtgs INTEGER NOT NULL DEFAULT 0,
            neft INTEGER NOT NULL DEFAULT 0,
            imps INTEGER NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_bank_data_ifsc ON bank_data(ifsc)
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn store_bank_data(pool: &DatabasePool, bank_data: &BankData) -> anyhow::Result<()> {
    sqlx::query(
        r#"
        INSERT INTO bank_data (ifsc, bank, branch, address, contact, city, rtgs, neft, imps, created_at, updated_at)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
        ON CONFLICT(ifsc) DO UPDATE SET
            bank = excluded.bank,
            branch = excluded.branch,
            address = excluded.address,
            contact = excluded.contact,
            city = excluded.city,
            rtgs = excluded.rtgs,
            neft = excluded.neft,
            imps = excluded.imps,
            updated_at = excluded.updated_at
        "#,
    )
    .bind(&bank_data.ifsc)
    .bind(&bank_data.bank)
    .bind(&bank_data.branch)
    .bind(&bank_data.address)
    .bind(&bank_data.contact)
    .bind(&bank_data.city)
    .bind(if bank_data.rtgs { 1 } else { 0 })
    .bind(if bank_data.neft { 1 } else { 0 })
    .bind(if bank_data.imps { 1 } else { 0 })
    .bind(bank_data.created_at.to_rfc3339())
    .bind(bank_data.updated_at.to_rfc3339())
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn fetch_all_bank_data(pool: &DatabasePool) -> anyhow::Result<Vec<BankData>> {
    let rows = sqlx::query(
        r#"
        SELECT ifsc, bank, branch, address, contact, city, rtgs, neft, imps, created_at, updated_at
        FROM bank_data
        ORDER BY ifsc
        "#,
    )
    .fetch_all(pool)
    .await?;

    let mut bank_data_list = Vec::new();
    for row in rows {
        let created_at_str: String = row.get("created_at");
        let updated_at_str: String = row.get("updated_at");
        
        let bank_data = BankData {
            ifsc: row.get("ifsc"),
            bank: row.get("bank"),
            branch: row.get("branch"),
            address: row.get("address"),
            contact: row.get("contact"),
            city: row.get("city"),
            rtgs: row.get::<i32, _>("rtgs") != 0,
            neft: row.get::<i32, _>("neft") != 0,
            imps: row.get::<i32, _>("imps") != 0,
            created_at: chrono::DateTime::parse_from_rfc3339(&created_at_str)?
                .with_timezone(&chrono::Utc),
            updated_at: chrono::DateTime::parse_from_rfc3339(&updated_at_str)?
                .with_timezone(&chrono::Utc),
        };
        bank_data_list.push(bank_data);
    }

    Ok(bank_data_list)
}
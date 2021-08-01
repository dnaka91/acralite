#![allow(clippy::inline_always)]

use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

use anyhow::{Context, Result};
use r2d2::{ManageConnection, Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Connection;

pub struct DbConnPool(Arc<Pool<SqliteConnectionManager>>);

impl DbConnPool {
    pub fn get(&self) -> Result<DbConn> {
        self.0.get().map(DbConn).map_err(Into::into)
    }

    pub async fn run<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(&mut Connection) -> Result<T> + Send + Sync + 'static,
        T: Send + Sync + 'static,
    {
        let pool = self.clone();

        tokio::task::spawn_blocking(move || {
            let mut conn = pool.get()?;
            f(&mut *conn)
        })
        .await?
    }
}

impl Clone for DbConnPool {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

pub struct DbConn(PooledConnection<SqliteConnectionManager>);

impl Deref for DbConn {
    type Target = Connection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DbConn {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(debug_assertions)]
const DB_PATH: &str = "data.db";
#[cfg(not(debug_assertions))]
const DB_PATH: &str = concat!("/var/lib/", env!("CARGO_CRATE_NAME"), "/data.db");

pub fn create_pool() -> Result<DbConnPool> {
    let manager = if cfg!(test) {
        SqliteConnectionManager::memory()
    } else {
        #[cfg(not(debug_assertions))]
        std::fs::create_dir_all(concat!("/var/lib/", env!("CARGO_CRATE_NAME")))?;
        SqliteConnectionManager::file(DB_PATH)
    }
    .with_init(init_connection);

    // First create a single connection to make sure all eventually locking PRAGMAs are run,
    // so we don't get any errors when spinning up the pool.
    manager.connect().context("failed to initialize database")?;

    let pool = Pool::builder().build(manager)?;

    Ok(DbConnPool(Arc::new(pool)))
}

fn init_connection(conn: &mut Connection) -> Result<(), rusqlite::Error> {
    conn.pragma_update(None, "foreign_keys", &"ON")?;
    Ok(())
}

#![allow(clippy::inline_always)]

use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

use anyhow::{Context, Result};
use r2d2::{ManageConnection, Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Connection;

use crate::dirs::DIRS;

pub struct DbConnPool(Arc<Pool<SqliteConnectionManager>>);

impl DbConnPool {
    pub fn get(&self) -> Result<DbConn, r2d2::Error> {
        self.0.get().map(DbConn).map_err(Into::into)
    }

    #[allow(clippy::trait_duplication_in_bounds)]
    pub async fn run<F, T, E>(&self, f: F) -> Result<T, E>
    where
        F: FnOnce(&mut Connection) -> Result<T, E> + Send + Sync + 'static,
        T: Send + Sync + 'static,
        E: Send + Sync + 'static + From<r2d2::Error> + From<tokio::task::JoinError>,
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

pub fn create_pool() -> Result<DbConnPool> {
    let manager = if cfg!(test) {
        SqliteConnectionManager::memory()
    } else {
        std::fs::create_dir_all(DIRS.data_dir())?;
        SqliteConnectionManager::file(DIRS.db_file())
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

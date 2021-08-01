use anyhow::Result;
use async_trait::async_trait;
use rusqlite::params;

use super::{
    models::{App, NewApp, NewReport, NewVersion, Version},
    DbConnPool,
};

#[async_trait]
pub trait AppRepository {
    async fn save(&self, app: NewApp) -> Result<i64>;
    async fn list(&self) -> Result<Vec<App>>;
    async fn get(&self, id: i64) -> Result<App>;
    async fn get_by_username(&self, username: String) -> Result<App>;
}

struct AppRepositoryImpl {
    pool: DbConnPool,
}

#[async_trait]
impl AppRepository for AppRepositoryImpl {
    async fn save(&self, app: NewApp) -> Result<i64> {
        self.pool
            .run(move |conn| {
                conn.prepare(
                    "INSERT INTO apps(user_id, name, username, password) VALUES (?,?,?,?)",
                )?
                .insert(params![app.user_id, app.name, app.username, app.password])
                .map_err(Into::into)
            })
            .await
    }

    async fn list(&self) -> Result<Vec<App>> {
        self.pool
            .run(|conn| {
                conn.prepare("SELECT * FROM apps")?
                    .query_map([], |row| {
                        Ok(App {
                            id: row.get(0)?,
                            user_id: row.get(1)?,
                            name: row.get(2)?,
                            username: row.get(3)?,
                            password: row.get(4)?,
                        })
                    })?
                    .map(|row| row.map_err(Into::into))
                    .collect()
            })
            .await
    }

    async fn get(&self, id: i64) -> Result<App> {
        self.pool
            .run(move |conn| {
                conn.prepare("SELECT * FROM apps WHERE id = ?")?
                    .query_row([id], |row| {
                        Ok(App {
                            id: row.get(0)?,
                            user_id: row.get(1)?,
                            name: row.get(2)?,
                            username: row.get(3)?,
                            password: row.get(4)?,
                        })
                    })
                    .map_err(Into::into)
            })
            .await
    }

    async fn get_by_username(&self, username: String) -> Result<App> {
        self.pool
            .run(move |conn| {
                conn.prepare("SELECT * FROM apps WHERE username = ?")?
                    .query_row([username], |row| {
                        Ok(App {
                            id: row.get(0)?,
                            user_id: row.get(1)?,
                            name: row.get(2)?,
                            username: row.get(3)?,
                            password: row.get(4)?,
                        })
                    })
                    .map_err(Into::into)
            })
            .await
    }
}

pub fn app_repo(pool: DbConnPool) -> impl AppRepository {
    AppRepositoryImpl { pool }
}

#[async_trait]
pub trait VersionRepository {
    async fn save(&self, version: NewVersion) -> Result<i64>;
    async fn get_or_create(&self, version: NewVersion) -> Result<i64>;
    async fn list(&self) -> Result<Vec<Version>>;
    async fn list_by_app(&self, id: i64) -> Result<Vec<Version>>;
}

struct VersionRepositoryImpl {
    pool: DbConnPool,
}

#[async_trait]
impl VersionRepository for VersionRepositoryImpl {
    async fn save(&self, version: NewVersion) -> Result<i64> {
        self.pool
            .run(move |conn| {
                conn.prepare("INSERT INTO versions(app_id, name, code) VALUES (?,?,?)")?
                    .insert(params![version.app_id, version.name, version.code])
                    .map_err(Into::into)
            })
            .await
    }

    async fn get_or_create(&self, version: NewVersion) -> Result<i64> {
        self.pool
            .run(move |conn| {
                match conn.query_row(
                    "SELECT id FROM versions WHERE app_id = ? AND name = ? AND code = ?",
                    params![version.app_id, version.name, version.code],
                    |row| row.get(0),
                ) {
                    Ok(id) => Ok(id),
                    Err(rusqlite::Error::QueryReturnedNoRows) => conn
                        .prepare("INSERT INTO versions(app_id, name, code) VALUES (?,?,?)")?
                        .insert(params![version.app_id, version.name, version.code])
                        .map_err(Into::into),
                    Err(e) => Err(e.into()),
                }
            })
            .await
    }

    async fn list(&self) -> Result<Vec<Version>> {
        self.pool
            .run(|conn| {
                conn.prepare("SELECT * FROM versions")?
                    .query_map([], |row| {
                        Ok(Version {
                            id: row.get(0)?,
                            app_id: row.get(1)?,
                            name: row.get(2)?,
                            code: row.get(3)?,
                        })
                    })?
                    .map(|row| row.map_err(Into::into))
                    .collect()
            })
            .await
    }

    async fn list_by_app(&self, id: i64) -> Result<Vec<Version>> {
        self.pool
            .run(move |conn| {
                conn.prepare("SELECT * FROM versions WHERE app_id = ?")?
                    .query_map([id], |row| {
                        Ok(Version {
                            id: row.get(0)?,
                            app_id: row.get(1)?,
                            name: row.get(2)?,
                            code: row.get(3)?,
                        })
                    })?
                    .map(|row| row.map_err(Into::into))
                    .collect()
            })
            .await
    }
}

pub fn version_repo(pool: DbConnPool) -> impl VersionRepository {
    VersionRepositoryImpl { pool }
}

#[async_trait]
pub trait ReportRepository {
    async fn save(&self, app: NewReport) -> Result<i64>;
}

struct ReportRepositoryImpl {
    pool: DbConnPool,
}

#[async_trait]
impl ReportRepository for ReportRepositoryImpl {
    async fn save(&self, report: NewReport) -> Result<i64> {
        self.pool
            .run(move |conn| {
                conn.prepare(
                    "INSERT INTO reports(version_id, report_id, crash_date) VALUES (?,?,?)",
                )?
                .insert(params![
                    report.version_id,
                    report.report_id,
                    report.crash_date,
                ])
                .map_err(Into::into)
            })
            .await
    }
}

pub fn report_repo(pool: DbConnPool) -> impl ReportRepository {
    ReportRepositoryImpl { pool }
}

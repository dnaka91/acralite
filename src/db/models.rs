#[derive(Debug)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
}

pub struct NewUser {
    pub username: String,
    pub password: String,
}

#[derive(Debug)]
pub struct App {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub username: String,
    pub password: String,
}

pub struct NewApp {
    pub user_id: i64,
    pub name: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug)]
pub struct Version {
    pub id: i64,
    pub app_id: i64,
    pub name: String,
    pub code: i64,
}

pub struct NewVersion {
    pub app_id: i64,
    pub name: String,
    pub code: i64,
}

pub struct NewReport {
    pub version_id: i64,
    pub report_id: String,
    pub crash_date: String,
}

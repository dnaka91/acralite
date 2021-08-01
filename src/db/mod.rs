#![allow(clippy::module_name_repetitions)]

pub use self::{
    connection::{create_pool, DbConn, DbConnPool},
    migrations::run as run_migrations,
};

pub mod models;
pub mod repositories;

mod connection;
mod migrations;

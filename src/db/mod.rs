#![allow(clippy::module_name_repetitions)]

pub use self::{
    connection::{DbConnPool, create_pool},
    migrations::run as run_migrations,
};

pub mod models;
pub mod repositories;

mod connection;
mod migrations;

use anyhow::Result;

use super::DbConnPool;

mod embedded {
    refinery::embed_migrations!("migrations");
}

pub fn run(pool: &DbConnPool) -> Result<()> {
    embedded::migrations::runner().run(&mut *pool.get()?)?;
    Ok(())
}

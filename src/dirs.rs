use anyhow::{Context, Result};
use once_cell::sync::Lazy;
use unidirs::{Directories, UnifiedDirs, Utf8Path, Utf8PathBuf};

// Unwrap: We can't run the server without knowning where to place files, so panic here as there is
// no good recovery case other than throwing an error and shutting down.
pub static DIRS: Lazy<Dirs> = Lazy::new(|| Dirs::new().unwrap());

pub struct Dirs {
    settings_file: Utf8PathBuf,
    mapping_file: Utf8PathBuf,
    db_file: Utf8PathBuf,
    reports_dir: Utf8PathBuf,
    dirs: UnifiedDirs,
}

impl Dirs {
    fn new() -> Result<Self> {
        let dirs = UnifiedDirs::simple("rocks", "dnaka91", env!("CARGO_PKG_NAME"))
            .default()
            .context("failed finding project directories")?;

        Ok(Self {
            settings_file: dirs.config_dir().join("config.toml"),
            mapping_file: dirs.data_dir().join("mapping.txt"),
            db_file: dirs.data_dir().join("data.db"),
            reports_dir: dirs.data_dir().join("reports"),
            dirs,
        })
    }

    pub fn settings_file(&self) -> &Utf8Path {
        &self.settings_file
    }

    pub fn mapping_file(&self) -> &Utf8Path {
        &self.mapping_file
    }

    pub fn db_file(&self) -> &Utf8Path {
        &self.db_file
    }

    pub fn reports_dir(&self) -> &Utf8Path {
        &self.reports_dir
    }

    pub fn data_dir(&self) -> &Utf8Path {
        self.dirs.data_dir()
    }
}

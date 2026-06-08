use std::path::PathBuf;
use std::sync::Mutex;

use anode_core::{AppConfig, Library, Result};

pub struct AppState {
    pub config: AppConfig,
    pub library: Option<Library>,
}

impl AppState {
    pub fn new() -> Result<Self> {
        let config = anode_core::get_config()?;
        let library = if anode_core::is_first_run()? {
            None
        } else {
            Some(Library::open(&config)?)
        };
        Ok(Self { config, library })
    }

    pub fn with_library<F, T>(&mut self, f: F) -> Result<T>
    where
        F: FnOnce(&Library) -> Result<T>,
    {
        let lib = self.library.as_ref().ok_or_else(|| {
            anode_core::AnodeError::msg("Library not initialized — complete first-run setup")
        })?;
        f(lib)
    }

    pub fn init_library(&mut self, path: PathBuf) -> Result<()> {
        self.config = anode_core::set_library_path(path)?;
        let library = Library::open(&self.config)?;
        let _ = anode_core::write_lock_marker(library.path());
        self.library = Some(library);
        Ok(())
    }

    pub fn ensure_library(&mut self) -> Result<()> {
        if self.library.is_none() {
            let library = Library::open(&self.config)?;
            self.library = Some(library);
        }
        Ok(())
    }
}

pub type SharedState = Mutex<AppState>;

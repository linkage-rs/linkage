use std::collections::HashSet;
use std::fmt;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

pub mod dictionary;
pub mod keyboard;
pub mod profile;
pub mod random;
pub mod theme;
pub mod training;
pub mod words;
pub mod zipper_list;

pub use theme::Theme;
pub use words::Words;

pub type CharSet = HashSet<char>;

const VERSION: u16 = 1;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Saved {
    version: u16,
    pub profiles: profile::Saved,
    pub theme_name: String,
}

impl Saved {
    pub fn new(profiles: profile::List, theme_name: &str) -> Self {
        Self {
            version: VERSION,
            profiles: profiles.into(),
            theme_name: theme_name.to_string(),
        }
    }

    pub async fn load() -> Result<Self, Error> {
        let path = Self::path().await.ok_or(Error::Corrupted)?;

        // On first launch the file won't exist — return default without error.
        if !path.exists() {
            return Ok(Self::default());
        }

        let data = tokio::fs::read(&path)
            .await
            .map_err(|e| Error::FileSystem(format!("Failed to read {}: {}", path.display(), e)))?;
        let data = String::from_utf8_lossy(&data);
        serde_json::from_str(&data)
            .map_err(|e| Error::Serde(format!("Failed to parse save data: {}", e)))
    }

    pub async fn save(&self) -> Result<(), Error> {
        let path = Self::path().await.ok_or(Error::Corrupted)?;

        // Ensure the parent directory exists
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await.map_err(|e| {
                Error::FileSystem(format!(
                    "Failed to create data directory {}: {}",
                    parent.display(),
                    e
                ))
            })?;
        }

        let data = serde_json::to_string(&self)
            .map_err(|e| Error::Serde(format!("Failed to serialize save data: {}", e)))?;

        tokio::fs::write(&path, data.into_bytes())
            .await
            .map_err(|e| Error::FileSystem(format!("Failed to write {}: {}", path.display(), e)))
    }

    async fn path() -> Option<PathBuf> {
        let mut path = dirs_next::data_dir()?;
        path.push("Linkage");
        // We don't create the directory here anymore; save() handles it.
        // This avoids silently swallowing mkdir errors with .ok()?.
        Some({
            path.push("save.dat");
            path
        })
    }
}

#[derive(Debug, Clone)]
pub enum Error {
    /// Could not determine the data directory path
    Corrupted,
    /// File system I/O error
    FileSystem(String),
    /// Serialization / deserialization error
    Serde(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Corrupted => write!(f, "Could not determine data directory path"),
            Error::FileSystem(msg) => write!(f, "{}", msg),
            Error::Serde(msg) => write!(f, "{}", msg),
        }
    }
}

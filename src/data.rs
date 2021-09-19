use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::PathBuf;

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
    pub fn new(profiles: profile::List, theme: &Theme) -> Self {
        Self {
            version: VERSION,
            profiles: profiles.into(),
            theme_name: theme.name.clone(),
        }
    }

    pub async fn load() -> Result<Self, Error> {
        let path = Self::path().await.ok_or(Error::Corrupted)?;
        let data = tokio::fs::read(path).await.map_err(Error::FileSystem)?;
        let data = String::from_utf8_lossy(&data);
        serde_json::from_str(&data).map_err(Error::Serde)
    }

    pub async fn save(&self) -> Result<(), Error> {
        let path = Self::path().await.ok_or(Error::Corrupted)?;
        let data = serde_json::to_string(&self).map_err(Error::Serde)?;

        tokio::fs::write(path, data.into_bytes())
            .await
            .map_err(Error::FileSystem)
    }

    async fn path() -> Option<PathBuf> {
        let mut path = dirs_next::data_dir()?;
        path.push("Linkage");
        tokio::fs::create_dir_all(&path).await.ok()?;
        path.push("save.dat");
        Some(path)
    }
}

#[derive(Debug)]
pub enum Error {
    Corrupted,
    FileSystem(std::io::Error),
    Serde(serde_json::Error),
}

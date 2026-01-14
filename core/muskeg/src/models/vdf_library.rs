use std::collections::HashMap;
use std::path::PathBuf;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct LibraryFolders {
    pub libraries: Vec<Library>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Library {
    pub path: PathBuf,
    pub apps: HashMap<u64, u64>,
}

#[derive(Deserialize, Debug, Hash, Eq)]
#[allow(dead_code)]
pub struct AppState {
    pub appid: u64,
    name: String,
    pub installdir: String,
}

impl PartialEq for AppState {
    fn eq(&self, other: &Self) -> bool {
        self.appid == other.appid
    }
}
use std::collections::HashMap;
use std::path::PathBuf;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct LibraryFolders {
    libraries: Vec<Library>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Library {
    path: PathBuf,
    apps: HashMap<u64, u64>,
}

#[derive(Deserialize, Debug, Hash, Eq)]
#[allow(dead_code)]
struct AppState {
    appid: u64,
    name: String,
    installdir: String,
}

impl PartialEq for AppState {
    fn eq(&self, other: &Self) -> bool {
        self.appid == other.appid
    }
}
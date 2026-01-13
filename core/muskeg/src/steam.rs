use serde::Deserialize;

#[cfg(target_os = "windows")]
mod windows {
    use std::io;
    use std::path::PathBuf;
    use winreg::RegKey;
    use winreg::enums::*;

    fn is_installed() -> io::Result<PathBuf> {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

        #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
        let key = hklm.open_subkey("SOFTWARE\\WOW6432NODE\\Valve\\Steam")?;
        #[cfg(any(target_arch = "x86", target_arch = "arm"))]
        let key = hklm.open_subkey("SOFTWARE\\Valve\\Steam")?;

        let steam_path: PathBuf = key.get_value("InstallPath")?;
        Ok(steam_path)
    }
}

#[cfg(target_os = "macos")]
mod macos {
    pub fn is_installed() {
        println!("Hello From macOS!");
    }
}

#[cfg(target_os = "linux")]
mod linux {
    use std::env;
    use std::path::Path;
    use std::path::PathBuf;

    pub fn is_installed() -> PathBuf {
        let home_dir = env::home_dir();
        let steam_dir = Path::new(".steam/steam");
        let full_dir = home_dir.unwrap().join(steam_dir);

        return full_dir;
    }
}

mod library {
    use std::borrow::Cow;
    use std::collections::HashMap;
    use std::fs;
    use std::path::{Path, PathBuf};
    use keyvalues_parser::Vdf;
    use keyvalues_serde::from_vdf;
    use crate::models::vdf_library;

    fn read_libraries(steam_dir: PathBuf) -> keyvalues_serde::Result<Vec<Library>> {
        let library_folders_dir = Path::new("config/libraryfolders.vdf");
        let full_dir = steam_dir.join(library_folders_dir);
        let vdf_text = fs::read_to_string(full_dir)?;

        let mut vdf = Vdf::parse(&vdf_text)?;
        let obj = vdf.value.get_mut_obj().unwrap();

        // Switch all the entries with keys that are an index (0, 1, ...) to `"libraries"`
        let mut index = 0;
        while let Some(mut library) = obj.remove(index.to_string().as_str()) {
            obj.entry(Cow::from("libraries"))
                .or_insert(Vec::new())
                .push(library.pop().unwrap());

            index += 1;
        }

        let deserialized: LibraryFolders = from_vdf(vdf)?;
        Ok(deserialized.libraries)
    }

    fn read_games(libraries: Vec<Library>,) -> keyvalues_serde::Result<HashMap<AppState, PathBuf>> {
        let mut apps = HashMap::new();

        for i in libraries {
            let steamapps_dir = Path::new("steamapps");
            let full_dir = i.path.join(steamapps_dir);

            if !full_dir.exists() {
                continue;
            }

            for j in i.apps {
                let app_id = j.0;
                let acf_file = format!("appmanifest_{app_id}.acf");
                let acf_dir = full_dir.join(&acf_file);
                let acf_text = fs::read_to_string(acf_dir)?;
                let acf = Vdf::parse(&acf_text)?;
                let deserialized: AppState = from_vdf(acf)?;

                let dir = full_dir.clone();
                apps.insert(deserialized, dir);
            }
        }

        Ok(apps)
    }

    fn game_installed(app_id: u64, games: HashMap<AppState, PathBuf>) -> PathBuf {
        let mut app_dir: PathBuf = PathBuf::new();

        for i in games {
            let app_state = i.0;
            let appid = app_state.appid;

            if appid != app_id {
                continue;
            }

            let install_dir = app_state.installdir;
            let steam_apps_dir = i.1.join("common");
            app_dir = steam_apps_dir.join(install_dir);
        }

        app_dir
    }
}
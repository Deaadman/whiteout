use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Mod {
    #[serde(rename = "DisplayName")]
    pub display_name: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Type")]
    pub r#type: String,
    #[serde(rename = "Author")]
    pub author: String,
    #[serde(rename = "DisplayAuthor")]
    pub display_author: Vec<String>,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Aliases")]
    pub aliases: Vec<String>,
    #[serde(rename = "Replaces")]
    pub replaces: Vec<String>,
    #[serde(rename = "ModUrl")]
    pub mod_url: String,
    #[serde(rename = "RepoName")]
    pub repo_name: String,
    #[serde(rename = "Dependencies")]
    pub dependencies: Vec<String>,
    #[serde(rename = "Version")]
    pub version: String,
    #[serde(rename = "Error")]
    pub error: bool,
    #[serde(rename = "AutoUpdate")]
    pub auto_update: bool,
    #[serde(rename = "Download")]
    pub download: String,
    #[serde(rename = "Downloads")]
    pub downloads: Vec<String>,
    #[serde(rename = "AuthorUrl")]
    pub author_url: Option<String>,
    #[serde(rename = "SupportUrl")]
    pub support_url: Option<String>,
    #[serde(rename = "Categories")]
    pub categories: Vec<String>,
    #[serde(rename = "TestedOn")]
    pub tested_on: TestedOn,
    #[serde(rename = "Released")]
    pub released: String,
    #[serde(rename = "Updated")]
    pub updated: String,
    #[serde(rename = "Images")]
    pub images: Vec<String>,
    #[serde(rename = "Status")]
    pub status: Status,
    #[serde(rename = "PreviousAuthors")]
    pub previous_authors: Option<String>,
    #[serde(rename = "Source")]
    pub source: String,
    #[serde(rename = "GameVersion")]
    pub game_version: Vec<String>
}

#[derive(Serialize, Deserialize)]
pub struct TestedOn {
    pub tld: String,
    pub ml: String,
}

#[derive(Serialize, Deserialize)]
pub struct Status {
    pub working: bool,
    pub beta: Option<bool>,
    pub patchnotes: String,
    pub notes: String,
    pub issues: Option<String>
}
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Mod {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Type")]
    r#type: String,
    #[serde(rename = "Author")]
    author: String,
    #[serde(rename = "DisplayAuthor")]
    display_author: Vec<String>,
    #[serde(rename = "Description")]
    description: String,
    #[serde(rename = "Aliases")]
    aliases: Vec<String>,
    #[serde(rename = "Replaces")]
    replaces: Vec<String>,
    #[serde(rename = "ModUrl")]
    mod_url: String,
    #[serde(rename = "RepoName")]
    repo_name: String,
    #[serde(rename = "Dependencies")]
    dependencies: Vec<String>,
    #[serde(rename = "Version")]
    version: String,
    #[serde(rename = "Error")]
    error: bool,
    #[serde(rename = "AutoUpdate")]
    auto_update: bool,
    #[serde(rename = "Download")]
    download: String,
    #[serde(rename = "Downloads")]
    downloads: Vec<String>,
    #[serde(rename = "AuthorUrl")]
    author_url: Option<String>,
    #[serde(rename = "SupportUrl")]
    support_url: Option<String>,
    #[serde(rename = "Categories")]
    categories: Vec<String>,
    #[serde(rename = "TestedOn")]
    tested_on: TestedOn,
    #[serde(rename = "Released")]
    released: String,
    #[serde(rename = "Updated")]
    updated: String,
    #[serde(rename = "Images")]
    images: Vec<String>,
    #[serde(rename = "Status")]
    status: Status,
    #[serde(rename = "PreviousAuthors")]
    previous_authors: Option<String>,
    #[serde(rename = "Source")]
    source: String,
    #[serde(rename = "GameVersion")]
    game_version: Vec<String>
}

#[derive(Serialize, Deserialize)]
struct TestedOn {
    tld: String,
    ml: String,
}

#[derive(Serialize, Deserialize)]
struct Status {
    working: bool,
    beta: Option<bool>,
    patchnotes: String,
    notes: String,
    issues: Option<String>
}
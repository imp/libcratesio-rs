use std::io::Read;
use std::collections::HashMap;

use chrono::{DateTime, UTC};
use reqwest;
use serde_json;

use errors::*;

#[derive(Debug, Deserialize)]
pub struct BadgeData {
    pub badge_type: String,
    pub attributes: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct CategoryData {
    pub category: String,
    pub crates_cnt: i32,
    pub created_at: DateTime<UTC>,
    pub description: String,
    pub id: String,
    pub slug: String,
}

#[derive(Debug, Deserialize)]
pub struct CrateLinks {
    pub version_downloads: String,
    pub versions: Option<String>,
    pub owners: Option<String>,
    pub reverse_dependencies: String,
}

#[derive(Debug, Deserialize)]
pub struct KeywordData {
    pub crates_cnt: i32,
    pub created_at: DateTime<UTC>,
    pub id: String,
    pub keyword: String,
}

#[derive(Debug, Deserialize)]
pub struct VersionLinks {
    pub authors: String,
    pub dependencies: String,
    pub version_downloads: String,
}

#[derive(Debug, Deserialize)]
pub struct VersionData {
    #[serde(rename(deserialize = "crate"))]
    pub krate: String,
    pub created_at: DateTime<UTC>,
    pub dl_path: String,
    pub id: i32,
    pub links: VersionLinks,
    pub num: String, // XXX should be semver::Version
    pub updated_at: DateTime<UTC>,
    pub downloads: i32,
    pub features: HashMap<String, Vec<String>>,
    pub yanked: bool,
}

#[derive(Debug, Deserialize)]
pub struct CrateData {
    pub badges: Option<Vec<BadgeData>>,
    pub categories: Option<Vec<String>>,
    pub created_at: DateTime<UTC>,
    pub description: Option<String>,
    pub documentation: Option<String>,
    pub downloads: i32,
    pub homepage: Option<String>,
    pub id: String,
    pub keywords: Option<Vec<String>>,
    pub name: String,
    pub license: Option<String>,
    pub links: CrateLinks,
    pub max_version: String,
    pub repository: Option<String>,
    pub updated_at: DateTime<UTC>,
    pub versions: Option<Vec<i32>>,
}

#[derive(Debug, Deserialize)]
pub struct CratesIO {
    pub categories: Vec<CategoryData>,
    #[serde(rename(deserialize = "crate"))]
    pub krate: CrateData,
    pub keywords: Vec<KeywordData>,
    pub versions: Vec<VersionData>,
}

impl CratesIO {
    fn query(krate: &str) -> reqwest::Result<reqwest::Response> {
        let url = format!("https://crates.io/api/v1/crates/{}", krate);
        reqwest::get(&url)
    }

    pub fn raw_data(krate: &str) -> Result<String> {
        let mut body = String::with_capacity(20480);
        let mut response = CratesIO::query(krate)?;
        response.read_to_string(&mut body)?;
        Ok(body)
    }

    pub fn by_name(krate: &str) -> Result<CratesIO> {
        let response = CratesIO::query(krate)?;
        let krate = serde_json::from_reader(response)?;
        Ok(krate)
    }
}

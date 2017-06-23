use std::io::Read;
use std::collections::HashMap;
use std::str::FromStr;

use chrono::{DateTime, Utc};
use reqwest;
use serde_json::{self, Value};

use errors::*;

#[derive(Debug, Deserialize)]
pub struct Errors {
    pub detail: String,
}

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    pub errors: Vec<Errors>,
}

impl ErrorResponse {
    pub fn detail(&self) -> &str {
        self.errors.get(0).map(|x| x.detail.as_str()).unwrap_or("")
    }
}

impl FromStr for ErrorResponse {
    type Err = Error;
    fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
        serde_json::from_str(s).chain_err(|| "Failed to parse JSON")
    }
}

#[derive(Debug, Deserialize)]
pub struct BadgeData {
    pub attributes: HashMap<String, Option<String>>,
    pub badge_type: String,
}

#[derive(Debug, Deserialize)]
pub struct CategoryData {
    pub category: String,
    pub crates_cnt: i32,
    pub created_at: DateTime<Utc>,
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
    pub created_at: DateTime<Utc>,
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
    pub created_at: DateTime<Utc>,
    pub dl_path: String,
    pub downloads: i32,
    pub features: HashMap<String, Vec<String>>,
    pub id: i32,
    pub links: VersionLinks,
    pub num: String, // XXX should be semver::Version
    pub updated_at: DateTime<Utc>,
    pub yanked: bool,
}

#[derive(Debug, Deserialize)]
pub struct CrateData {
    pub badges: Option<Vec<BadgeData>>,
    pub categories: Option<Vec<String>>,
    pub created_at: DateTime<Utc>,
    pub description: Option<String>,
    pub documentation: Option<String>,
    pub downloads: i32,
    pub exact_match: bool,
    pub homepage: Option<String>,
    pub id: String,
    pub keywords: Option<Vec<String>>,
    pub license: Option<String>,
    pub links: CrateLinks,
    pub max_version: String,
    pub name: String,
    pub repository: Option<String>,
    pub updated_at: DateTime<Utc>,
    pub versions: Option<Vec<i32>>,
}

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub categories: Vec<CategoryData>,
    #[serde(rename(deserialize = "crate"))]
    pub krate: CrateData,
    pub keywords: Vec<KeywordData>,
    pub versions: Vec<VersionData>,
}

impl FromStr for ApiResponse {
    type Err = Error;
    fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
        serde_json::from_str(s).chain_err(|| "Failed to parse JSON")
    }
}

#[derive(Debug)]
pub struct CratesIO {
    response: reqwest::Response,
    body: String,
}

impl CratesIO {
    pub fn query(krate: &str) -> Result<Self> {
        let url = format!("https://crates.io/api/v1/crates/{}", krate);
        let mut response = reqwest::get(&url)?;
        let mut body = String::with_capacity(20480);
        response.read_to_string(&mut body)?;
        Ok(CratesIO {
            response: response,
            body: body,
        })
    }

    pub fn raw_data(&self) -> &str {
        &self.body
    }

    pub fn as_json(&self) -> Result<Value> {
        serde_json::from_str(&self.body).chain_err(|| "Failed to parse JSON")
        // serde_json::to_string_pretty(&json).chain_err(|| "Failed to prettify")
    }

    pub fn as_data(&self) -> Result<ApiResponse> {
        if *self.response.status() == reqwest::StatusCode::Ok {
            self.body.parse::<ApiResponse>()
        } else {
            self.body
                .parse::<ErrorResponse>()
                .and_then(|er| Err(ErrorKind::CratesIOError(er).into()))
        }
    }
}

use chrono::{DateTime, UTC};
use semver;
use serde_json;

use api::CratesIO;
use errors::*;

#[derive(Debug)]
pub struct Crate {
    pub id: String,
    pub name: String,
    pub updated_at: DateTime<UTC>,
    pub created_at: DateTime<UTC>,
    pub downloads: i32,
    pub max_version: String, // XXX Should be semver::Version
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub documentation: Option<String>,
    pub readme: Option<String>,
    pub license: Option<String>,
    pub repository: Option<String>,
    pub max_upload_size: Option<i32>,
}

impl Crate {
    pub fn json_data(name: &str) -> Result<String> {
        CratesIO::raw_data(name)
    }

    pub fn json_pretty(name: &str) -> Result<String> {
        let json = CratesIO::raw_data(name)?;
        let json = serde_json::from_str::<serde_json::Value>(&json)?;
        serde_json::to_string_pretty(&json).chain_err(|| "Failed to prettify")
    }

    pub fn by_name(name: &str) -> Result<Self> {
        let data = CratesIO::by_name(name)?;
        Ok(Crate {
            id: data.krate.id,
            name: data.krate.name,
            updated_at: data.krate.updated_at,
            created_at: data.krate.created_at,
            downloads: data.krate.downloads,
            max_version: data.krate.max_version,
            description: data.krate.description,
            homepage: data.krate.homepage,
            documentation: data.krate.documentation,
            readme: None,
            license: data.krate.license,
            repository: data.krate.repository,
            max_upload_size: None,
        })
    }
}

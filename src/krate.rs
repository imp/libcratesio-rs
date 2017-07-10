use std::collections::HashMap;
use std::ops::Deref;
use std::slice::Iter;

use chrono::{DateTime, Utc};

use api::{CratesIO, ApiResponse, VersionData};
use errors::*;

#[derive(Debug)]
pub struct Version {
    pub num: String, // XXX should be semver::Version
    pub downloads: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub features: HashMap<String, Vec<String>>,
    pub yanked: bool,
    pub license: Option<String>,
}

impl Version {
    fn from_versiondata(data: &VersionData) -> Self {
        Version {
            num: data.num.clone(),
            downloads: data.downloads,
            created_at: data.created_at,
            updated_at: data.updated_at,
            features: data.features.clone(),
            yanked: data.yanked,
            license: data.license.clone(),
        }
    }
}

#[derive(Debug)]
pub struct Versions {
    versions: Vec<Version>,
}

impl Versions {
    fn by_id(id: i32, versions: &[VersionData]) -> Option<&VersionData> {
        for v in versions {
            if id == v.id {
                return Some(v);
            }
        }
        None
    }

    fn from_crate_data(data: &ApiResponse) -> Self {
        let mut versions = Vec::new();
        if let Some(vers) = data.krate.versions.as_ref() {
            for id in vers {
                if let Some(versiondata) = Versions::by_id(*id, &data.versions) {
                    versions.push(Version::from_versiondata(versiondata))
                }
            }
        }
        Versions { versions: versions }
    }

    pub fn iter(&self) -> Iter<Version> {
        self.versions.iter()
    }
}

impl Deref for Versions {
    type Target = [Version];
    fn deref(&self) -> &[Version] {
        &self.versions
    }
}

#[derive(Debug)]
pub struct Crate {
    pub id: String,
    pub name: String,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub downloads: i32,
    pub max_version: String, // XXX Should be semver::Version
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub documentation: Option<String>,
    pub readme: Option<String>,
    pub license: Option<String>,
    pub repository: Option<String>,
    pub max_upload_size: Option<i32>,
    pub versions: Versions,
    pub keywords: Vec<String>,
}

impl Crate {
    pub fn from_apiresponse(data: &ApiResponse) -> Result<Self> {
        let versions = Versions::from_crate_data(data);
        let keywords = data.krate.keywords.clone().unwrap_or_default();
        Ok(Crate {
            id: data.krate.id.clone(),
            name: data.krate.name.clone(),
            updated_at: data.krate.updated_at,
            created_at: data.krate.created_at,
            downloads: data.krate.downloads,
            max_version: data.krate.max_version.clone(),
            description: data.krate.description.clone(),
            homepage: data.krate.homepage.clone(),
            documentation: data.krate.documentation.clone(),
            readme: None,
            license: data.krate.license.clone(),
            repository: data.krate.repository.clone(),
            max_upload_size: None,
            versions: versions,
            keywords: keywords,
        })
    }

    pub fn by_name(name: &str) -> Result<Self> {
        let data = CratesIO::query(name)?.as_data()?;
        Crate::from_apiresponse(&data)
    }
}

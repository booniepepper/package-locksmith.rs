//! A representation of a [uv.lock](https://docs.astral.sh/uv/concepts/projects/layout/#the-lockfile) file.

// Reference: https://github.com/astral-sh/uv/blob/00efde06b61756f0f305fcf67b12db71a29063d3/crates/uv-resolver/src/lock/mod.rs#L111

use std::{collections::HashMap, path::Path};

use anyhow::Result;
use packageurl::PackageUrl;
use serde::{self, Deserialize};
use toml::Table;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct UvLock {
    pub version: u32,
    pub revision: u32,
    pub requires_python: String,
    pub resolution_markers: Option<Vec<String>>,
    #[serde(rename = "package")]
    pub packages: Vec<Package>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Package {
    pub name: String,
    pub version: Option<String>,
    pub source: Source,
    pub dependencies: Option<Vec<Dependency>>,
    pub sdist: Option<SDist>,
    pub wheels: Option<Vec<Wheel>>,
    pub dev_dependencies: Option<HashMap<String, Vec<Dependency>>>,
    pub optional_dependencies: Option<HashMap<String, Vec<Dependency>>>,
    pub metadata: Option<Table>,
}

#[derive(Clone, Debug, Deserialize)]
pub enum Source {
    #[serde(rename = "registry")]
    Registry(String),
    #[serde(rename = "virtual")]
    Virtual(String),
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SDist {
    pub url: String,
    pub hash: String,
    pub size: u32,
    pub upload_time: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Wheel {
    pub url: String,
    pub hash: String,
    pub size: u32,
    pub upload_time: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Dependency {
    pub name: String,
    pub marker: Option<String>,
}

impl UvLock {
    pub fn load<T>(filename: T) -> Result<UvLock>
    where
        T: AsRef<Path>,
    {
        let content = std::fs::read_to_string(filename)?;
        UvLock::from(&content)
    }

    // TODO: Actually implement std::str::FromStr (?)
    pub fn from(contents: &str) -> Result<UvLock> {
        Ok(toml::from_str(contents)?)
    }
}

/// Convert a `pylock.toml` into a Package URL (PURL).
///
/// Refer to: https://github.com/package-url/purl-spec/blob/main/types-doc/pypi-definition.md
impl From<Package> for PackageUrl<'_> {
    fn from(pkg: Package) -> Self {
        let mut purl = PackageUrl::new("pypi", pkg.name).unwrap();
        let purl = match pkg.version {
            Some(version) => purl.with_version(version),
            None => purl.without_version(),
        };

        purl.to_owned()
    }
}

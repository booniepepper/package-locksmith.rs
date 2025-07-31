//! A representation of a [pylock.toml](https://packaging.python.org/en/latest/specifications/pylock-toml/) file,
//! as introduced in [PEP 751](https://peps.python.org/pep-0751/).

use anyhow::Result;
use packageurl::PackageUrl;
use serde::{self, Deserialize};
use std::{collections::HashMap, path::Path};
use toml::Table;

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "lock-version")]
pub enum PylockToml {
    #[serde(rename = "1.0")]
    V1(PylockTomlV1),
}

#[derive(Clone, Debug, Deserialize)]
pub struct PylockTomlV1 {
    pub packages: Vec<Package>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Package {
    pub name: String,
    pub version: Option<String>,
    pub marker: Option<String>,
    pub requires_python: Option<String>,
    pub dependencies: Option<Vec<Package>>,

    #[serde(flatten)]
    pub source: Option<Source>,

    pub archive: Option<Archive>,
    pub index: Option<String>,
    pub attestation_identities: Option<Vec<AttestationIdentities>>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase", rename_all_fields = "kebab-case")]
pub enum Source {
    Vcs(Vcs),
    Directory(Directory),
    SDist(SDist),
    Wheel(Wheel),
}

#[derive(Clone, Debug, Deserialize)]
pub struct Vcs {
    pub r#type: String,
    pub location: Location,
    pub requested_revision: Option<String>,
    pub commit_id: String,
    pub subdirectory: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Directory {
    pub path: String,
    pub editable: Option<bool>,
    pub subdirectory: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SDist {
    pub name: Option<String>,
    pub upload_time: Option<String>,
    pub location: Location,
    pub size: Option<i128>,
    pub hashes: HashMap<String, String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Wheel {
    pub name: Option<String>,
    pub upload_time: Option<String>,
    pub location: Location,
    pub size: Option<i128>,
    pub hashes: HashMap<String, String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Archive {
    pub location: Location,
    pub size: Option<i128>,
    pub upload_time: Option<String>,
    pub hashes: HashMap<String, String>,
    pub subdirectory: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub enum Location {
    #[serde(rename = "path")]
    Path(String),

    #[serde(rename = "url")]
    Url(String),
}

#[derive(Clone, Debug, Deserialize)]
pub struct AttestationIdentities {
    pub kind: String,
    pub tool: Option<Table>,
}

impl PylockToml {
    /// Returns true iff filename follows [the specification](https://packaging.python.org/en/latest/specifications/pylock-toml/)
    /// for a `pylock.toml` format file. If the input is a path to a file, directories will be stripped, and only the file name
    /// (i.e. basename) will be evaluated as a file name.
    ///
    /// Useful in detecting whether a file may be a `pylock.toml` format file.
    pub fn is_standard_filename<T>(filename: T) -> bool
    where
        T: AsRef<Path>,
    {
        if let Some(filename) = filename.as_ref().file_name() {
            // We don't worry about inner contents of the file name, only prefix/suffix
            let filename = filename.to_string_lossy();

            filename.len() >= 11 && filename.starts_with("pylock.") && filename.ends_with(".toml")
        } else {
            false
        }
    }

    pub fn load<T>(filename: T) -> Result<PylockToml>
    where
        T: AsRef<Path>,
    {
        let content = std::fs::read_to_string(filename)?;
        PylockToml::from(&content)
    }

    // TODO: Actually implement std::str::FromStr (?)
    pub fn from(contents: &str) -> Result<PylockToml> {
        Ok(toml::from_str(contents)?)
    }

    pub fn packages(&self) -> Vec<Package> {
        match self {
            PylockToml::V1(pylock) => pylock.packages.clone(),
        }
    }
}

/// Convert a `pylock.toml` into a Package URL (PURL).
///
/// Refer to: https://github.com/package-url/purl-spec/blob/main/types-doc/pypi-definition.md
impl From<Package> for PackageUrl<'_> {
    fn from(pkg: Package) -> Self {
        let mut purl = PackageUrl::new("pypi", pkg.name).unwrap();
        let mut purl = match pkg.version {
            Some(version) => purl.with_version(version),
            None => purl.without_version(),
        };

        if let Some(source) = pkg.source {
            if let Source::Wheel(wheel) = source {
                if let Some(name) = wheel.name {
                    purl = purl.add_qualifier("file_name", name).unwrap()
                }
            } else if let Source::SDist(sdist) = source {
                if let Some(name) = sdist.name {
                    purl = purl.add_qualifier("file_name", name).unwrap()
                }
            }
        }

        purl.to_owned()
    }
}

#[cfg(test)]
mod standard_filename_tests {
    use crate::PylockToml;

    #[test]
    fn valid_filenames() {
        let paths = ["", "/", "/dir/", "rel/", "/some/long/path/to/"];

        let filenames = [
            "pylock.toml",
            "pylock.z.toml",
            "pylock.some.name.toml",
            "pylock.何でもいいかな.toml",
        ];

        for filepath in filenames
            .into_iter()
            .flat_map(|filename| paths.into_iter().map(|path| String::from(path) + filename))
        {
            assert!(
                PylockToml::is_standard_filename(&filepath),
                "Expected {filepath:?} to be valid pylock.toml filename."
            );
        }
    }

    #[test]
    fn invalid_filenames() {
        let filenames = [
            "pylocktoml",
            "pylock-z.toml",
            "pylock-some-name.toml",
            "pylock.toml/wat",
        ];

        for filename in filenames {
            assert!(
                !PylockToml::is_standard_filename(&filename),
                "Expected {filename:?} to be invalid pylock.toml filename."
            );
        }
    }
}

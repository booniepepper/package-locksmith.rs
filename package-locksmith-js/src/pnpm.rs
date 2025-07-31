use std::path::Path;

use super::npm::NpmPackage;
use yaml_peg::repr::RcRepr;

// TODO: Flesh out. Most of the document is left unparsed
#[derive(Debug)]
pub struct PnpmLock {
    pub lockfile_version: String,
    pub packages: Vec<NpmPackage>,
}

impl PnpmLock {
    pub fn load(file: &Path) -> Result<Self, std::io::Error> {
        let contents = std::fs::read_to_string(file)?;

        let yaml = yaml_peg::parse::<RcRepr>(&contents).expect("Unable to parse YAML");
        let yaml = &yaml[0];

        let lockfile_version = yaml
            .get("lockfileVersion")
            .expect("Expect there to be lockfileVersion");
        let lockfile_version = lockfile_version
            .as_str()
            .map(|v| v.to_string())
            .expect("Expect lockfileVersion to be a string");

        let packages = yaml.get("packages").expect("Expect there to be packages");
        let packages = packages.as_map().expect("Expect packages to be a map.");
        let packages = packages
            .keys()
            .map(|key| key.as_str().expect("Expect all package keys to be strings"))
            .filter_map(|package| NpmPackage::try_from(package).ok())
            .collect::<Vec<_>>();

        Ok(PnpmLock {
            lockfile_version,
            packages,
        })
    }
}

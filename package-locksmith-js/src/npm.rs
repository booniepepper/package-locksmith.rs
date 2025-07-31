// TODO: Add
#[derive(Debug)]
pub struct NpmPackage {
    pub scope: Option<String>,
    pub package_name: String,
    pub version: String,
}

#[derive(Debug)]
pub enum ParseError {
    InvalidNpmPackageName(String),
}

impl TryFrom<&str> for NpmPackage {
    type Error = ParseError;

    fn try_from(specifier: &str) -> Result<Self, Self::Error> {
        let parts = specifier.split('/').collect::<Vec<_>>();

        let (scope, parts) = match parts[..] {
            [scope, rest] => (Some(scope.to_string()), rest),
            [rest] => (None, rest),
            _ => {
                return Err(ParseError::InvalidNpmPackageName(
                    "Too many \"/\" characters in Npm specifier.".to_string(),
                ));
            }
        };

        let parts = parts.split('@').collect::<Vec<_>>();

        let (package_name, version) = match parts[..] {
            [package_name, version] => (package_name.to_string(), version.to_string()),
            _ => {
                return Err(ParseError::InvalidNpmPackageName(
                    "Should be exactly one \"@\" in non-scope part of NPM specifier.".to_string(),
                ));
            }
        };

        Ok(Self {
            scope,
            package_name,
            version,
        })
    }
}

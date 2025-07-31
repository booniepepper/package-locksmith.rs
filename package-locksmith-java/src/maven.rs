// TODO: Support the chains maven lockfile (https://github.com/chains-project/maven-lockfile)

// https://maven.apache.org/pom.html#Maven_Coordinates

#[derive(Debug)]
pub struct MavenCoordinate {
    pub group_id: String,
    pub artifact_id: String,
    pub version: String,
}

// Alias for a common abbreviation in JVM world.
pub type GAV = MavenCoordinate;

#[derive(Debug)]
pub enum ParseError {
    InvalidMavenCoordinate(String),
}

impl TryFrom<&str> for MavenCoordinate {
    type Error = ParseError;

    fn try_from(coordinate: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = coordinate.split(':').collect();
        match &parts[..] {
            [group_id, artifact_id, version] => Ok(Self {
                group_id: group_id.to_string(),
                artifact_id: artifact_id.to_string(),
                version: version.to_string(),
            }),
            _ => Err(ParseError::InvalidMavenCoordinate(coordinate.to_string())),
        }
    }
}

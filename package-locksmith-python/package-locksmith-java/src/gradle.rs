use std::path::Path;

use super::maven::MavenCoordinate;

#[derive(Debug)]
pub struct GradleLock {
    pub dependencies: Vec<Dependency>,
}

#[derive(Debug)]
pub struct Dependency {
    pub coordinate: MavenCoordinate,
    pub closures: Vec<String>, // compileClasspath ,runtimeClasspath, etc
}

impl GradleLock {
    pub fn load(path: &Path) -> Result<Self, std::io::Error> {
        let contents = std::fs::read_to_string(path)?;

        // https://docs.gradle.org/current/userguide/dependency_locking.html
        let dependencies = contents
            .lines()
            .enumerate()
            .filter(|(_, line)| !line.is_empty() && !line.starts_with('#')) // Drop empty/comment lines.
            .filter_map(|(line_num, line)| {
                let parts: Vec<&str> = line.split("=").collect();
                if parts.len() != 2 {
                    eprintln!("[ERROR] File {path:?}: Line {line_num} appears malformed.");
                    return None;
                }

                let coordinate = match MavenCoordinate::try_from(parts[0]) {
                    Ok(coordinate) => coordinate,
                    Err(error) => {
                        eprintln!("[ERROR] File {path:?}: Line {line_num} contains invalid maven coordinate ({error:?})");
                        return None;
                    }
                };

                let closures = parts[1].split(",").map(|closure| closure.to_string()).collect::<Vec<_>>();

                Some(Dependency { coordinate, closures })
            })
            .collect();

        Ok(Self { dependencies })
    }
}

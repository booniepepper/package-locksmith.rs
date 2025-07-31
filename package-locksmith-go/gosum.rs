use std::path::Path;

#[derive(Debug)]
pub struct GoSum {
    pub dependencies: Vec<Dependency>,
}

#[derive(Debug)]
pub struct Dependency {
    pub module_path: String,
    pub version: String,
    pub hash: DependencyHash,
}

#[derive(Debug)]
pub struct DependencyHash {
    pub algorithm: Algorithm,
    pub hash_of: HashType,
    pub hash: String,
}

#[derive(Debug)]
pub enum Algorithm {
    SHA256,
}

#[derive(Debug)]
pub enum HashType {
    Mod,
    Zip,
}

impl GoSum {
    pub fn load(path: &Path) -> Result<Self, std::io::Error> {
        let contents = std::fs::read_to_string(path)?;

        // https://go.dev/ref/mod#go-sum-files
        // https://sum.golang.org/
        let dependencies = contents
            .lines()
            .enumerate()
            .filter_map(|(line_num, line)| {
                let chunks: Vec<&str> = line.split_ascii_whitespace().collect();

                if chunks.len() != 3 {
                    eprintln!("[ERROR] File {path:?}: Line {line_num} appears malformed");
                    return None;
                }

                let module_path = chunks[0].to_string();
                let version = chunks[1];
                let hash = chunks[2];

                let (hash_of, version) = match version.ends_with("/go.mod") {
                    true => (
                        HashType::Mod,
                        version.strip_suffix("/go.mod").unwrap().to_string(),
                    ),
                    false => (HashType::Zip, version.to_string()),
                };

                let hash_parts = hash.split(':').collect::<Vec<_>>();
                let (algorithm, hash) = match hash_parts[..] {
                    ["h1", hash] => (Algorithm::SHA256, hash.to_string()),
                    _ => {
                        eprintln!(
                            "[ERROR] File {path:?}: Unknown hashing algorithm for hash {hash:?}"
                        );
                        return None;
                    }
                };

                Some(Dependency {
                    module_path,
                    version,
                    hash: DependencyHash {
                        algorithm,
                        hash_of,
                        hash,
                    },
                })
            })
            .collect();

        Ok(GoSum { dependencies })
    }
}

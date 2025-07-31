//! A representation of a [requirements.txt](https://pip.pypa.io/en/stable/reference/requirements-file-format/) file.

use std::path::Path;

use anyhow::Result;
use pest::Parser;
use pest_derive::Parser;

#[derive(Clone, Debug, Parser)]
#[grammar = "requirements.txt.pest"]
pub struct RequirementsTxt {
    pub requirement_specifiers: Vec<RequirementSpecifier>,
}

#[derive(Clone, Debug)]
pub struct RequirementSpecifier {
    pub package: String,
    pub version_specifier: Vec<VersionClause>,
}

#[derive(Clone, Debug)]
pub struct VersionClause {
    pub comparison: Comparison,
    pub version: String,
}

#[derive(Clone, Debug)]
pub enum Comparison {
    /// `~=`: [Compatible release](https://packaging.python.org/en/latest/specifications/version-specifiers/#compatible-release).
    CompatibleRelease,
    /// `==`: [Version matching](https://packaging.python.org/en/latest/specifications/version-specifiers/#version-matching).
    VersionMatching,
    /// `!=`: [Version exclusion](https://packaging.python.org/en/latest/specifications/version-specifiers/#version-exclusion).
    VersionExclusion,
    /// `<=`: [Inclusive ordered comparison](https://packaging.python.org/en/latest/specifications/version-specifiers/#inclusive-ordered-comparison).
    LessThanInclusive,
    /// `>=`: [Inclusive ordered comparison](https://packaging.python.org/en/latest/specifications/version-specifiers/#inclusive-ordered-comparison).
    GreaterThanInclusive,
    /// `<`: [Exclusive ordered comparison](https://packaging.python.org/en/latest/specifications/version-specifiers/#exclusive-ordered-comparison).
    LessThanExclusive,
    /// `>`: [Exclusive ordered comparison](https://packaging.python.org/en/latest/specifications/version-specifiers/#exclusive-ordered-comparison).
    GreaterThanExclusive,
    /// `===`: [Arbitrary equality](https://packaging.python.org/en/latest/specifications/version-specifiers/#arbitrary-equality).
    ArbitraryEquality,
}

impl RequirementsTxt {
    pub fn load<F>(file: F) -> Result<RequirementsTxt>
    where
        F: AsRef<Path>,
    {
        let contents = std::fs::read_to_string(file)?;
        RequirementsTxt::load_contents(&contents)
    }

    pub fn load_contents(contents: &str) -> Result<RequirementsTxt> {
        let document = RequirementsTxt::parse(Rule::document, contents.into())?;

        eprintln!("[DEBUG] document: {document:#?}");

        let document = document.into_iter().next().unwrap();

        let mut requirements_txt = RequirementsTxt {
            requirement_specifiers: vec![],
        };

        for line in document.into_inner() {
            if let Rule::requirement_specifier = line.as_rule() {
                let mut rs_pairs = line.into_inner().into_iter();
                let package = rs_pairs.next().unwrap().as_span().as_str().to_string();

                let version_specifier = match rs_pairs.next().map(|pair| pair.into_inner()) {
                    Some(version_specifier) => vec![], // version_specifier.map(||),
                    None => vec![],
                };

                for version_clause in &version_specifier {
                    eprintln!("VERSION_CLAUSE! {version_clause:#?}");
                }

                requirements_txt.requirement_specifiers.push(
                    RequirementSpecifier { package, version_specifier }
                );
            }
            else {
                eprintln!("WHAT RULE IS IT ANYWAY: {:#?}", line.as_rule());
            }
        }

        Ok(requirements_txt)
    }
}

#[cfg(test)]
mod tests {
    use crate::requirements_txt::RequirementsTxt;

    #[test]
    fn simple_relations_no_ending_newline() {
        let contents = [
            "numpy>=1.13.3",
            "scipy>=1.0.0",
            "pandas>=0.21.0",
            "numdifftools>=0.9.20",
        ]
        .join("\n");

        let requirements_txt = RequirementsTxt::load_contents(&contents).unwrap();

        assert_eq!(requirements_txt.requirement_specifiers.len(), 4);
    }
}

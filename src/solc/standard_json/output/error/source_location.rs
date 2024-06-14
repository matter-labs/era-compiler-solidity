//!
//! The `solc --standard-json` output error source location.
//!

use std::str::FromStr;

use serde::Deserialize;
use serde::Serialize;

///
/// The `solc --standard-json` output error source location.
///
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SourceLocation {
    /// The source file path.
    pub file: String,
    /// The start location.
    pub start: isize,
    /// The end location.
    pub end: isize,
}

impl SourceLocation {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(file: String) -> Self {
        Self {
            file,
            start: -1,
            end: -1,
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn new_with_location(file: String, start: isize, end: isize) -> Self {
        Self { file, start, end }
    }

    ///
    /// Resolves the location, converting absolute offsets to line and column.
    ///
    pub fn resolve(&self) -> String {
        self.file.to_owned()
    }
}

impl FromStr for SourceLocation {
    type Err = anyhow::Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut parts = string.split(':');
        let start = parts
            .next()
            .map(|string| string.parse::<isize>())
            .and_then(Result::ok)
            .unwrap_or_default();
        let length = parts
            .next()
            .map(|string| string.parse::<isize>())
            .and_then(Result::ok)
            .unwrap_or_default();
        let file = parts.next().unwrap_or_default().to_owned();

        Ok(Self::new_with_location(file, start, start + length))
    }
}

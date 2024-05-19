//!
//! The `solc --standard-json` input source.
//!

use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

///
/// The `solc --standard-json` input source.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    /// The source code file content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// The source file URLs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<String>>,
}

impl TryInto<String> for Source {
    type Error = anyhow::Error;

    fn try_into(self) -> anyhow::Result<String> {
        match (self.content, self.urls) {
            (Some(content), None) => Ok(content),
            (None, Some(mut urls)) => {
                let url = match urls.pop() {
                    Some(url) => url,
                    None => anyhow::bail!("The URL list is empty"),
                };
                if !urls.is_empty() {
                    anyhow::bail!("Only one source code URL is allowed");
                }

                let url_path = PathBuf::from(url);
                let source_with_content = Self::try_from(url_path.as_path())?;
                Ok(source_with_content.content.expect("Always exists"))
            }
            (Some(_), Some(_)) => anyhow::bail!("Both `content` and `urls` cannot be set"),
            (None, None) => anyhow::bail!("Either `content` or `urls` must be set"),
        }
    }
}

impl From<String> for Source {
    fn from(content: String) -> Self {
        Self {
            content: Some(content),
            urls: None,
        }
    }
}

impl TryFrom<&Path> for Source {
    type Error = anyhow::Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let content = if path.to_string_lossy() == "-" {
            let mut solidity_code = String::with_capacity(16384);
            std::io::stdin()
                .read_to_string(&mut solidity_code)
                .map_err(|error| anyhow::anyhow!("<stdin> reading error: {}", error))?;
            solidity_code
        } else {
            std::fs::read_to_string(path)
                .map_err(|error| anyhow::anyhow!("File {:?} reading error: {}", path, error))?
        };

        Ok(Self {
            content: Some(content),
            urls: None,
        })
    }
}

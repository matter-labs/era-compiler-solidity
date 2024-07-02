//!
//! The `solc --standard-json` input source.
//!

use std::path::Path;
use std::path::PathBuf;

///
/// The `solc --standard-json` input source.
///
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    /// The source code file content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// The source file URLs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<String>>,
}

impl Source {
    ///
    /// Reads the source from the file system.
    ///
    pub fn try_read(path: &Path) -> anyhow::Result<Self> {
        let content = if path.to_string_lossy() == "-" {
            std::io::read_to_string(std::io::stdin())
                .map_err(|error| anyhow::anyhow!("<stdin> reading: {error}"))
        } else {
            std::fs::read_to_string(path)
                .map_err(|error| anyhow::anyhow!("File {path:?} reading: {error}"))
        }?;

        Ok(Self {
            content: Some(content),
            urls: None,
        })
    }

    ///
    /// Tries to resolve the source code.
    ///
    /// At the moment only one URL pointing to the file system is supported.
    ///
    pub fn try_resolve(self) -> anyhow::Result<Self> {
        match (self.content.as_ref(), self.urls.as_ref()) {
            (Some(_), None) => Ok(self),
            (None, Some(urls)) => {
                let url = match urls.first() {
                    Some(url) => url,
                    None => anyhow::bail!("The URL list is empty"),
                };

                let url_path = PathBuf::from(url);
                let source_with_content = Source::try_read(url_path.as_path())?;
                Ok(source_with_content)
            }
            (Some(_), Some(_)) => anyhow::bail!("Both `content` and `urls` cannot be set"),
            (None, None) => anyhow::bail!("Either `content` or `urls` must be set"),
        }
    }

    ///
    /// Returns the source code reference, if the source has been previously read or resolved.
    ///
    pub fn content(&self) -> Option<&str> {
        self.content.as_deref()
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

impl From<&Path> for Source {
    fn from(path: &Path) -> Self {
        Self {
            content: None,
            urls: Some(vec![path.to_string_lossy().to_string()]),
        }
    }
}

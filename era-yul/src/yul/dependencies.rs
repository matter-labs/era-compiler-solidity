//!
//! Collection of Yul dependencies.
//!

///
/// This structure represents an ordered dependency collection
/// in the order they are encountered in Yul from the top to the bottom.
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Dependencies {
    /// Top-level object identifier.
    pub identifier: String,
    /// List of EVM-like dependencies.
    pub inner: Vec<String>,
}

impl Dependencies {
    ///
    /// Create a new instance of dependencies.
    ///
    pub fn new(identifier: &str) -> Self {
        Self {
            identifier: identifier.to_owned(),
            inner: Vec::new(),
        }
    }

    ///
    /// Push a single dependency.
    ///
    /// TODO: move suffixes to Yul/EVMLA translators
    ///
    pub fn push(&mut self, dependency: String) {
        if dependency == self.identifier || self.inner.contains(&dependency) {
            return;
        }

        let is_runtime_code = self.identifier
            == format!(
                "{}.deploy",
                dependency
                    .strip_suffix(".runtime")
                    .unwrap_or(&self.identifier)
            )
            || self.identifier.as_str()
                == dependency
                    .strip_suffix("_deployed")
                    .unwrap_or(self.identifier.as_str());
        if is_runtime_code {
            self.inner.insert(0, dependency);
        } else {
            self.inner.push(dependency);
        }
    }

    ///
    /// Extend with multiple dependencies.
    ///
    pub fn extend<I>(&mut self, dependencies: I)
    where
        I: IntoIterator<Item = String>,
    {
        for dependency in dependencies.into_iter() {
            self.push(dependency);
        }
    }
}

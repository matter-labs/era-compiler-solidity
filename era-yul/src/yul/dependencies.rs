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
    pub dependencies: Vec<String>,
}

impl Dependencies {
    ///
    /// Create a new instance of dependencies.
    ///
    pub fn new(identifier: &str) -> Self {
        Self {
            identifier: identifier.to_owned(),
            dependencies: Vec::new(),
        }
    }

    ///
    /// Push a single dependency.
    ///
    pub fn push(&mut self, dependency: String) {
        if dependency == self.identifier || self.dependencies.contains(&dependency) {
            return;
        }

        let dependency_after_dot = dependency
            .split('.')
            .last()
            .expect("Always exists")
            .to_owned();
        self.dependencies.push(dependency_after_dot);
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

impl IntoIterator for Dependencies {
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.dependencies.into_iter()
    }
}

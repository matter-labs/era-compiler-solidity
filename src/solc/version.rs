//!
//! The Solidity compiler version representation.
//!

///
/// The Solidity compiler version representation.
///
pub struct Version {
    /// The long version string.
    pub long: String,
    /// The short `semver` representation.
    pub default: semver::Version,
}

impl Version {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(long: String, default: semver::Version) -> Self {
        Self { long, default }
    }
}

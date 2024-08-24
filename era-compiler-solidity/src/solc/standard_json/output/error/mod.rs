//!
//! The `solc --standard-json` output error.
//!

pub mod collectable;
pub mod mapped_location;
pub mod source_location;

use std::collections::BTreeMap;

use crate::solc::standard_json::input::source::Source as StandardJSONInputSource;

use self::mapped_location::MappedLocation;
use self::source_location::SourceLocation;

///
/// The `solc --standard-json` output error.
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Error {
    /// The component type.
    pub component: String,
    /// The error code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// The formatted error message.
    pub formatted_message: String,
    /// The non-formatted error message.
    pub message: String,
    /// The error severity.
    pub severity: String,
    /// The error location data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_location: Option<SourceLocation>,
    /// The error type.
    pub r#type: String,
}

impl Error {
    /// The list of ignored `solc` warnings that are strictly EVM-related.
    pub const IGNORED_WARNING_CODES: [&'static str; 5] = ["1699", "3860", "5159", "5574", "6417"];

    /// The default size of an error line.
    pub const DEFAULT_ERROR_LINE_LENGTH: usize = 96;

    ///
    /// A shortcut constructor.
    ///
    pub fn new<S>(
        r#type: &str,
        message: S,
        source_location: Option<SourceLocation>,
        sources: Option<&BTreeMap<String, StandardJSONInputSource>>,
    ) -> Self
    where
        S: std::fmt::Display,
    {
        let message = message.to_string();

        let message_trimmed = message.trim();
        let mut formatted_message = if message_trimmed.starts_with(r#type) {
            message_trimmed.to_owned()
        } else {
            format!("{}: {}", r#type, message_trimmed)
        };
        formatted_message.push('\n');
        if let Some(ref source_location) = source_location {
            let source_code = sources.and_then(|sources| {
                sources
                    .get(source_location.file.as_str())
                    .and_then(|source| source.content())
            });
            let mapped_location =
                MappedLocation::try_from_source_location(source_location, source_code);
            formatted_message.push_str(mapped_location.to_string().as_str());
            formatted_message.push('\n');
        }

        Self {
            component: "general".to_owned(),
            error_code: None,
            formatted_message,
            message,
            severity: r#type.to_lowercase(),
            source_location,
            r#type: r#type.to_owned(),
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn new_error<S>(
        message: S,
        source_location: Option<SourceLocation>,
        sources: Option<&BTreeMap<String, StandardJSONInputSource>>,
    ) -> Self
    where
        S: std::fmt::Display,
    {
        Self::new("Error", message, source_location, sources)
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn new_warning<S>(
        message: S,
        source_location: Option<SourceLocation>,
        sources: Option<&BTreeMap<String, StandardJSONInputSource>>,
    ) -> Self
    where
        S: std::fmt::Display,
    {
        Self::new("Warning", message, source_location, sources)
    }

    ///
    /// Returns the `origin` instruction usage warning.
    ///
    pub fn warning_tx_origin(
        node: Option<&str>,
        id_paths: &BTreeMap<usize, &String>,
        sources: &BTreeMap<String, StandardJSONInputSource>,
    ) -> Self {
        let message = r#"
You are checking for 'tx.origin', which might lead to unexpected behavior.

ZKsync Era comes with native account abstraction support, and therefore the initiator of a
transaction might be different from the contract calling your code. It is highly recommended NOT
to rely on tx.origin, but use msg.sender instead.

Learn more about Account Abstraction at https://docs.zksync.io/build/developer-reference/account-abstraction/

You may disable this warning with:
    a. `suppressedWarnings = ["txorigin"]` in standard JSON.
    b. `--suppress-warnings txorigin` in the CLI.
"#;

        Self::new_warning(
            message,
            node.and_then(|node| SourceLocation::try_from_ast(node, id_paths)),
            Some(sources),
        )
    }

    ///
    /// Returns the `<address payable>`'s `send` and `transfer` methods usage error.
    ///
    pub fn error_send_and_transfer(
        node: Option<&str>,
        id_paths: &BTreeMap<usize, &String>,
        sources: &BTreeMap<String, StandardJSONInputSource>,
    ) -> Self {
        let message = r#"
You are using '<address payable>.send/transfer(<X>)' without providing the gas amount.
Such calls will fail depending on the pubdata costs.

Please use 'payable(<address>).call{value: <X>}("")' instead, but be careful with the
reentrancy attack. `send` and `transfer` send limited amount of gas that prevents reentrancy,
whereas `<address>.call{value: <X>}` sends all gas to the callee.

In Solidity v0.4, where there is no `payable` type, this may be a false positive
if `using X for address` is used with `X` implementing its own `send` or `transfer` functions.

Learn more about reentrancy at https://docs.soliditylang.org/en/latest/security-considerations.html#reentrancy

You may disable this error with:
    1. `suppressedErrors = ["sendtransfer"]` in standard JSON.
    2. `--suppress-errors sendtransfer` in the CLI.
"#;

        Self::new_error(
            message,
            node.and_then(|node| SourceLocation::try_from_ast(node, id_paths)),
            Some(sources),
        )
    }

    ///
    /// Returns the internal function pointer usage error.
    ///
    pub fn error_internal_function_pointer(
        node: Option<&str>,
        id_paths: &BTreeMap<usize, &String>,
        sources: &BTreeMap<String, StandardJSONInputSource>,
    ) -> Self {
        let message = r#"
Internal function pointers are not supported in the EVM assembly pipeline.
Please do one of the following:
    1. Use the ZKsync fork of the Solidity compiler: https://github.com/matter-labs/era-solidity/releases
    2. Switch to the latest solc with Yul assembly pipeline: https://docs.soliditylang.org/en/latest/yul.html
"#;

        Self::new_error(
            message,
            node.and_then(|node| SourceLocation::try_from_ast(node, id_paths)),
            Some(sources),
        )
    }

    ///
    /// Returns the runtime code usage error.
    ///
    pub fn error_runtime_code(
        node: Option<&str>,
        id_paths: &BTreeMap<usize, &String>,
        sources: &BTreeMap<String, StandardJSONInputSource>,
    ) -> Self {
        let message = r#"
Deploy and runtime code are merged together on ZKsync, so reading `type(T).runtimeCode` is not possible.
Please consider changing the functionality relying on reading runtime code to a different approach.
"#;

        Self::new_error(
            message,
            node.and_then(|node| SourceLocation::try_from_ast(node, id_paths)),
            Some(sources),
        )
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted_message)
    }
}

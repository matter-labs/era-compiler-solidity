//!
//! The `solc --standard-json` output error.
//!

pub mod source_location;

use std::collections::BTreeMap;

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
    /// The list of ignored `solc` warnings, which are strictly EVM-related.
    pub const IGNORED_WARNING_CODES: [&'static str; 5] = ["1699", "3860", "5159", "5574", "6417"];

    ///
    /// A shortcut constructor.
    ///
    pub fn new_error<S>(message: S, source_location: Option<SourceLocation>) -> Self
    where
        S: std::fmt::Display,
    {
        let r#type = "Error";
        let message = message.to_string();

        let mut formatted_message = format!("┌─┤ {type} ├{}┐", "─".repeat(93 - r#type.len()));
        formatted_message.push_str(message.as_str());
        formatted_message.push_str(format!("└{}┘", "─".repeat(98)).as_str());
        if let Some(ref source_location) = source_location {
            formatted_message.push_str(format!("--> {source_location}\n").as_str());
        }

        Self {
            component: "general".to_owned(),
            error_code: None,
            formatted_message: message.to_string(),
            message: message.to_string(),
            severity: "error".to_owned(),
            source_location,
            r#type: r#type.to_owned(),
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn new_warning<S>(message: S, source_location: Option<SourceLocation>) -> Self
    where
        S: std::fmt::Display,
    {
        let r#type = "Warning";
        let message = message.to_string();

        let mut formatted_message = format!("┌─┤ {type} ├{}┐", "─".repeat(93 - r#type.len()));
        formatted_message.push('\n');
        formatted_message.push_str(message.as_str());
        formatted_message.push('\n');
        formatted_message.push_str(format!("└{}┘", "─".repeat(98)).as_str());
        formatted_message.push('\n');
        if let Some(ref source_location) = source_location {
            formatted_message.push_str(format!("--> {source_location}").as_str());
            formatted_message.push('\n');
        }

        Self {
            component: "general".to_owned(),
            error_code: None,
            formatted_message,
            message,
            severity: "warning".to_owned(),
            source_location,
            r#type: r#type.to_owned(),
        }
    }

    ///
    /// Returns the `ecrecover` function usage warning.
    ///
    pub fn warning_ecrecover(node: Option<&str>, id_paths: &BTreeMap<usize, &String>) -> Self {
        let message = r#"│ It looks like you are using 'ecrecover' to validate a signature of a user account.               │
│ ZKsync Era comes with native account abstraction support, therefore it is highly recommended NOT │
│ to rely on the fact that the account has an ECDSA private key attached to it since accounts      │
│ might implement other signature schemes.                                                         │
│ Read more about Account Abstraction at https://v2-docs.zksync.io/dev/developer-guides/aa.html    │"#;

        Self::new_warning(
            message,
            node.and_then(|node| SourceLocation::try_from_ast(node, id_paths)),
        )
    }

    ///
    /// Returns the `<address payable>`'s `send` and `transfer` methods usage error.
    ///
    pub fn warning_send_and_transfer(
        node: Option<&str>,
        id_paths: &BTreeMap<usize, &String>,
    ) -> Self {
        let message = r#"│ It looks like you are using '<address payable>.send/transfer(<X>)' without providing the gas     │
│ amount. Such calls will fail depending on the pubdata costs.                                     │
│ Please use 'payable(<address>).call{value: <X>}("")' instead, but be careful with the reentrancy │
│ attack. `send` and `transfer` send limited amount of gas that prevents reentrancy, whereas       │
│ `<address>.call{value: <X>}` sends all gas to the callee.                                        │
│ Learn more on https://docs.soliditylang.org/en/latest/security-considerations.html#reentrancy    │"#;

        Self::new_warning(
            message,
            node.and_then(|node| SourceLocation::try_from_ast(node, id_paths)),
        )
    }

    ///
    /// Returns the `extcodesize` instruction usage warning.
    ///
    pub fn warning_extcodesize(node: Option<&str>, id_paths: &BTreeMap<usize, &String>) -> Self {
        let message = r#"│ Your code or one of its dependencies uses the 'extcodesize' instruction, which is usually needed │
│ in the following cases:                                                                          │
│   1. To detect whether an address belongs to a smart contract.                                   │
│   2. To detect whether the deploy code execution has finished.                                   │
│ ZKsync Era comes with native account abstraction support (so accounts are smart contracts,       │
│ including private-key controlled EOAs), and you should avoid differentiating between contracts   │
│ and non-contract addresses.                                                                      │"#;

        Self::new_warning(
            message,
            node.and_then(|node| SourceLocation::try_from_ast(node, id_paths)),
        )
    }

    ///
    /// Returns the `origin` instruction usage warning.
    ///
    pub fn warning_tx_origin(node: Option<&str>, id_paths: &BTreeMap<usize, &String>) -> Self {
        let message = r#"│ You are checking for 'tx.origin' in your code, which might lead to unexpected behavior.          │
│ ZKsync Era comes with native account abstraction support, and therefore the initiator of a       │
│ transaction might be different from the contract calling your code. It is highly recommended NOT │
│ to rely on tx.origin, but use msg.sender instead.                                                │
│ Read more about Account Abstraction at https://v2-docs.zksync.io/dev/developer-guides/aa.html    │"#;

        Self::new_warning(
            message,
            node.and_then(|node| SourceLocation::try_from_ast(node, id_paths)),
        )
    }

    ///
    /// Returns the internal function pointer usage error.
    ///
    pub fn error_internal_function_pointer(
        node: Option<&str>,
        id_paths: &BTreeMap<usize, &String>,
    ) -> Self {
        let message = r#"│ Internal function pointers are not supported in EVM legacy assembly pipeline.                    │
│ Please use the latest solc with Yul codegen instead.                                             │"#;

        Self::new_error(
            message,
            node.and_then(|node| SourceLocation::try_from_ast(node, id_paths)),
        )
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted_message)
    }
}

//!
//! The `solc --standard-json` output error.
//!

pub mod source_location;

use std::str::FromStr;

use serde::Deserialize;
use serde::Serialize;

use self::source_location::SourceLocation;

///
/// The `solc --standard-json` output error.
///
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Error {
    /// The component type.
    pub component: String,
    /// The error code.
    pub error_code: Option<String>,
    /// The formatted error message.
    pub formatted_message: String,
    /// The non-formatted error message.
    pub message: String,
    /// The error severity.
    pub severity: String,
    /// The error location data.
    pub source_location: Option<SourceLocation>,
    /// The error type.
    pub r#type: String,
}

impl Error {
    ///
    /// Returns the `ecrecover` function usage warning.
    ///
    pub fn message_ecrecover(src: Option<&str>) -> Self {
        let message = r#"
┌──────────────────────────────────────────────────────────────────────────────────────────────────┐
│ Warning: It looks like you are using 'ecrecover' to validate a signature of a user account.      │
│ zkSync era comes with native account abstraction support, therefore it is highly recommended NOT │
│ to rely on the fact that the account has an ECDSA private key attached to it since accounts might│
│ implement other signature schemes.                                                               │
│ Read more about Account Abstraction at https://v2-docs.zksync.io/dev/developer-guides/aa.html    │
└──────────────────────────────────────────────────────────────────────────────────────────────────┘"#
            .to_owned();

        Self {
            component: "general".to_owned(),
            error_code: None,
            formatted_message: message.clone(),
            message,
            severity: "warning".to_owned(),
            source_location: src.map(SourceLocation::from_str).and_then(Result::ok),
            r#type: "Warning".to_owned(),
        }
    }

    ///
    /// Returns the `<address payable>`'s `send` and `transfer` methods usage error.
    ///
    pub fn message_send_and_transfer(src: Option<&str>) -> Self {
        let message = r#"
┌──────────────────────────────────────────────────────────────────────────────────────────────────┐
│ Warning: It looks like you are using '<address payable>.send/transfer(<X>)' without providing    │
│ the gas amount. Such calls will fail depending on the pubdata costs.                             │
│ This might be a false positive if you are using some interface (like IERC20) instead of the      │
│ native Solidity send/transfer                                                                    │
│ Please use 'payable(<address>).call{value: <X>}("")' instead.                                    │
└──────────────────────────────────────────────────────────────────────────────────────────────────┘"#
            .to_owned();

        Self {
            component: "general".to_owned(),
            error_code: None,
            formatted_message: message.clone(),
            message,
            severity: "warning".to_owned(),
            source_location: src.map(SourceLocation::from_str).and_then(Result::ok),
            r#type: "Warning".to_owned(),
        }
    }

    ///
    /// Returns the `extcodesize` instruction usage warning.
    ///
    pub fn message_extcodesize(src: Option<&str>) -> Self {
        let message = r#"
┌──────────────────────────────────────────────────────────────────────────────────────────────────┐
│ Warning: It looks like your code or one of its dependencies uses the 'extcodesize' instruction,  │
│ which is usually needed in the following cases:                                                  │
│   1. To detect whether an address belongs to a smart contract.                                   │
│   2. To detect whether the deploy code execution has finished.                                   │
│ zkSync era comes with native account abstraction support (so accounts are smart contracts,       │
│ including private-key controlled EOAs), and you should avoid differentiating between contracts   │
│ and non-contract addresses.                                                                      │
└──────────────────────────────────────────────────────────────────────────────────────────────────┘"#
            .to_owned();

        Self {
            component: "general".to_owned(),
            error_code: None,
            formatted_message: message.clone(),
            message,
            severity: "warning".to_owned(),
            source_location: src.map(SourceLocation::from_str).and_then(Result::ok),
            r#type: "Warning".to_owned(),
        }
    }

    ///
    /// Returns the `origin` instruction usage warning.
    ///
    pub fn message_origin(src: Option<&str>) -> Self {
        let message = r#"
┌──────────────────────────────────────────────────────────────────────────────────────────────────┐
│ Warning: It looks like you are checking for 'tx.origin' in your code, which might lead to        │
│ unexpected behavior. zkSync era comes with native account abstraction support, and therefore the │
│ initiator of a transaction might be different from the contract calling your code. It is highly  │
│ recommended NOT to rely on tx.origin, but use msg.sender instead.                                │
│ Read more about Account Abstraction at https://v2-docs.zksync.io/dev/developer-guides/aa.html    │
└──────────────────────────────────────────────────────────────────────────────────────────────────┘"#
            .to_owned();

        Self {
            component: "general".to_owned(),
            error_code: None,
            formatted_message: message.clone(),
            message,
            severity: "warning".to_owned(),
            source_location: src.map(SourceLocation::from_str).and_then(Result::ok),
            r#type: "Warning".to_owned(),
        }
    }

    ///
    /// Returns the `timestamp` instruction usage warning.
    ///
    pub fn message_timestamp(src: Option<&str>) -> Self {
        let message = r#"
┌──────────────────────────────────────────────────────────────────────────────────────────────────┐
│ Warning: It looks like you are checking for 'block.timestamp' in your code, which might lead to  │
│ unexpected behavior. Due to the nature of the zkEVM, the timestamp of a block actually refers to │
│ the timestamp of the whole batch that will be sent to L1 (meaning, the timestamp of this batch   │
│ started being processed).                                                                        │
│ We will provide a custom method to access the L2 block timestamp from the smart contract code in │
│ the future.                                                                                      │
└──────────────────────────────────────────────────────────────────────────────────────────────────┘"#
            .to_owned();

        Self {
            component: "general".to_owned(),
            error_code: None,
            formatted_message: message.clone(),
            message,
            severity: "warning".to_owned(),
            source_location: src.map(SourceLocation::from_str).and_then(Result::ok),
            r#type: "Warning".to_owned(),
        }
    }

    ///
    /// Returns the `number` usage warning.
    ///
    pub fn message_number(src: Option<&str>) -> Self {
        let message = r#"
┌──────────────────────────────────────────────────────────────────────────────────────────────────┐
│ Warning: It looks like you are checking for 'block.number' in your code, which might lead to     │
│ unexpected behavior. Due to the nature of the zkEVM, the number of a block actually refers to    │
│ the number of the whole batch will be sent to L1 (a sequentially generated batch number).        │
│ We will provide a custom method to access the L2 block number from the smart contract code in    │
│ the future.                                                                                      │
└──────────────────────────────────────────────────────────────────────────────────────────────────┘"#
            .to_owned();

        Self {
            component: "general".to_owned(),
            error_code: None,
            formatted_message: message.clone(),
            message,
            severity: "warning".to_owned(),
            source_location: src.map(SourceLocation::from_str).and_then(Result::ok),
            r#type: "Warning".to_owned(),
        }
    }

    ///
    /// Appends the contract path to the message..
    ///
    pub fn push_contract_path(&mut self, path: &str) {
        self.formatted_message
            .push_str(format!("\n--> {path}\n").as_str());
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted_message)
    }
}

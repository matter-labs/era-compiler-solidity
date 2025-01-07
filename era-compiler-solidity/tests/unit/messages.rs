//!
//! Unit tests for compiler errors and warnings.
//!

use test_case::test_case;

pub const SEND_TEST_SOURCE_04: &str = r#"
contract SendExample {
    function s() public returns (bool) {
        address r = address(0);
        return r.send(0);
    }
}
"#;

pub const SEND_TEST_SOURCE_05_07: &str = r#"
contract SendExample {
    function s() public returns (bool) {
        address payable r = address(0);
        return r.send(0);
    }
}
"#;

pub const SEND_TEST_SOURCE: &str = r#"
contract SendExample {
    function s() public payable returns (bool) {
        address r = address(0);
        return payable(r).send(msg.value);
    }
}
"#;

#[test_case(
    semver::Version::new(0, 4, 26),
    era_solc::StandardJsonInputCodegen::EVMLA,
    SEND_TEST_SOURCE_04
)]
#[test_case(
    semver::Version::new(0, 5, 17),
    era_solc::StandardJsonInputCodegen::EVMLA,
    SEND_TEST_SOURCE_05_07
)]
#[test_case(
    semver::Version::new(0, 6, 12),
    era_solc::StandardJsonInputCodegen::EVMLA,
    SEND_TEST_SOURCE_05_07
)]
#[test_case(
    semver::Version::new(0, 7, 6),
    era_solc::StandardJsonInputCodegen::EVMLA,
    SEND_TEST_SOURCE_05_07
)]
#[test_case(
    era_solc::Compiler::LAST_SUPPORTED_VERSION,
    era_solc::StandardJsonInputCodegen::EVMLA,
    SEND_TEST_SOURCE
)]
#[test_case(
    era_solc::Compiler::LAST_SUPPORTED_VERSION,
    era_solc::StandardJsonInputCodegen::Yul,
    SEND_TEST_SOURCE
)]
fn send(version: semver::Version, codegen: era_solc::StandardJsonInputCodegen, source_code: &str) {
    if cfg!(target_os = "windows") && version < semver::Version::new(0, 6, 0) {
        return;
    }

    assert!(crate::common::check_solidity_message(
        source_code,
        "You are using '<address payable>.send/transfer(<X>)' without providing",
        era_solc::StandardJsonInputLibraries::default(),
        &version,
        codegen,
        vec![],
        vec![],
    )
    .expect("Test failure"));
}

#[test_case(
    semver::Version::new(0, 4, 26),
    era_solc::StandardJsonInputCodegen::EVMLA,
    SEND_TEST_SOURCE_04
)]
#[test_case(
    semver::Version::new(0, 5, 17),
    era_solc::StandardJsonInputCodegen::EVMLA,
    SEND_TEST_SOURCE_05_07
)]
#[test_case(
    semver::Version::new(0, 6, 12),
    era_solc::StandardJsonInputCodegen::EVMLA,
    SEND_TEST_SOURCE_05_07
)]
#[test_case(
    semver::Version::new(0, 7, 6),
    era_solc::StandardJsonInputCodegen::EVMLA,
    SEND_TEST_SOURCE_05_07
)]
#[test_case(
    era_solc::Compiler::LAST_SUPPORTED_VERSION,
    era_solc::StandardJsonInputCodegen::EVMLA,
    SEND_TEST_SOURCE
)]
#[test_case(
    era_solc::Compiler::LAST_SUPPORTED_VERSION,
    era_solc::StandardJsonInputCodegen::Yul,
    SEND_TEST_SOURCE
)]
fn send_suppressed(
    version: semver::Version,
    codegen: era_solc::StandardJsonInputCodegen,
    source_code: &str,
) {
    if cfg!(target_os = "windows") && version < semver::Version::new(0, 6, 0) {
        return;
    }

    assert!(!crate::common::check_solidity_message(
        source_code,
        "You are using '<address payable>.send/transfer(<X>)' without providing",
        era_solc::StandardJsonInputLibraries::default(),
        &version,
        codegen,
        vec![era_solc::StandardJsonInputErrorType::SendTransfer],
        vec![],
    )
    .expect("Test failure"));
}

pub const TRANSFER_TEST_SOURCE_04: &str = r#"
contract TransferExample {
    function s() public {
        address r = address(0);
        r.transfer(0);
    }
}
"#;

pub const TRANSFER_TEST_SOURCE_05_07: &str = r#"
contract TransferExample {
    function s() public {
        address payable r = address(0);
        r.transfer(0);
    }
}
"#;

pub const TRANSFER_TEST_SOURCE: &str = r#"
contract TransferExample {
    function s() public payable {
        address r = address(0);
        payable(r).transfer(msg.value);
    }
}
"#;

#[test_case(
    semver::Version::new(0, 4, 26),
    era_solc::StandardJsonInputCodegen::EVMLA,
    TRANSFER_TEST_SOURCE_04
)]
#[test_case(
    semver::Version::new(0, 5, 17),
    era_solc::StandardJsonInputCodegen::EVMLA,
    TRANSFER_TEST_SOURCE_05_07
)]
#[test_case(
    semver::Version::new(0, 6, 12),
    era_solc::StandardJsonInputCodegen::EVMLA,
    TRANSFER_TEST_SOURCE_05_07
)]
#[test_case(
    semver::Version::new(0, 7, 6),
    era_solc::StandardJsonInputCodegen::EVMLA,
    TRANSFER_TEST_SOURCE_05_07
)]
#[test_case(
    era_solc::Compiler::LAST_SUPPORTED_VERSION,
    era_solc::StandardJsonInputCodegen::EVMLA,
    TRANSFER_TEST_SOURCE
)]
#[test_case(
    era_solc::Compiler::LAST_SUPPORTED_VERSION,
    era_solc::StandardJsonInputCodegen::Yul,
    TRANSFER_TEST_SOURCE
)]
fn transfer(
    version: semver::Version,
    codegen: era_solc::StandardJsonInputCodegen,
    source_code: &str,
) {
    if cfg!(target_os = "windows") && version < semver::Version::new(0, 6, 0) {
        return;
    }

    assert!(crate::common::check_solidity_message(
        source_code,
        "You are using '<address payable>.send/transfer(<X>)' without providing",
        era_solc::StandardJsonInputLibraries::default(),
        &version,
        codegen,
        vec![],
        vec![],
    )
    .expect("Test failure"));
}

#[test_case(
    semver::Version::new(0, 4, 26),
    era_solc::StandardJsonInputCodegen::EVMLA,
    TRANSFER_TEST_SOURCE_04
)]
#[test_case(
    semver::Version::new(0, 5, 17),
    era_solc::StandardJsonInputCodegen::EVMLA,
    TRANSFER_TEST_SOURCE_05_07
)]
#[test_case(
    semver::Version::new(0, 6, 12),
    era_solc::StandardJsonInputCodegen::EVMLA,
    TRANSFER_TEST_SOURCE_05_07
)]
#[test_case(
    semver::Version::new(0, 7, 6),
    era_solc::StandardJsonInputCodegen::EVMLA,
    TRANSFER_TEST_SOURCE_05_07
)]
#[test_case(
    era_solc::Compiler::LAST_SUPPORTED_VERSION,
    era_solc::StandardJsonInputCodegen::EVMLA,
    TRANSFER_TEST_SOURCE
)]
#[test_case(
    era_solc::Compiler::LAST_SUPPORTED_VERSION,
    era_solc::StandardJsonInputCodegen::Yul,
    TRANSFER_TEST_SOURCE
)]
fn transfer_suppressed(
    version: semver::Version,
    codegen: era_solc::StandardJsonInputCodegen,
    source_code: &str,
) {
    if cfg!(target_os = "windows") && version < semver::Version::new(0, 6, 0) {
        return;
    }

    assert!(!crate::common::check_solidity_message(
        source_code,
        "You are using '<address payable>.send/transfer(<X>)' without providing",
        era_solc::StandardJsonInputLibraries::default(),
        &version,
        codegen,
        vec![era_solc::StandardJsonInputErrorType::SendTransfer],
        vec![],
    )
    .expect("Test failure"));
}

pub const RUNTIME_CODE_SOURCE_CODE: &str = r#"
// SPDX-License-Identifier: Unlicensed

pragma solidity >=0.5.3;

contract A {}

contract Test {
    function main() public pure returns(bytes memory) {
        return type(A).runtimeCode;
    }
}
"#;

#[test_case(
    semver::Version::new(0, 5, 17),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    semver::Version::new(0, 6, 12),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    semver::Version::new(0, 7, 6),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    era_solc::Compiler::LAST_SUPPORTED_VERSION,
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    era_solc::Compiler::LAST_SUPPORTED_VERSION,
    era_solc::StandardJsonInputCodegen::Yul
)]
fn runtime_code(version: semver::Version, codegen: era_solc::StandardJsonInputCodegen) {
    if cfg!(target_os = "windows") && version < semver::Version::new(0, 6, 0) {
        return;
    }

    assert!(crate::common::check_solidity_message(
        RUNTIME_CODE_SOURCE_CODE,
        "Deploy and runtime code are merged together on ZKsync",
        era_solc::StandardJsonInputLibraries::default(),
        &version,
        codegen,
        vec![],
        vec![],
    )
    .expect("Test failure"));
}

pub const TX_ORIGIN_TEST_SOURCE: &str = r#"
contract TxOriginExample {
    function main() private {
        address txOrigin = tx.origin;
    }
}
"#;

#[test_case(
    semver::Version::new(0, 4, 26),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    semver::Version::new(0, 5, 17),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    semver::Version::new(0, 6, 12),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    semver::Version::new(0, 7, 6),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    era_solc::Compiler::LAST_SUPPORTED_VERSION,
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    era_solc::Compiler::LAST_SUPPORTED_VERSION,
    era_solc::StandardJsonInputCodegen::Yul
)]
fn tx_origin(version: semver::Version, codegen: era_solc::StandardJsonInputCodegen) {
    if cfg!(target_os = "windows") && version < semver::Version::new(0, 6, 0) {
        return;
    }

    assert!(crate::common::check_solidity_message(
        TX_ORIGIN_TEST_SOURCE,
        "You are checking for 'tx.origin', which might lead to",
        era_solc::StandardJsonInputLibraries::default(),
        &version,
        codegen,
        vec![],
        vec![],
    )
    .expect("Test failure"));
}

#[test_case(
    semver::Version::new(0, 4, 26),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    semver::Version::new(0, 5, 17),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    semver::Version::new(0, 6, 12),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    semver::Version::new(0, 7, 6),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    era_solc::Compiler::LAST_SUPPORTED_VERSION,
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    era_solc::Compiler::LAST_SUPPORTED_VERSION,
    era_solc::StandardJsonInputCodegen::Yul
)]
fn tx_origin_suppressed(version: semver::Version, codegen: era_solc::StandardJsonInputCodegen) {
    if cfg!(target_os = "windows") && version < semver::Version::new(0, 6, 0) {
        return;
    }

    assert!(!crate::common::check_solidity_message(
        TX_ORIGIN_TEST_SOURCE,
        "You are checking for 'tx.origin', which might lead to",
        era_solc::StandardJsonInputLibraries::default(),
        &version,
        codegen,
        vec![],
        vec![era_solc::StandardJsonInputWarningType::TxOrigin],
    )
    .expect("Test failure"));
}

pub const TX_ORIGIN_ASSEMBLY_TEST_SOURCE: &str = r#"
contract TxOriginExample {
    function main() private {
        assembly {
            let txOrigin := origin()
        }
    }
}
"#;

#[test_case(
    semver::Version::new(0, 4, 26),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    semver::Version::new(0, 5, 17),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    semver::Version::new(0, 6, 12),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    semver::Version::new(0, 7, 6),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    era_solc::Compiler::LAST_SUPPORTED_VERSION,
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    era_solc::Compiler::LAST_SUPPORTED_VERSION,
    era_solc::StandardJsonInputCodegen::Yul
)]
fn tx_origin_assembly(version: semver::Version, codegen: era_solc::StandardJsonInputCodegen) {
    if cfg!(target_os = "windows") && version < semver::Version::new(0, 6, 0) {
        return;
    }

    assert!(crate::common::check_solidity_message(
        TX_ORIGIN_ASSEMBLY_TEST_SOURCE,
        "You are checking for 'tx.origin', which might lead to",
        era_solc::StandardJsonInputLibraries::default(),
        &version,
        codegen,
        vec![],
        vec![],
    )
    .expect("Test failure"));
}

#[test_case(
    semver::Version::new(0, 4, 26),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    semver::Version::new(0, 5, 17),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    semver::Version::new(0, 6, 12),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    semver::Version::new(0, 7, 6),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    era_solc::Compiler::LAST_SUPPORTED_VERSION,
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    era_solc::Compiler::LAST_SUPPORTED_VERSION,
    era_solc::StandardJsonInputCodegen::Yul
)]
fn tx_origin_assembly_suppressed(
    version: semver::Version,
    codegen: era_solc::StandardJsonInputCodegen,
) {
    if cfg!(target_os = "windows") && version < semver::Version::new(0, 6, 0) {
        return;
    }

    assert!(!crate::common::check_solidity_message(
        TX_ORIGIN_ASSEMBLY_TEST_SOURCE,
        "You are checking for 'tx.origin' in your code, which might lead to",
        era_solc::StandardJsonInputLibraries::default(),
        &version,
        codegen,
        vec![],
        vec![era_solc::StandardJsonInputWarningType::TxOrigin],
    )
    .expect("Test failure"));
}

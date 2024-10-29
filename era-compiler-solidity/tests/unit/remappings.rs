//!
//! The Solidity compiler unit tests for remappings.
//!

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use era_compiler_solidity::solc::standard_json::input::settings::codegen::Codegen as SolcStandardJsonInputSettingsCodegen;
use era_compiler_solidity::solc::standard_json::input::settings::libraries::Libraries;
use era_compiler_solidity::solc::Compiler as SolcCompiler;

use crate::common;

#[test]
#[cfg_attr(target_os = "windows", ignore)]
fn default_04_evmla() {
    default(
        semver::Version::new(0, 4, 26),
        SolcStandardJsonInputSettingsCodegen::EVMLA,
    );
}
#[test]
#[cfg_attr(target_os = "windows", ignore)]
fn default_05_evmla() {
    default(
        semver::Version::new(0, 5, 17),
        SolcStandardJsonInputSettingsCodegen::EVMLA,
    );
}
#[test]
fn default_06_evmla() {
    default(
        semver::Version::new(0, 6, 12),
        SolcStandardJsonInputSettingsCodegen::EVMLA,
    );
}
#[test]
fn default_07_evmla() {
    default(
        semver::Version::new(0, 7, 6),
        SolcStandardJsonInputSettingsCodegen::EVMLA,
    );
}
#[test]
fn default_08_evmla() {
    default(
        SolcCompiler::LAST_SUPPORTED_VERSION,
        SolcStandardJsonInputSettingsCodegen::EVMLA,
    );
}
#[test]
fn default_08_yul() {
    default(
        SolcCompiler::LAST_SUPPORTED_VERSION,
        SolcStandardJsonInputSettingsCodegen::Yul,
    );
}

pub const CALLEE_TEST_SOURCE: &str = r#"
// SPDX-License-Identifier: MIT

pragma solidity >=0.4.12;

contract Callable {
    function f(uint a) public pure returns(uint) {
        return a * 2;
    }
}
"#;

pub const CALLER_TEST_SOURCE: &str = r#"
// SPDX-License-Identifier: MIT

pragma solidity >=0.4.12;

import "libraries/default/callable.sol";

contract Main {
    function main(Callable callable) public pure returns(uint) {
        return callable.f(5);
    }
}
"#;

fn default(version: semver::Version, codegen: SolcStandardJsonInputSettingsCodegen) {
    let mut sources = BTreeMap::new();
    sources.insert("./test.sol".to_owned(), CALLER_TEST_SOURCE.to_owned());
    sources.insert("./callable.sol".to_owned(), CALLEE_TEST_SOURCE.to_owned());

    let mut remappings = BTreeSet::new();
    remappings.insert("libraries/default/=./".to_owned());

    common::build_solidity(
        sources,
        Libraries::default(),
        era_compiler_common::HashType::Keccak256,
        remappings,
        &version,
        codegen,
        era_compiler_llvm_context::OptimizerSettings::cycles(),
    )
    .expect("Test failure");
}

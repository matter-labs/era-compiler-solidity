//!
//! The Solidity compiler unit tests for remappings.
//!

#![cfg(test)]

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use crate::solc::pipeline::Pipeline as SolcPipeline;
use crate::solc::Compiler as SolcCompiler;

#[test]
fn default_04_evmla() {
    default(semver::Version::new(0, 4, 26), SolcPipeline::EVMLA);
}
#[test]
fn default_05_evmla() {
    default(semver::Version::new(0, 5, 17), SolcPipeline::EVMLA);
}
#[test]
fn default_06_evmla() {
    default(semver::Version::new(0, 6, 12), SolcPipeline::EVMLA);
}
#[test]
fn default_07_evmla() {
    default(semver::Version::new(0, 7, 6), SolcPipeline::EVMLA);
}
#[test]
fn default_08_evmla() {
    default(SolcCompiler::LAST_SUPPORTED_VERSION, SolcPipeline::EVMLA);
}
#[test]
fn default_08_yul() {
    default(SolcCompiler::LAST_SUPPORTED_VERSION, SolcPipeline::Yul);
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

fn default(version: semver::Version, pipeline: SolcPipeline) {
    let mut sources = BTreeMap::new();
    sources.insert("./test.sol".to_owned(), CALLER_TEST_SOURCE.to_owned());
    sources.insert("./callable.sol".to_owned(), CALLEE_TEST_SOURCE.to_owned());

    let mut remappings = BTreeSet::new();
    remappings.insert("libraries/default/=./".to_owned());

    super::build_solidity(
        sources.clone(),
        BTreeMap::new(),
        Some(remappings.clone()),
        &version,
        pipeline,
        era_compiler_llvm_context::OptimizerSettings::cycles(),
    )
    .expect("Test failure");
}

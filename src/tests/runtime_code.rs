//!
//! The Solidity compiler unit tests for runtime code.
//!

#![cfg(test)]

use std::collections::BTreeMap;

use crate::solc::pipeline::Pipeline as SolcPipeline;
use crate::solc::Compiler as SolcCompiler;

#[test]
#[should_panic(expected = "runtimeCode is not supported")]
fn default_04_evmla() {
    default(semver::Version::new(0, 4, 26), SolcPipeline::EVMLA);
}
#[test]
#[should_panic(expected = "runtimeCode is not supported")]
fn default_05_evmla() {
    default(semver::Version::new(0, 5, 17), SolcPipeline::EVMLA);
}
#[test]
#[should_panic(expected = "runtimeCode is not supported")]
fn default_06_evmla() {
    default(semver::Version::new(0, 6, 12), SolcPipeline::EVMLA);
}
#[test]
#[should_panic(expected = "runtimeCode is not supported")]
fn default_07_evmla() {
    default(semver::Version::new(0, 7, 6), SolcPipeline::EVMLA);
}
#[test]
#[should_panic(expected = "runtimeCode is not supported")]
fn default_08_evmla() {
    default(SolcCompiler::LAST_SUPPORTED_VERSION, SolcPipeline::EVMLA);
}
#[test]
#[should_panic(expected = "runtimeCode is not supported")]
fn default_08_yul() {
    default(SolcCompiler::LAST_SUPPORTED_VERSION, SolcPipeline::Yul);
}

fn default(version: semver::Version, pipeline: SolcPipeline) {
    let source_code = r#"
// SPDX-License-Identifier: MIT
pragma solidity >=0.4.12;

contract A {}

contract Test {
    function main() public pure returns(bytes memory) {
        return type(A).runtimeCode;
    }
}
    "#;

    let mut sources = BTreeMap::new();
    sources.insert("test.sol".to_owned(), source_code.to_owned());

    super::build_solidity(
        sources.clone(),
        BTreeMap::new(),
        None,
        &version,
        pipeline,
        era_compiler_llvm_context::OptimizerSettings::cycles(),
    )
    .expect("Test failure");
}

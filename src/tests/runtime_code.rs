//!
//! The Solidity compiler unit tests for runtime code.
//!

#![cfg(test)]

use std::collections::BTreeMap;

use crate::solc::pipeline::Pipeline as SolcPipeline;

#[test]
fn default() {
    let source_code = r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract A {}

contract Test {
    function main() public pure returns(bytes memory) {
        return type(A).runtimeCode;
    }
}
    "#;

    assert!(
        super::build_solidity(source_code, BTreeMap::new(), SolcPipeline::Yul)
            .err()
            .unwrap()
            .to_string()
            .contains("runtimeCode is not supported")
    );
}

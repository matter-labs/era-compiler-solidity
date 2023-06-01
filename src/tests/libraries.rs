//!
//! The Solidity compiler unit tests for libraries.
//!

#![cfg(test)]

use std::collections::BTreeMap;

use crate::solc::pipeline::Pipeline as SolcPipeline;

pub const LIBRARY_TEST_SOURCE: &str = r#"
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

// A simple library with at least one external method
library SimpleLibrary {
    function add(uint256 a, uint256 b) external pure returns (uint256) {
        return a + b;
    }
}

// A contract calling that library
contract SimpleContract {
    using SimpleLibrary for uint256;

    function addTwoNumbers(uint256 a, uint256 b) public pure returns (uint256) {
        return a.add(b);
    }
}
    "#;

#[test]
#[should_panic(expected = "Library `test.sol:SimpleLibrary` not found in the project")]
fn not_specified() {
    super::build_solidity(LIBRARY_TEST_SOURCE, BTreeMap::new(), SolcPipeline::Yul)
        .expect("Test failure");
}

#[test]
fn specified() {
    let mut libraries = BTreeMap::new();
    libraries
        .entry("test.sol".to_string())
        .or_insert_with(BTreeMap::new)
        .entry("SimpleLibrary".to_string())
        .or_insert("0x00000000000000000000000000000000DEADBEEF".to_string());

    super::build_solidity(LIBRARY_TEST_SOURCE, libraries, SolcPipeline::Yul).expect("Test failure");
}

//!
//! CLI tests for the constant folding in the EVMLA abstract interpreter.
//!

use test_case::test_case;

///
/// Compiles a contract with `codecopy(<expression>, off, 32)` in inline assembly, where `off` is a
/// runtime value, and returns the destination of the copy as seen by the abstract interpreter.
///
/// The result is a constant in uppercase hexadecimal, or `runtime` if the interpreter doesn't know
/// the value. It is read from the `.ethir` dump, as a folded value doesn't always affect the
/// generated code.
///
fn folded(assembly: &str) -> anyhow::Result<String> {
    crate::common::setup()?;

    let tmp_dir = tempfile::TempDir::with_prefix("evmla_constant_folding")?;
    let path = tmp_dir.path().join("Test.sol");
    std::fs::write(
        &path,
        format!(
            r#"// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
contract Test {{
    function main(uint256 off) external view returns (bytes32 r) {{
        assembly {{
            {assembly}
            r := mload(0)
        }}
    }}
}}
"#
        ),
    )?;

    let tmp_dir_debug = tempfile::TempDir::with_prefix("evmla_constant_folding_debug")?;
    let args = &[
        "--codegen",
        "evmla",
        "--asm",
        path.to_str().expect("Always valid"),
        "--debug-output-dir",
        tmp_dir_debug.path().to_str().expect("Always valid"),
    ];
    crate::cli::execute_zksolc(args)?.success();

    let mut destinations = Vec::new();
    for entry in std::fs::read_dir(tmp_dir_debug.path())? {
        let path = entry?.path();
        if path
            .extension()
            .is_some_and(|extension| extension == "ethir")
        {
            destinations.extend(
                std::fs::read_to_string(path)?
                    .lines()
                    .filter(|line| line.trim_start().starts_with("CODECOPY"))
                    // Skip the copy of the runtime code in the deploy code.
                    .filter(|line| !line.contains(".runtime"))
                    .filter_map(|line| {
                        line.rsplit_once(" | ")
                            .and_then(|(_, top)| top.strip_suffix(" ]"))
                            .map(|top| {
                                if top.starts_with("V_") {
                                    "runtime".to_owned()
                                } else {
                                    top.to_owned()
                                }
                            })
                    }),
            );
        }
    }
    assert_eq!(destinations.len(), 1, "expected a single user `codecopy`");

    Ok(destinations.remove(0))
}

#[test_case("12", "C"; "control")]
#[test_case("shr(255, shl(255, 1))", "1"; "in range")]
#[test_case("add(shl(256, 1), 10)", "A"; "shl by word size")]
#[test_case("add(shl(0x10000000000000000, 1), 10)", "A"; "shl by more than u64")]
#[test_case("shr(256, 11)", "0"; "shr by word size")]
#[test_case("shr(0x10000000000000000, 11)", "0"; "shr by more than u64")]
fn shift_amount(destination: &str, expected: &str) -> anyhow::Result<()> {
    let assembly = format!("codecopy({destination}, off, 32)");
    assert_eq!(folded(assembly.as_str())?, expected);

    Ok(())
}

// The top bit of each result is 1, and it would be 3 if the overflow bit were kept.
#[test_case("shr(255, add(0x8000000000000000000000000000000000000000000000000000000000000000, 1))", "1"; "add in range")]
#[test_case("shr(255, mul(0x4000000000000000000000000000000000000000000000000000000000000000, 2))", "1"; "mul in range")]
#[test_case("shr(255, shl(254, 2))", "1"; "shl in range")]
#[test_case("shr(255, add(0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff, 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff))", "1"; "add overflow")]
#[test_case("shr(255, mul(0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff, 2))", "1"; "mul overflow")]
#[test_case("shr(255, shl(1, 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff))", "1"; "shl overflow")]
fn overflow(destination: &str, expected: &str) -> anyhow::Result<()> {
    let assembly = format!("codecopy({destination}, off, 32)");
    assert_eq!(folded(assembly.as_str())?, expected);

    Ok(())
}

// `d` is 1 or 2 depending on the route to `codecopy`, so it's only known at runtime.
#[test_case(
    r#"
    let d := 1
    if calldataload(36) { d := 2 }
    codecopy(d, off, 32)
    "#,
    "1";
    "forward join"
)]
#[test_case(
    r#"
    let d := 1
    for { let i := 0 } lt(i, calldataload(36)) { i := add(i, 1) } { d := 2 }
    codecopy(d, off, 32)
    "#,
    "1";
    "back edge"
)]
fn route_dependent(assembly: &str, expected: &str) -> anyhow::Result<()> {
    assert_eq!(folded(assembly)?, expected);

    Ok(())
}

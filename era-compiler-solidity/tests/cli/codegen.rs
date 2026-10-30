//!
//! CLI tests for the eponymous option.
//!

use era_solc::StandardJsonInputCodegen;
use predicates::prelude::*;
use test_case::test_case;

#[test_case(StandardJsonInputCodegen::EVMLA)]
#[test_case(StandardJsonInputCodegen::Yul)]
fn default(codegen: StandardJsonInputCodegen) -> anyhow::Result<()> {
    crate::common::setup()?;

    let codegen = codegen.to_string();
    let args = &[
        "--codegen",
        codegen.as_str(),
        "--bin",
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result
        .success()
        .stdout(predicate::str::contains("Binary:\n"));

    Ok(())
}

#[test_case(StandardJsonInputCodegen::EVMLA)]
#[test_case(StandardJsonInputCodegen::Yul)]
fn yul(codegen: era_solc::StandardJsonInputCodegen) -> anyhow::Result<()> {
    crate::common::setup()?;

    let codegen = codegen.to_string();
    let args = &[
        "--codegen",
        codegen.as_str(),
        "--yul",
        "--bin",
        crate::common::TEST_YUL_CONTRACT_PATH,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "Error: Codegen settings are only available in Solidity mode.",
    ));

    Ok(())
}

#[test_case(StandardJsonInputCodegen::EVMLA)]
#[test_case(StandardJsonInputCodegen::Yul)]
fn llvm_ir(codegen: era_solc::StandardJsonInputCodegen) -> anyhow::Result<()> {
    crate::common::setup()?;

    let codegen = codegen.to_string();
    let args = &[
        "--codegen",
        codegen.as_str(),
        "--llvm-ir",
        "--bin",
        crate::common::TEST_LLVM_IR_CONTRACT_ERAVM_PATH,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "Error: Codegen settings are only available in Solidity mode.",
    ));

    Ok(())
}

#[test_case(StandardJsonInputCodegen::EVMLA)]
#[test_case(StandardJsonInputCodegen::Yul)]
fn eravm_assembly(codegen: era_solc::StandardJsonInputCodegen) -> anyhow::Result<()> {
    crate::common::setup()?;

    let codegen = codegen.to_string();
    let args = &[
        "--codegen",
        codegen.as_str(),
        "--eravm-assembly",
        "--bin",
        crate::common::TEST_ERAVM_ASSEMBLY_CONTRACT_PATH,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result.failure().stderr(predicate::str::contains(
        "Error: Codegen settings are only available in Solidity mode.",
    ));

    Ok(())
}

#[test_case(StandardJsonInputCodegen::EVMLA)]
#[test_case(StandardJsonInputCodegen::Yul)]
fn standard_json(codegen: era_solc::StandardJsonInputCodegen) -> anyhow::Result<()> {
    crate::common::setup()?;

    let codegen = codegen.to_string();
    let args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
        "--codegen",
        codegen.as_str(),
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "Codegen must be passed via standard JSON input.",
    ));

    Ok(())
}

#[test]
fn invalid() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--codegen",
        "invalid",
        "--bin",
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result
        .failure()
        .stderr(predicate::str::contains("error: invalid value 'invalid' for '--codegen <CODEGEN>': Invalid codegen: `invalid`. Available options: evmla, yul"));

    Ok(())
}

#[test]
fn missing() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_ZKSOLC_FORCE_EVMLA,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "The default codegen of `zksolc` does not match that of `solc` for historical reasons.",
    ));

    Ok(())
}

#[test]
fn evmla_codecopy_to_library_marker_offset() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--codegen",
        "evmla",
        "--asm",
        crate::common::TEST_SOLIDITY_CONTRACT_CODECOPY_ELEVEN_PATH,
    ];

    // Offset 11 is where solc's library deploy stub writes the `PUSH20` opcode,
    // but this copy is hand-written and must be emitted as a real copy. The
    // marker lowers to an `or` against `0x73 << 248`, so its absence, together
    // with the pointer arithmetic of an actual copy, pins the behaviour.
    let result = crate::cli::execute_zksolc(args)?;
    result
        .success()
        .stdout(
            predicate::str::contains(
                "52015977587075634662932278421871521105961125924018222127100086566054726205440",
            )
            .not(),
        )
        .stdout(predicate::str::contains("addp"));

    Ok(())
}

#[test]
fn evmla_library_deploy_stub_keeps_marker() -> anyhow::Result<()> {
    crate::common::setup()?;

    let tmp_dir_debug = tempfile::TempDir::with_prefix("library_marker")?;
    let args = &[
        "--codegen",
        "evmla",
        "--bin",
        crate::common::TEST_SOLIDITY_CONTRACT_LIBRARY_PATH,
        "--debug-output-dir",
        tmp_dir_debug.path().to_str().unwrap(),
    ];
    crate::cli::execute_zksolc(args)?.success();

    // The counterpart of the test above, asserted on the IR rather than the
    // assembly, where the marker is unambiguous and the text is stable.
    //
    // The library must still get `mstore8(11, 0x73)`, or solc's deploy-time
    // assertion that byte 11 is PUSH20 fails and the library reverts on
    // deployment. The contract in the same file copies its own runtime
    // sub-assembly to offset 0, so it must not get the marker: that is the
    // `Path` source with a destination other than 11.
    const MARKER: &str = "@__mstore8(ptr addrspace(1)";
    const MARKER_ARGS: &str = "inttoptr (i256 11 to ptr addrspace(1)), i256 115)";

    let mut library_markers = 0;
    let mut contract_markers = 0;
    for entry in std::fs::read_dir(tmp_dir_debug.path())? {
        let path = entry?.path();
        let name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        if !name.ends_with(".unoptimized.ll") {
            continue;
        }
        let ir = std::fs::read_to_string(&path)?;
        let count = ir
            .lines()
            .filter(|line| line.contains(MARKER) && line.contains(MARKER_ARGS))
            .count();
        if name.contains("SimpleLibrary") {
            library_markers += count;
        } else if name.contains("SimpleContract") {
            contract_markers += count;
        }
    }

    assert_eq!(library_markers, 1, "the library lost its deploy marker");
    assert_eq!(contract_markers, 0, "a non-library got the deploy marker");

    Ok(())
}

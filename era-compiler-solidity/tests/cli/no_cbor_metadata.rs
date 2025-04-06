//!
//! CLI tests for the eponymous option.
//!

use era_compiler_common::EVMMetadataHashType;
use era_compiler_common::EraVMMetadataHashType;
use era_compiler_common::Target;
use predicates::prelude::*;
use test_case::test_case;

#[test_case(Target::EraVM, EraVMMetadataHashType::None.to_string())]
// #[test_case(Target::EVM, EVMMetadataHashType::None.to_string())] TODO: move metadata to linker
fn none(target: Target, hash_type: String) -> anyhow::Result<()> {
    let _ = crate::common::setup();

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--metadata-hash",
        hash_type.as_str(),
        "--no-cbor-metadata",
        "--bin",
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result
        .success()
        .stdout(predicate::str::contains("Binary"))
        .stdout(predicate::str::contains("a165").not())
        .stdout(predicate::str::ends_with("0023").not());

    Ok(())
}

#[test_case(Target::EraVM, EraVMMetadataHashType::IPFS.to_string())]
// #[test_case(Target::EVM, EVMMetadataHashType::IPFS.to_string())] TODO: move metadata to linker
fn ipfs_solidity(target: Target, hash_type: String) -> anyhow::Result<()> {
    let _ = crate::common::setup();

    let args = &[
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
        "--metadata-hash",
        hash_type.as_str(),
        "--no-cbor-metadata",
        "--bin",
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result
        .success()
        .stdout(predicate::str::contains("Binary"))
        .stdout(predicate::str::contains("a264").not())
        .stdout(predicate::str::ends_with("0055").not());

    Ok(())
}

#[test_case(Target::EraVM, EraVMMetadataHashType::IPFS.to_string())]
// #[test_case(Target::EVM, EVMMetadataHashType::IPFS.to_string())] TODO: move metadata to linker
fn ipfs_yul(target: Target, hash_type: String) -> anyhow::Result<()> {
    let _ = crate::common::setup();

    let args = &[
        "--yul",
        crate::common::TEST_YUL_CONTRACT_PATH,
        "--metadata-hash",
        hash_type.as_str(),
        "--no-cbor-metadata",
        "--bin",
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result
        .success()
        .stdout(predicate::str::contains("Binary"))
        .stdout(predicate::str::contains("a264").not())
        .stdout(predicate::str::ends_with("003e").not());

    Ok(())
}

#[test_case(Target::EraVM, EraVMMetadataHashType::IPFS.to_string())]
// #[test_case(Target::EVM, EVMMetadataHashType::IPFS.to_string())] TODO: move metadata to linker
fn ipfs_llvm_ir(target: Target, hash_type: String) -> anyhow::Result<()> {
    let _ = crate::common::setup();

    let args = &[
        "--llvm-ir",
        crate::common::TEST_LLVM_IR_CONTRACT_PATH,
        "--metadata-hash",
        hash_type.as_str(),
        "--no-cbor-metadata",
        "--bin",
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result
        .success()
        .stdout(predicate::str::contains("Binary"))
        .stdout(predicate::str::contains("a264").not())
        .stdout(predicate::str::ends_with("003e").not());

    Ok(())
}

#[test_case(Target::EraVM, EraVMMetadataHashType::IPFS.to_string())]
// #[test_case(Target::EVM, EVMMetadataHashType::IPFS.to_string())] TODO: move metadata to linker
fn ipfs_eravm_assembly(target: Target, hash_type: String) -> anyhow::Result<()> {
    let _ = crate::common::setup();

    let args = &[
        "--eravm-assembly",
        crate::common::TEST_ERAVM_ASSEMBLY_CONTRACT_PATH,
        "--metadata-hash",
        hash_type.as_str(),
        "--no-cbor-metadata",
        "--bin",
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result
        .success()
        .stdout(predicate::str::contains("Binary"))
        .stdout(predicate::str::contains("a264").not())
        .stdout(predicate::str::ends_with("003e").not());

    Ok(())
}

#[test_case(Target::EraVM)]
// #[test_case(Target::EVM, EVMMetadataHashType::IPFS.to_string())] TODO: move metadata to linker
fn standard_json(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &["--standard-json", crate::common::TEST_JSON_NO_CBOR_METADATA];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result
        .success()
        .stdout(predicate::str::contains("a264").not())
        .stdout(predicate::str::ends_with("0055").not());

    Ok(())
}

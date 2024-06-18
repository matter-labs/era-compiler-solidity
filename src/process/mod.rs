//!
//! Process for compiling a single compilation unit.
//!

pub mod input_eravm;
pub mod input_evm;
pub mod output_eravm;
pub mod output_evm;

use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::sync::OnceLock;

use crate::solc::standard_json::output::error::source_location::SourceLocation as SolcStandardJsonOutputSourceLocation;
use crate::solc::standard_json::output::error::Error as SolcStandardJsonOutputError;

use self::input_eravm::Input as EraVMInput;
use self::input_evm::Input as EVMInput;
use self::output_eravm::Output as EraVMOutput;
use self::output_evm::Output as EVMOutput;

/// The overriden executable name used when the compiler is run as a library.
pub static EXECUTABLE: OnceLock<PathBuf> = OnceLock::new();

///
/// Read input from `stdin`, compile a contract, and write the output to `stdout`.
///
pub fn run(target: era_compiler_llvm_context::Target) {
    match target {
        era_compiler_llvm_context::Target::EraVM => {
            let input_json =
                std::io::read_to_string(std::io::stdin()).expect("Stdin reading error");
            let input: EraVMInput = era_compiler_common::deserialize_from_str(input_json.as_str())
                .expect("Stdin reading error");
            if input.enable_test_encoding {
                zkevm_assembly::set_encoding_mode(zkevm_assembly::RunningVmEncodingMode::Testing);
            }

            let contract = input.contract.expect("Always exists");
            let source_location = SolcStandardJsonOutputSourceLocation::new(contract.path.clone());
            let result: crate::Result<EraVMOutput> = contract
                .compile_to_eravm(
                    input.dependency_data,
                    input.enable_eravm_extensions,
                    input.include_metadata_hash,
                    input.optimizer_settings,
                    input.llvm_options,
                    input.output_assembly,
                    input.debug_config,
                )
                .map(EraVMOutput::new)
                .map_err(|error| {
                    SolcStandardJsonOutputError::new_error(error, Some(source_location))
                });
            serde_json::to_writer(std::io::stdout(), &result).expect("Stdout writing error");
        }
        era_compiler_llvm_context::Target::EVM => {
            let input_json =
                std::io::read_to_string(std::io::stdin()).expect("Stdin reading error");
            let input: EVMInput = era_compiler_common::deserialize_from_str(input_json.as_str())
                .expect("Stdin reading error");

            let contract = input.contract.expect("Always exists");
            let source_location = SolcStandardJsonOutputSourceLocation::new(contract.path.clone());
            let result: crate::Result<EVMOutput> = contract
                .compile_to_evm(
                    input.dependency_data,
                    input.include_metadata_hash,
                    input.optimizer_settings,
                    input.llvm_options,
                    input.debug_config,
                )
                .map(EVMOutput::new)
                .map_err(|error| {
                    SolcStandardJsonOutputError::new_error(error, Some(source_location))
                });
            serde_json::to_writer(std::io::stdout(), &result).expect("Stdout writing error");
        }
    }
}

///
/// Runs this process recursively to compile a single contract.
///
pub fn call<I, O>(input: I, target: era_compiler_llvm_context::Target) -> crate::Result<O>
where
    I: serde::Serialize,
    O: serde::de::DeserializeOwned,
{
    let executable = EXECUTABLE
        .get()
        .unwrap_or_else(|| panic!("Current executable path getting error"));

    let mut command = Command::new(executable.as_path());
    command.stdin(std::process::Stdio::piped());
    command.stdout(std::process::Stdio::piped());
    command.arg("--recursive-process");
    command.arg("--target");
    command.arg(target.to_string());

    let mut process = command
        .spawn()
        .unwrap_or_else(|error| panic!("{executable:?} subprocess spawning error: {error:?}"));

    let stdin = process
        .stdin
        .as_mut()
        .unwrap_or_else(|| panic!("{executable:?} subprocess stdin getting error"));
    let stdin_input = serde_json::to_vec(&input).expect("Always valid");
    stdin
        .write_all(stdin_input.as_slice())
        .unwrap_or_else(
            |error| panic!("{executable:?} subprocess stdin writing error: {error:?}",),
        );

    let result = process.wait_with_output().unwrap_or_else(|error| {
        panic!("{executable:?} subprocess output reading error: {error:?}")
    });
    era_compiler_common::deserialize_from_slice(result.stdout.as_slice())
        .unwrap_or_else(|error| panic!("{executable:?} subprocess stdout parsing error: {error:?}"))
}

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

use self::input_eravm::Input as EraVMInput;
use self::input_evm::Input as EVMInput;
use self::output_eravm::Output as EraVMOutput;
use self::output_evm::Output as EVMOutput;

/// The overridden executable name used when the compiler is run as a library.
pub static EXECUTABLE: OnceLock<PathBuf> = OnceLock::new();

///
/// Read input from `stdin`, compile a contract, and write the output to `stdout`.
///
pub fn run(target: era_compiler_common::Target) -> anyhow::Result<()> {
    match target {
        era_compiler_common::Target::EraVM => {
            let input_json = std::io::read_to_string(std::io::stdin())
                .map_err(|error| anyhow::anyhow!("Stdin reading error: {error}"))?;
            let input: EraVMInput = era_compiler_common::deserialize_from_str(input_json.as_str())
                .map_err(|error| anyhow::anyhow!("Stdin parsing error: {error}"))?;

            let source_location = era_solc::StandardJsonOutputErrorSourceLocation::new(
                input.contract.name.path.clone(),
            );
            let result = input
                .contract
                .compile_to_eravm(
                    input.solc_version,
                    input.identifier_paths,
                    input.missing_libraries,
                    input.factory_dependencies,
                    input.enable_eravm_extensions,
                    input.metadata_hash_type,
                    input.optimizer_settings,
                    input.llvm_options,
                    input.output_assembly,
                    input.debug_config,
                )
                .map(EraVMOutput::new)
                .map_err(|error| {
                    era_solc::StandardJsonOutputError::new_error(error, Some(source_location), None)
                });
            serde_json::to_writer(std::io::stdout(), &result)
                .map_err(|error| anyhow::anyhow!("Stdout writing error: {error}"))?;
        }
        era_compiler_common::Target::EVM => {
            let input_json = std::io::read_to_string(std::io::stdin())
                .map_err(|error| anyhow::anyhow!("Stdin reading error: {error}"))?;
            let input: EVMInput = era_compiler_common::deserialize_from_str(input_json.as_str())
                .map_err(|error| anyhow::anyhow!("Stdin parsing error: {error}"))?;

            let source_location = era_solc::StandardJsonOutputErrorSourceLocation::new(
                input.contract.name.path.clone(),
            );
            let result = input
                .contract
                .compile_to_evm(
                    input.solc_version,
                    input.identifier_paths,
                    input.missing_libraries,
                    input.metadata_hash_type,
                    input.optimizer_settings,
                    input.llvm_options,
                    input.debug_config,
                )
                .map(EVMOutput::new)
                .map_err(|error| {
                    era_solc::StandardJsonOutputError::new_error(error, Some(source_location), None)
                });
            serde_json::to_writer(std::io::stdout(), &result)
                .map_err(|error| anyhow::anyhow!("Stdout writing error: {error}"))?;
        }
    }
    unsafe { inkwell::support::shutdown_llvm() };
    Ok(())
}

///
/// Runs this process recursively to compile a single contract.
///
pub fn call<I, O>(path: &str, input: I, target: era_compiler_common::Target) -> crate::Result<O>
where
    I: serde::Serialize,
    O: serde::de::DeserializeOwned,
{
    let executable = EXECUTABLE
        .get()
        .cloned()
        .unwrap_or_else(|| std::env::current_exe().expect("Current executable path getting error"));

    let mut command = Command::new(executable.as_path());
    command.stdin(std::process::Stdio::piped());
    command.stdout(std::process::Stdio::piped());
    command.stderr(std::process::Stdio::piped());
    command.arg("--recursive-process");
    command.arg("--target");
    command.arg(target.to_string());

    let mut process = command
        .spawn()
        .unwrap_or_else(|error| panic!("{executable:?} subprocess spawning: {error:?}"));

    let stdin = process
        .stdin
        .as_mut()
        .unwrap_or_else(|| panic!("{executable:?} subprocess stdin getting error"));
    let stdin_input = serde_json::to_vec(&input).expect("Always valid");
    stdin
        .write_all(stdin_input.as_slice())
        .unwrap_or_else(|error| panic!("{executable:?} subprocess stdin writing: {error:?}"));

    let result = process
        .wait_with_output()
        .unwrap_or_else(|error| panic!("{executable:?} subprocess output reading: {error:?}"));

    if result.status.code() != Some(era_compiler_common::EXIT_CODE_SUCCESS) {
        let message = format!(
            "{executable:?} subprocess failed with exit code {:?}:\n{}\n{}",
            result.status.code(),
            String::from_utf8_lossy(result.stdout.as_slice()),
            String::from_utf8_lossy(result.stderr.as_slice()),
        );
        return Err(era_solc::StandardJsonOutputError::new_error(
            message,
            Some(era_solc::StandardJsonOutputErrorSourceLocation::new(
                path.to_owned(),
            )),
            None,
        ));
    }

    match era_compiler_common::deserialize_from_slice(result.stdout.as_slice()) {
        Ok(output) => output,
        Err(error) => {
            panic!(
                "{executable:?} subprocess stdout parsing error: {error:?}\n{}\n{}",
                String::from_utf8_lossy(result.stdout.as_slice()),
                String::from_utf8_lossy(result.stderr.as_slice()),
            );
        }
    }
}

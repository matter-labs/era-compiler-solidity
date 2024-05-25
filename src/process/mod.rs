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

use once_cell::sync::OnceCell;

use self::input_eravm::Input as EraVMInput;
use self::input_evm::Input as EVMInput;
use self::output_eravm::Output as EraVMOutput;
use self::output_evm::Output as EVMOutput;

/// The overriden executable name used when the compiler is run as a library.
pub static EXECUTABLE: OnceCell<PathBuf> = OnceCell::new();

///
/// Read input from `stdin`, compile a contract, and write the output to `stdout`.
///
pub fn run(target: era_compiler_llvm_context::Target) -> anyhow::Result<()> {
    match target {
        era_compiler_llvm_context::Target::EraVM => {
            let input: EraVMInput = era_compiler_common::deserialize_from_reader(std::io::stdin())
                .expect("Stdin reading error");

            if input.enable_test_encoding {
                zkevm_assembly::set_encoding_mode(zkevm_assembly::RunningVmEncodingMode::Testing);
            }
            let result = input.contract.into_owned().compile_to_eravm(
                input.project.into_owned(),
                input.optimizer_settings,
                input.is_system_mode,
                input.include_metadata_hash,
                input.debug_config,
            );

            match result {
                Ok(build) => {
                    let output = EraVMOutput::new(build);
                    serde_json::to_writer(std::io::stdout(), &output)
                        .expect("Stdout writing error");
                    Ok(())
                }
                Err(error) => {
                    std::io::stderr()
                        .write_all(error.to_string().as_bytes())
                        .expect("Stderr writing error");
                    Err(error)
                }
            }
        }
        era_compiler_llvm_context::Target::EVM => {
            let input: EVMInput = era_compiler_common::deserialize_from_reader(std::io::stdin())
                .expect("Stdin reading error");

            let result = input.contract.into_owned().compile_to_evm(
                input.project.into_owned(),
                input.optimizer_settings,
                input.include_metadata_hash,
                input.debug_config,
            );

            match result {
                Ok(build) => {
                    let output = EVMOutput::new(build);
                    serde_json::to_writer(std::io::stdout(), &output)
                        .expect("Stdout writing error");
                    Ok(())
                }
                Err(error) => {
                    std::io::stderr()
                        .write_all(error.to_string().as_bytes())
                        .expect("Stderr writing error");
                    Err(error)
                }
            }
        }
    }
}

///
/// Runs this process recursively to compile a single contract.
///
pub fn call<I, O>(input: I, target: era_compiler_llvm_context::Target) -> anyhow::Result<O>
where
    I: serde::Serialize,
    O: serde::de::DeserializeOwned,
{
    let executable = match EXECUTABLE.get() {
        Some(executable) => executable.to_owned(),
        None => std::env::current_exe()?,
    };

    let mut command = Command::new(executable.as_path());
    command.stdin(std::process::Stdio::piped());
    command.stdout(std::process::Stdio::piped());
    command.stderr(std::process::Stdio::piped());
    command.arg("--recursive-process");
    command.arg("--target");
    command.arg(target.to_string());
    let mut process = command.spawn().map_err(|error| {
        anyhow::anyhow!("{:?} subprocess spawning error: {:?}", executable, error)
    })?;

    let stdin = process
        .stdin
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("{:?} subprocess stdin getting error", executable))?;
    serde_json::to_writer(stdin, &input).map_err(|error| {
        anyhow::anyhow!(
            "{:?} subprocess stdin writing error: {:?}",
            executable,
            error
        )
    })?;
    let status = process.wait().map_err(|error| {
        anyhow::anyhow!("{:?} subprocess waiting error: {:?}", executable, error)
    })?;
    if !status.success() {
        let stderr = process
            .stderr
            .ok_or_else(|| anyhow::anyhow!("{:?} subprocess stderr getting error", executable))?;
        anyhow::bail!(
            "{}",
            std::io::read_to_string(stderr).map_err(|error| {
                anyhow::anyhow!(
                    "{:?} subprocess stderr reading error: {:?}",
                    executable,
                    error
                )
            })?
        );
    }

    let stdout = process
        .stdout
        .ok_or_else(|| anyhow::anyhow!("{:?} subprocess stdout getting error", executable))?;
    let output: O = era_compiler_common::deserialize_from_reader(stdout).map_err(|error| {
        anyhow::anyhow!(
            "{:?} subprocess stdout parsing error: {}",
            executable,
            error,
        )
    })?;
    Ok(output)
}

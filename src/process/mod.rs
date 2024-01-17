//!
//! Process for compiling a single compilation unit.
//!

pub mod input_eravm;
pub mod input_evm;
pub mod output_eravm;
pub mod output_evm;

use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

use once_cell::sync::OnceCell;
use serde::de::DeserializeOwned;
use serde::Serialize;

use self::input_eravm::Input as EraVMInput;
use self::input_evm::Input as EVMInput;
use self::output_eravm::Output as EraVMOutput;
use self::output_evm::Output as EVMOutput;

/// The overriden executable name used when the compiler is run as a library.
pub static EXECUTABLE: OnceCell<PathBuf> = OnceCell::new();

///
/// Read input from `stdin`, compile a contract, and write the output to `stdout`.
///
pub fn run(target: compiler_llvm_context::Target) -> anyhow::Result<()> {
    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut stderr = std::io::stderr();

    let mut buffer = Vec::with_capacity(16384);
    stdin.read_to_end(&mut buffer).expect("Stdin reading error");

    match target {
        compiler_llvm_context::Target::EraVM => {
            let input: EraVMInput = compiler_common::deserialize_from_slice(buffer.as_slice())?;

            if input.enable_test_encoding {
                zkevm_assembly::set_encoding_mode(zkevm_assembly::RunningVmEncodingMode::Testing);
            }
            let result = input.contract.compile_to_eravm(
                input.project,
                input.optimizer_settings,
                input.is_system_mode,
                input.include_metadata_hash,
                input.debug_config,
            );

            match result {
                Ok(build) => {
                    let output = EraVMOutput::new(build);
                    let json = serde_json::to_vec(&output).expect("Always valid");
                    stdout
                        .write_all(json.as_slice())
                        .expect("Stdout writing error");
                    Ok(())
                }
                Err(error) => {
                    let message = error.to_string();
                    stderr
                        .write_all(message.as_bytes())
                        .expect("Stderr writing error");
                    Err(error)
                }
            }
        }
        compiler_llvm_context::Target::EVM => {
            let input: EVMInput = compiler_common::deserialize_from_slice(buffer.as_slice())?;

            let result = input.contract.compile_to_evm(
                input.project,
                input.optimizer_settings,
                input.include_metadata_hash,
                input.debug_config,
            );

            match result {
                Ok(build) => {
                    let output = EVMOutput::new(build);
                    let json = serde_json::to_vec(&output).expect("Always valid");
                    stdout
                        .write_all(json.as_slice())
                        .expect("Stdout writing error");
                    Ok(())
                }
                Err(error) => {
                    let message = error.to_string();
                    stderr
                        .write_all(message.as_bytes())
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
pub fn call<I, O>(input: I, target: compiler_llvm_context::Target) -> anyhow::Result<O>
where
    I: Serialize,
    O: DeserializeOwned,
{
    let input_json = serde_json::to_vec(&input).expect("Always valid");

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
    let process = command.spawn().map_err(|error| {
        anyhow::anyhow!("{:?} subprocess spawning error: {:?}", executable, error)
    })?;

    process
        .stdin
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("{:?} stdin getting error", executable))?
        .write_all(input_json.as_slice())
        .map_err(|error| anyhow::anyhow!("{:?} stdin writing error: {:?}", executable, error))?;
    let output = process.wait_with_output().map_err(|error| {
        anyhow::anyhow!("{:?} subprocess output error: {:?}", executable, error)
    })?;
    if !output.status.success() {
        anyhow::bail!(
            "{}",
            String::from_utf8_lossy(output.stderr.as_slice()).to_string(),
        );
    }

    let output: O =
        compiler_common::deserialize_from_slice(output.stdout.as_slice()).map_err(|error| {
            anyhow::anyhow!(
                "{:?} subprocess output parsing error: {}",
                executable,
                error,
            )
        })?;
    Ok(output)
}

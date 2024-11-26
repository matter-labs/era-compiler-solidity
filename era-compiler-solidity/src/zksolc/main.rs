//!
//! Solidity to EraVM compiler binary.
//!

pub mod arguments;

use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;

use clap::Parser;

use self::arguments::Arguments;

/// The rayon worker stack size.
const RAYON_WORKER_STACK_SIZE: usize = 16 * 1024 * 1024;

#[cfg(target_env = "musl")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

///
/// The application entry point.
///
fn main() -> anyhow::Result<()> {
    let arguments = Arguments::try_parse()?;
    let is_standard_json = arguments.standard_json.is_some();
    let mut messages = arguments.validate();
    if messages.iter().all(|error| error.severity != "error") {
        if let Err(error) = main_inner(arguments, &mut messages) {
            messages.push(era_solc::StandardJsonOutputError::new_error(
                error, None, None,
            ));
        }
    }

    if is_standard_json {
        let output = era_solc::StandardJsonOutput::new_with_messages(messages);
        output.write_and_exit(era_solc::StandardJsonInputSelection::default());
    }

    let exit_code = if messages.iter().any(|error| error.severity == "error") {
        era_compiler_common::EXIT_CODE_FAILURE
    } else {
        era_compiler_common::EXIT_CODE_SUCCESS
    };
    std::io::stderr()
        .write_all(
            messages
                .into_iter()
                .map(|error| error.to_string())
                .collect::<Vec<String>>()
                .join("\n")
                .as_bytes(),
        )
        .expect("Stderr writing error");
    std::process::exit(exit_code);
}

///
/// The auxiliary `main` function to facilitate the `?` error conversion operator.
///
fn main_inner(
    arguments: Arguments,
    messages: &mut Vec<era_solc::StandardJsonOutputError>,
) -> anyhow::Result<()> {
    if arguments.version {
        writeln!(
            std::io::stdout(),
            "{} v{} (LLVM build {})",
            env!("CARGO_PKG_DESCRIPTION"),
            era_compiler_solidity::version(),
            inkwell::support::get_commit_id().to_string(),
        )?;
        return Ok(());
    }

    let target = match arguments.target {
        Some(ref target) => era_compiler_common::Target::from_str(target.as_str())?,
        None => era_compiler_common::Target::EraVM,
    };

    let mut thread_pool_builder = rayon::ThreadPoolBuilder::new();
    if let Some(threads) = arguments.threads {
        thread_pool_builder = thread_pool_builder.num_threads(threads);
    }
    thread_pool_builder
        .stack_size(RAYON_WORKER_STACK_SIZE)
        .build_global()
        .expect("Thread pool configuration failure");

    inkwell::support::enable_llvm_pretty_stack_trace();
    era_compiler_llvm_context::initialize_target(target);

    if arguments.recursive_process {
        return era_compiler_solidity::run_recursive(target);
    }

    let (input_files, remappings) = arguments.split_input_files_and_remappings()?;

    let mut optimizer_settings = match arguments.optimization {
        Some(mode) => era_compiler_llvm_context::OptimizerSettings::try_from_cli(mode)?,
        None => era_compiler_llvm_context::OptimizerSettings::cycles(),
    };
    if arguments.fallback_to_optimizing_for_size {
        optimizer_settings.enable_fallback_to_size();
    }
    optimizer_settings.is_verify_each_enabled = arguments.llvm_verify_each;
    optimizer_settings.is_debug_logging_enabled = arguments.llvm_debug_logging;

    let llvm_options: Vec<String> = arguments
        .llvm_options
        .as_ref()
        .map(|options| {
            options
                .split_whitespace()
                .map(|option| option.to_owned())
                .collect()
        })
        .unwrap_or_default();

    let suppressed_errors = era_solc::StandardJsonInputErrorType::try_from_strings(
        arguments.suppress_errors.unwrap_or_default().as_slice(),
    )?;
    let suppressed_warnings = era_solc::StandardJsonInputWarningType::try_from_strings(
        arguments.suppress_warnings.unwrap_or_default().as_slice(),
    )?;

    let debug_config = match arguments.debug_output_dir {
        Some(ref debug_output_directory) => {
            std::fs::create_dir_all(debug_output_directory.as_path())?;
            Some(era_compiler_llvm_context::DebugConfig::new(
                debug_output_directory.to_owned(),
            ))
        }
        None => None,
    };

    let enable_eravm_extensions = arguments.enable_eravm_extensions || arguments.system_mode;

    let metadata_hash_type = arguments
        .metadata_hash
        .unwrap_or(era_compiler_common::HashType::Keccak256);

    match target {
        era_compiler_common::Target::EraVM => {
            let build = if arguments.yul {
                era_compiler_solidity::yul_to_eravm(
                    input_files.as_slice(),
                    arguments.libraries.as_slice(),
                    arguments.solc,
                    messages,
                    enable_eravm_extensions,
                    metadata_hash_type,
                    optimizer_settings,
                    llvm_options,
                    arguments.output_assembly,
                    debug_config,
                )
            } else if arguments.llvm_ir {
                era_compiler_solidity::llvm_ir_to_eravm(
                    input_files.as_slice(),
                    arguments.libraries.as_slice(),
                    messages,
                    metadata_hash_type,
                    optimizer_settings,
                    llvm_options,
                    arguments.output_assembly,
                    debug_config,
                )
            } else if arguments.eravm_assembly {
                era_compiler_solidity::eravm_assembly(
                    input_files.as_slice(),
                    messages,
                    metadata_hash_type,
                    llvm_options,
                    arguments.output_assembly,
                    debug_config,
                )
            } else if arguments.disassemble {
                return era_compiler_solidity::disassemble_eravm(arguments.inputs);
            } else if arguments.link {
                return match arguments.standard_json {
                    Some(path) => era_compiler_solidity::link_eravm_json(path),
                    None => {
                        era_compiler_solidity::link_eravm(arguments.inputs, arguments.libraries)
                    }
                };
            } else if let Some(standard_json) = arguments.standard_json {
                let solc_compiler = match arguments.solc.as_deref() {
                    Some(executable) => Some(era_solc::Compiler::try_from_path(executable)?),
                    None => None,
                };
                return era_compiler_solidity::standard_json_eravm(
                    solc_compiler,
                    arguments.codegen,
                    enable_eravm_extensions,
                    arguments.detect_missing_libraries,
                    standard_json.map(PathBuf::from),
                    messages,
                    arguments.base_path,
                    arguments.include_path,
                    arguments.allow_paths,
                    debug_config,
                );
            } else if let Some(format) = arguments.combined_json {
                let solc_compiler = era_solc::Compiler::try_from_path(
                    arguments
                        .solc
                        .as_deref()
                        .unwrap_or(era_solc::Compiler::DEFAULT_EXECUTABLE_NAME),
                )?;
                return era_compiler_solidity::combined_json_eravm(
                    format,
                    input_files.as_slice(),
                    arguments.libraries.as_slice(),
                    &solc_compiler,
                    messages,
                    arguments.codegen,
                    arguments.evm_version,
                    enable_eravm_extensions,
                    metadata_hash_type,
                    arguments.metadata_literal,
                    arguments.base_path,
                    arguments.include_path,
                    arguments.allow_paths,
                    remappings,
                    arguments.output_dir,
                    arguments.overwrite,
                    optimizer_settings,
                    llvm_options,
                    arguments.output_assembly,
                    suppressed_errors,
                    suppressed_warnings,
                    debug_config,
                );
            } else {
                let solc_compiler = era_solc::Compiler::try_from_path(
                    arguments
                        .solc
                        .as_deref()
                        .unwrap_or(era_solc::Compiler::DEFAULT_EXECUTABLE_NAME),
                )?;
                era_compiler_solidity::standard_output_eravm(
                    input_files.as_slice(),
                    arguments.libraries.as_slice(),
                    &solc_compiler,
                    messages,
                    arguments.codegen,
                    arguments.evm_version,
                    enable_eravm_extensions,
                    metadata_hash_type,
                    arguments.metadata_literal,
                    arguments.base_path,
                    arguments.include_path,
                    arguments.allow_paths,
                    remappings,
                    optimizer_settings,
                    llvm_options,
                    arguments.output_assembly,
                    suppressed_errors,
                    suppressed_warnings,
                    debug_config,
                )
            }?;

            if let Some(output_directory) = arguments.output_dir {
                build.write_to_directory(
                    &output_directory,
                    arguments.output_metadata,
                    arguments.output_binary,
                    arguments.overwrite,
                )?;
            } else {
                build.write_to_terminal(
                    arguments.output_metadata,
                    arguments.output_assembly,
                    arguments.output_binary,
                )?;
            }
        }
        era_compiler_common::Target::EVM => {
            let build = if arguments.yul {
                era_compiler_solidity::yul_to_evm(
                    input_files.as_slice(),
                    arguments.libraries.as_slice(),
                    arguments.solc,
                    messages,
                    metadata_hash_type,
                    optimizer_settings,
                    llvm_options,
                    arguments.threads,
                    debug_config,
                )
            } else if arguments.llvm_ir {
                era_compiler_solidity::llvm_ir_to_evm(
                    input_files.as_slice(),
                    arguments.libraries.as_slice(),
                    messages,
                    metadata_hash_type,
                    optimizer_settings,
                    llvm_options,
                    arguments.threads,
                    debug_config,
                )
            } else if arguments.disassemble {
                anyhow::bail!("The EVM target does not support disassembling yet.");
            } else if arguments.link {
                anyhow::bail!("The EVM target does not support linking yet.");
            } else if let Some(standard_json) = arguments.standard_json {
                let solc_compiler = match arguments.solc.as_deref() {
                    Some(executable) => Some(era_solc::Compiler::try_from_path(executable)?),
                    None => None,
                };
                return era_compiler_solidity::standard_json_evm(
                    solc_compiler,
                    arguments.codegen,
                    standard_json.map(PathBuf::from),
                    messages,
                    arguments.base_path,
                    arguments.include_path,
                    arguments.allow_paths,
                    arguments.threads,
                    debug_config,
                );
            } else if let Some(format) = arguments.combined_json {
                let solc_compiler = era_solc::Compiler::try_from_path(
                    arguments
                        .solc
                        .as_deref()
                        .unwrap_or(era_solc::Compiler::DEFAULT_EXECUTABLE_NAME),
                )?;
                return era_compiler_solidity::combined_json_evm(
                    format,
                    input_files.as_slice(),
                    arguments.libraries.as_slice(),
                    &solc_compiler,
                    messages,
                    arguments.codegen,
                    arguments.evm_version,
                    metadata_hash_type,
                    arguments.metadata_literal,
                    arguments.base_path,
                    arguments.include_path,
                    arguments.allow_paths,
                    remappings,
                    arguments.output_dir,
                    arguments.overwrite,
                    optimizer_settings,
                    llvm_options,
                    arguments.threads,
                    debug_config,
                );
            } else {
                let solc = era_solc::Compiler::try_from_path(
                    arguments
                        .solc
                        .as_deref()
                        .unwrap_or(era_solc::Compiler::DEFAULT_EXECUTABLE_NAME),
                )?;
                era_compiler_solidity::standard_output_evm(
                    input_files.as_slice(),
                    arguments.libraries.as_slice(),
                    &solc,
                    messages,
                    arguments.codegen,
                    arguments.evm_version,
                    metadata_hash_type,
                    arguments.metadata_literal,
                    arguments.base_path,
                    arguments.include_path,
                    arguments.allow_paths,
                    remappings,
                    optimizer_settings,
                    llvm_options,
                    arguments.threads,
                    debug_config,
                )
            }?;

            if let Some(output_directory) = arguments.output_dir {
                build.write_to_directory(
                    &output_directory,
                    arguments.output_metadata,
                    arguments.output_assembly,
                    arguments.output_binary,
                    arguments.overwrite,
                )?;
            } else {
                build.write_to_terminal(
                    arguments.output_metadata,
                    arguments.output_assembly,
                    arguments.output_binary,
                )?;
            }
        }
    }

    Ok(())
}

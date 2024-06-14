//!
//! Solidity to EraVM compiler binary.
//!

pub mod arguments;

use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;

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
    std::process::exit(match main_inner() {
        Ok(()) => era_compiler_common::EXIT_CODE_SUCCESS,
        Err(error) => {
            std::io::stderr()
                .write_all(error.to_string().as_bytes())
                .expect("Stderr writing error");
            era_compiler_common::EXIT_CODE_FAILURE
        }
    })
}

///
/// The auxiliary `main` function to facilitate the `?` error conversion operator.
///
fn main_inner() -> anyhow::Result<()> {
    let arguments = Arguments::new();
    arguments.validate()?;

    if arguments.version {
        writeln!(
            std::io::stdout(),
            "{} v{} (LLVM build {})",
            env!("CARGO_PKG_DESCRIPTION"),
            env!("CARGO_PKG_VERSION"),
            inkwell::support::get_commit_id().to_string(),
        )?;
        return Ok(());
    }

    let target = match arguments.target {
        Some(ref target) => era_compiler_llvm_context::Target::from_str(target.as_str())?,
        None => era_compiler_llvm_context::Target::EraVM,
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
    if let era_compiler_llvm_context::Target::EVM = target {
        anyhow::bail!("The EVM target is under development and not available yet.")
    }

    let debug_config = match arguments.debug_output_directory {
        Some(ref debug_output_directory) => {
            std::fs::create_dir_all(debug_output_directory.as_path())?;
            Some(era_compiler_llvm_context::DebugConfig::new(
                debug_output_directory.to_owned(),
            ))
        }
        None => None,
    };

    let (input_files, remappings) = arguments.split_input_files_and_remappings()?;

    let suppressed_warnings = match arguments.suppress_warnings {
        Some(warnings) => Some(era_compiler_solidity::Warning::try_from_strings(
            warnings.as_slice(),
        )?),
        None => None,
    };

    let evm_version = match arguments.evm_version {
        Some(evm_version) => Some(era_compiler_common::EVMVersion::try_from(
            evm_version.as_str(),
        )?),
        None => None,
    };

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
        .map(|options| options.split(' ').map(|option| option.to_owned()).collect())
        .unwrap_or_default();

    let include_metadata_hash = match arguments.metadata_hash {
        Some(metadata_hash) => {
            let metadata =
                era_compiler_llvm_context::EraVMMetadataHash::from_str(metadata_hash.as_str())?;
            metadata != era_compiler_llvm_context::EraVMMetadataHash::None
        }
        None => true,
    };

    match target {
        era_compiler_llvm_context::Target::EraVM => {
            let build = if arguments.yul {
                era_compiler_solidity::yul_to_eravm(
                    input_files.as_slice(),
                    arguments.libraries,
                    arguments.solc,
                    optimizer_settings,
                    llvm_options,
                    arguments.enable_eravm_extensions,
                    include_metadata_hash,
                    arguments.threads,
                    debug_config,
                )
            } else if arguments.llvm_ir {
                era_compiler_solidity::llvm_ir_to_eravm(
                    input_files.as_slice(),
                    optimizer_settings,
                    llvm_options,
                    include_metadata_hash,
                    arguments.threads,
                    debug_config,
                )
            } else if arguments.eravm_assembly {
                era_compiler_solidity::eravm_assembly(
                    input_files.as_slice(),
                    llvm_options,
                    include_metadata_hash,
                    arguments.threads,
                    debug_config,
                )
            } else if let Some(standard_json) = arguments.standard_json {
                let solc_compiler = match arguments.solc.as_deref() {
                    Some(executable) => Some(era_compiler_solidity::SolcCompiler::new(executable)?),
                    None => None,
                };
                era_compiler_solidity::standard_json_eravm(
                    solc_compiler.as_ref(),
                    arguments.force_evmla,
                    arguments.enable_eravm_extensions,
                    arguments.detect_missing_libraries,
                    standard_json.map(PathBuf::from),
                    arguments.base_path,
                    arguments.include_paths,
                    arguments.allow_paths,
                    arguments.threads,
                    debug_config,
                )?;
                return Ok(());
            } else if let Some(format) = arguments.combined_json {
                let solc_compiler = era_compiler_solidity::SolcCompiler::new(
                    arguments
                        .solc
                        .as_deref()
                        .unwrap_or(era_compiler_solidity::SolcCompiler::DEFAULT_EXECUTABLE_NAME),
                )?;
                era_compiler_solidity::combined_json_eravm(
                    format,
                    input_files.as_slice(),
                    arguments.libraries,
                    &solc_compiler,
                    evm_version,
                    !arguments.disable_solc_optimizer,
                    arguments.force_evmla,
                    arguments.enable_eravm_extensions,
                    include_metadata_hash,
                    arguments.metadata_literal,
                    arguments.base_path,
                    arguments.include_paths,
                    arguments.allow_paths,
                    remappings,
                    arguments.output_directory,
                    arguments.overwrite,
                    optimizer_settings,
                    llvm_options,
                    suppressed_warnings,
                    arguments.threads,
                    debug_config,
                )?;
                return Ok(());
            } else {
                let solc_compiler = era_compiler_solidity::SolcCompiler::new(
                    arguments
                        .solc
                        .as_deref()
                        .unwrap_or(era_compiler_solidity::SolcCompiler::DEFAULT_EXECUTABLE_NAME),
                )?;
                era_compiler_solidity::standard_output_eravm(
                    input_files.as_slice(),
                    arguments.libraries,
                    &solc_compiler,
                    evm_version,
                    !arguments.disable_solc_optimizer,
                    arguments.force_evmla,
                    arguments.enable_eravm_extensions,
                    include_metadata_hash,
                    arguments.metadata_literal,
                    arguments.base_path,
                    arguments.include_paths,
                    arguments.allow_paths,
                    remappings,
                    optimizer_settings,
                    llvm_options,
                    suppressed_warnings,
                    arguments.threads,
                    debug_config,
                )
            }?;

            if let Some(output_directory) = arguments.output_directory {
                std::fs::create_dir_all(&output_directory)?;

                build.write_to_directory(
                    &output_directory,
                    arguments.output_assembly,
                    arguments.output_binary,
                    arguments.overwrite,
                )?;

                writeln!(
                    std::io::stderr(),
                    "Compiler run successful. Artifact(s) can be found in directory {output_directory:?}."
                )?;
            } else if arguments.output_assembly || arguments.output_binary {
                build.write_to_terminal(arguments.output_assembly, arguments.output_binary)?;
            } else {
                writeln!(
                    std::io::stderr(),
                    "Compiler run successful. No output requested. Use --asm and --bin flags."
                )?;
            }
        }
        era_compiler_llvm_context::Target::EVM => {
            let build = if arguments.yul {
                era_compiler_solidity::yul_to_evm(
                    input_files.as_slice(),
                    arguments.libraries,
                    arguments.solc,
                    optimizer_settings,
                    llvm_options,
                    include_metadata_hash,
                    arguments.threads,
                    debug_config,
                )
            } else if arguments.llvm_ir {
                era_compiler_solidity::llvm_ir_to_evm(
                    input_files.as_slice(),
                    optimizer_settings,
                    llvm_options,
                    include_metadata_hash,
                    arguments.threads,
                    debug_config,
                )
            } else if let Some(standard_json) = arguments.standard_json {
                let solc_compiler = match arguments.solc.as_deref() {
                    Some(executable) => Some(era_compiler_solidity::SolcCompiler::new(executable)?),
                    None => None,
                };
                era_compiler_solidity::standard_json_evm(
                    solc_compiler.as_ref(),
                    arguments.force_evmla,
                    standard_json.map(PathBuf::from),
                    arguments.base_path,
                    arguments.include_paths,
                    arguments.allow_paths,
                    arguments.threads,
                    debug_config,
                )?;
                return Ok(());
            } else if let Some(format) = arguments.combined_json {
                let solc_compiler = era_compiler_solidity::SolcCompiler::new(
                    arguments
                        .solc
                        .as_deref()
                        .unwrap_or(era_compiler_solidity::SolcCompiler::DEFAULT_EXECUTABLE_NAME),
                )?;
                era_compiler_solidity::combined_json_evm(
                    format,
                    input_files.as_slice(),
                    arguments.libraries,
                    &solc_compiler,
                    evm_version,
                    !arguments.disable_solc_optimizer,
                    arguments.force_evmla,
                    include_metadata_hash,
                    arguments.metadata_literal,
                    arguments.base_path,
                    arguments.include_paths,
                    arguments.allow_paths,
                    remappings,
                    arguments.output_directory,
                    arguments.overwrite,
                    optimizer_settings,
                    llvm_options,
                    arguments.threads,
                    debug_config,
                )?;
                return Ok(());
            } else {
                let solc = era_compiler_solidity::SolcCompiler::new(
                    arguments
                        .solc
                        .as_deref()
                        .unwrap_or(era_compiler_solidity::SolcCompiler::DEFAULT_EXECUTABLE_NAME),
                )?;
                era_compiler_solidity::standard_output_evm(
                    input_files.as_slice(),
                    arguments.libraries,
                    &solc,
                    evm_version,
                    !arguments.disable_solc_optimizer,
                    arguments.force_evmla,
                    include_metadata_hash,
                    arguments.metadata_literal,
                    arguments.base_path,
                    arguments.include_paths,
                    arguments.allow_paths,
                    remappings,
                    optimizer_settings,
                    llvm_options,
                    arguments.threads,
                    debug_config,
                )
            }?;

            if let Some(output_directory) = arguments.output_directory {
                std::fs::create_dir_all(&output_directory)?;

                build.write_to_directory(
                    &output_directory,
                    arguments.output_assembly,
                    arguments.output_binary,
                    arguments.overwrite,
                )?;

                writeln!(
                    std::io::stderr(),
                    "Compiler run successful. Artifact(s) can be found in directory {output_directory:?}."
                )?;
            } else if arguments.output_assembly || arguments.output_binary {
                build.write_to_terminal(arguments.output_assembly, arguments.output_binary)?;
            } else {
                writeln!(
                    std::io::stderr(),
                    "Compiler run successful. No output requested. Use --asm and --bin flags."
                )?;
            }
        }
    }

    Ok(())
}

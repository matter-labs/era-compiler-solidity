//!
//! Solidity to zkEVM compiler binary.
//!

pub mod arguments;

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
fn main() {
    std::process::exit(match main_inner() {
        Ok(()) => compiler_common::EXIT_CODE_SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            compiler_common::EXIT_CODE_FAILURE
        }
    })
}

///
/// The auxiliary `main` function to facilitate the `?` error conversion operator.
///
fn main_inner() -> anyhow::Result<()> {
    let mut arguments = Arguments::new();
    arguments.validate()?;

    if arguments.version {
        println!(
            "{} v{} (LLVM build {})",
            env!("CARGO_PKG_DESCRIPTION"),
            env!("CARGO_PKG_VERSION"),
            inkwell::support::get_commit_id().to_string(),
        );
        return Ok(());
    }

    let debug_config = match arguments.debug_output_directory {
        Some(debug_output_directory) => {
            std::fs::create_dir_all(debug_output_directory.as_path())?;
            Some(compiler_llvm_context::DebugConfig::new(
                debug_output_directory,
            ))
        }
        None => None,
    };

    rayon::ThreadPoolBuilder::new()
        .stack_size(RAYON_WORKER_STACK_SIZE)
        .build_global()
        .expect("Thread pool configuration failure");

    for path in arguments.input_files.iter_mut() {
        *path = path.canonicalize()?;
    }

    inkwell::support::enable_llvm_pretty_stack_trace();
    compiler_llvm_context::initialize_target();

    let mut solc =
        compiler_solidity::SolcCompiler::new(arguments.solc.unwrap_or_else(|| {
            compiler_solidity::SolcCompiler::DEFAULT_EXECUTABLE_NAME.to_owned()
        }));

    let mut optimizer_settings = match arguments.optimization {
        Some(mode) => compiler_llvm_context::OptimizerSettings::try_from_cli(mode)?,
        None => compiler_llvm_context::OptimizerSettings::cycles(),
    };
    optimizer_settings.is_verify_each_enabled = arguments.llvm_verify_each;
    optimizer_settings.is_debug_logging_enabled = arguments.llvm_debug_logging;

    let include_metadata_hash = match arguments.metadata_hash {
        Some(metadata_hash) => {
            let metadata = compiler_llvm_context::MetadataHash::from_str(metadata_hash.as_str())?;
            metadata != compiler_llvm_context::MetadataHash::None
        }
        None => true,
    };

    let build = if arguments.yul {
        compiler_solidity::yul(
            arguments.input_files.as_slice(),
            optimizer_settings,
            arguments.is_system_mode,
            include_metadata_hash,
            debug_config,
        )
    } else if arguments.llvm_ir {
        compiler_solidity::llvm_ir(
            arguments.input_files.as_slice(),
            optimizer_settings,
            arguments.is_system_mode,
            include_metadata_hash,
            debug_config,
        )
    } else if arguments.standard_json {
        compiler_solidity::standard_json(
            &mut solc,
            arguments.force_evmla,
            arguments.is_system_mode,
            arguments.base_path,
            arguments.include_paths,
            arguments.allow_paths,
            debug_config,
        )?;
        return Ok(());
    } else if let Some(format) = arguments.combined_json {
        compiler_solidity::combined_json(
            format,
            arguments.input_files.as_slice(),
            arguments.libraries,
            &mut solc,
            !arguments.disable_solc_optimizer,
            optimizer_settings,
            arguments.force_evmla,
            arguments.is_system_mode,
            include_metadata_hash,
            arguments.base_path,
            arguments.include_paths,
            arguments.allow_paths,
            debug_config,
            arguments.output_directory,
            arguments.overwrite,
        )?;
        return Ok(());
    } else {
        compiler_solidity::standard_output(
            arguments.input_files.as_slice(),
            arguments.libraries,
            &mut solc,
            !arguments.disable_solc_optimizer,
            optimizer_settings,
            arguments.force_evmla,
            arguments.is_system_mode,
            include_metadata_hash,
            arguments.base_path,
            arguments.include_paths,
            arguments.allow_paths,
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

        eprintln!(
            "Compiler run successful. Artifact(s) can be found in directory {output_directory:?}."
        );
    } else if arguments.output_assembly || arguments.output_binary {
        for (path, contract) in build.contracts.into_iter() {
            if arguments.output_assembly {
                println!(
                    "Contract `{}` assembly:\n\n{}",
                    path, contract.build.assembly_text
                );
            }
            if arguments.output_binary {
                println!(
                    "Contract `{}` bytecode: 0x{}",
                    path,
                    hex::encode(contract.build.bytecode)
                );
            }
        }
    } else {
        eprintln!("Compiler run successful. No output requested. Use --asm and --bin flags.");
    }

    Ok(())
}

//!
//! Solidity to EraVM compiler library.
//!

#![allow(non_camel_case_types)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::enum_variant_names)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::result_large_err)]

pub mod build_eravm;
pub mod build_evm;
pub mod r#const;
pub mod evmla;
pub mod linker;
pub mod missing_libraries;
pub mod process;
pub mod project;
pub mod yul;

pub use self::build_eravm::contract::Contract as EraVMContractBuild;
pub use self::build_eravm::Build as EraVMBuild;
pub use self::build_evm::contract::Contract as EVMContractBuild;
pub use self::build_evm::Build as EVMBuild;
pub use self::linker::input::Input as LinkerInput;
pub use self::linker::output::Output as LinkerOutput;
pub use self::linker::Linker;
pub use self::process::input_eravm::Input as EraVMProcessInput;
pub use self::process::input_evm::Input as EVMProcessInput;
pub use self::process::output_eravm::Output as EraVMProcessOutput;
pub use self::process::output_evm::Output as EVMProcessOutput;
pub use self::process::run as run_recursive;
pub use self::process::EXECUTABLE;
pub use self::project::contract::Contract as ProjectContract;
pub use self::project::Project;
pub use self::r#const::*;

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashSet;
use std::io::Write;
use std::path::PathBuf;

use rayon::iter::IntoParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

use era_solc::CollectableError;

/// The default error compatible with `solc` standard JSON output.
pub type Result<T> = std::result::Result<T, era_solc::StandardJsonOutputError>;

///
/// Runs the Yul mode for the EraVM target.
///
pub fn yul_to_eravm(
    paths: &[PathBuf],
    libraries: &[String],
    solc_path: Option<String>,
    messages: &mut Vec<era_solc::StandardJsonOutputError>,
    enable_eravm_extensions: bool,
    metadata_hash_type: era_compiler_common::HashType,
    optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
    llvm_options: Vec<String>,
    output_assembly: bool,
    debug_config: Option<era_compiler_llvm_context::DebugConfig>,
) -> anyhow::Result<EraVMBuild> {
    let libraries = era_solc::StandardJsonInputLibraries::try_from(libraries)?;
    let linker_symbols = libraries.as_linker_symbols()?;

    let solc_version = match solc_path {
        Some(solc_path) => {
            if enable_eravm_extensions {
                anyhow::bail!("Yul validation cannot be done if EraVM extensions are enabled. Consider compiling without `solc`.")
            }
            let solc_compiler = era_solc::Compiler::try_from_path(solc_path.as_str())?;
            solc_compiler.validate_yul_paths(paths, libraries.clone(), messages)?;
            Some(solc_compiler.version)
        }
        None => None,
    };

    let project = Project::try_from_yul_paths(
        paths,
        libraries,
        None,
        solc_version.as_ref(),
        debug_config.as_ref(),
    )?;

    let mut build = project.compile_to_eravm(
        messages,
        enable_eravm_extensions,
        metadata_hash_type,
        optimizer_settings,
        llvm_options,
        output_assembly,
        debug_config,
    )?;
    build.take_and_write_warnings();
    build.check_errors()?;

    let mut build = build.link(linker_symbols);
    build.take_and_write_warnings();
    build.check_errors()?;
    Ok(build)
}

///
/// Runs the Yul mode for the EVM target.
///
pub fn yul_to_evm(
    paths: &[PathBuf],
    libraries: &[String],
    solc_path: Option<String>,
    messages: &mut Vec<era_solc::StandardJsonOutputError>,
    metadata_hash_type: era_compiler_common::HashType,
    optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
    llvm_options: Vec<String>,
    threads: Option<usize>,
    debug_config: Option<era_compiler_llvm_context::DebugConfig>,
) -> anyhow::Result<EVMBuild> {
    let libraries = era_solc::StandardJsonInputLibraries::try_from(libraries)?;
    let _linker_symbols = libraries.as_linker_symbols()?;

    let solc_version = match solc_path {
        Some(solc_path) => {
            let solc_compiler = era_solc::Compiler::try_from_path(solc_path.as_str())?;
            solc_compiler.validate_yul_paths(paths, libraries.clone(), messages)?;
            Some(solc_compiler.version)
        }
        None => None,
    };

    let project = Project::try_from_yul_paths(
        paths,
        libraries,
        None,
        solc_version.as_ref(),
        debug_config.as_ref(),
    )?;

    let build = project.compile_to_evm(
        messages,
        metadata_hash_type,
        optimizer_settings,
        llvm_options,
        threads,
        debug_config,
    )?;
    Ok(build)
}

///
/// Runs the LLVM IR mode for the EraVM target.
///
pub fn llvm_ir_to_eravm(
    paths: &[PathBuf],
    libraries: &[String],
    messages: &mut Vec<era_solc::StandardJsonOutputError>,
    metadata_hash_type: era_compiler_common::HashType,
    optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
    llvm_options: Vec<String>,
    output_assembly: bool,
    debug_config: Option<era_compiler_llvm_context::DebugConfig>,
) -> anyhow::Result<EraVMBuild> {
    let libraries = era_solc::StandardJsonInputLibraries::try_from(libraries)?;
    let linker_symbols = libraries.as_linker_symbols()?;

    let project = Project::try_from_llvm_ir_paths(paths, libraries, None)?;

    let mut build = project.compile_to_eravm(
        messages,
        false,
        metadata_hash_type,
        optimizer_settings,
        llvm_options,
        output_assembly,
        debug_config,
    )?;
    build.take_and_write_warnings();
    build.check_errors()?;

    let mut build = build.link(linker_symbols);
    build.take_and_write_warnings();
    build.check_errors()?;
    Ok(build)
}

///
/// Runs the LLVM IR mode for the EVM target.
///
pub fn llvm_ir_to_evm(
    paths: &[PathBuf],
    libraries: &[String],
    messages: &mut Vec<era_solc::StandardJsonOutputError>,
    metadata_hash_type: era_compiler_common::HashType,
    optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
    llvm_options: Vec<String>,
    threads: Option<usize>,
    debug_config: Option<era_compiler_llvm_context::DebugConfig>,
) -> anyhow::Result<EVMBuild> {
    let libraries = era_solc::StandardJsonInputLibraries::try_from(libraries)?;

    let project = Project::try_from_llvm_ir_paths(paths, libraries, None)?;

    let build = project.compile_to_evm(
        messages,
        metadata_hash_type,
        optimizer_settings,
        llvm_options,
        threads,
        debug_config,
    )?;
    Ok(build)
}

///
/// Runs the EraVM assembly mode for the EraVM target.
///
pub fn eravm_assembly_to_eravm(
    paths: &[PathBuf],
    messages: &mut Vec<era_solc::StandardJsonOutputError>,
    metadata_hash_type: era_compiler_common::HashType,
    llvm_options: Vec<String>,
    output_assembly: bool,
    debug_config: Option<era_compiler_llvm_context::DebugConfig>,
) -> anyhow::Result<EraVMBuild> {
    let project = Project::try_from_eravm_assembly_paths(paths, None)?;

    let optimizer_settings = era_compiler_llvm_context::OptimizerSettings::none();
    let mut build = project.compile_to_eravm(
        messages,
        false,
        metadata_hash_type,
        optimizer_settings,
        llvm_options,
        output_assembly,
        debug_config,
    )?;
    build.take_and_write_warnings();
    build.check_errors()?;

    let mut build = build.link(BTreeMap::new());
    build.take_and_write_warnings();
    build.check_errors()?;
    Ok(build)
}

///
/// Runs the EraVM assembly mode for the EraVM target.
///
pub fn eravm_assembly_to_evm(
    paths: &[PathBuf],
    messages: &mut Vec<era_solc::StandardJsonOutputError>,
    metadata_hash_type: era_compiler_common::HashType,
    llvm_options: Vec<String>,
    threads: Option<usize>,
    debug_config: Option<era_compiler_llvm_context::DebugConfig>,
) -> anyhow::Result<EVMBuild> {
    let project = Project::try_from_eravm_assembly_paths(paths, None)?;

    let optimizer_settings = era_compiler_llvm_context::OptimizerSettings::none();
    let mut build = project.compile_to_evm(
        messages,
        metadata_hash_type,
        optimizer_settings,
        llvm_options,
        threads,
        debug_config,
    )?;
    build.take_and_write_warnings();
    build.check_errors()?;
    Ok(build)
}

///
/// Runs the standard output mode for the EraVM target.
///
pub fn standard_output_eravm(
    paths: &[PathBuf],
    libraries: &[String],
    solc_compiler: &era_solc::Compiler,
    messages: &mut Vec<era_solc::StandardJsonOutputError>,
    codegen: Option<era_solc::StandardJsonInputCodegen>,
    evm_version: Option<era_compiler_common::EVMVersion>,
    enable_eravm_extensions: bool,
    metadata_hash_type: era_compiler_common::HashType,
    use_literal_content: bool,
    base_path: Option<String>,
    include_paths: Vec<String>,
    allow_paths: Option<String>,
    remappings: BTreeSet<String>,
    optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
    llvm_options: Vec<String>,
    output_assembly: bool,
    suppressed_errors: Vec<era_solc::StandardJsonInputErrorType>,
    suppressed_warnings: Vec<era_solc::StandardJsonInputWarningType>,
    debug_config: Option<era_compiler_llvm_context::DebugConfig>,
) -> anyhow::Result<EraVMBuild> {
    let solc_version = solc_compiler.version.to_owned();
    let solc_codegen = era_solc::StandardJsonInputCodegen::new(&solc_version, codegen);

    let mut solc_input = era_solc::StandardJsonInput::try_from_solidity_paths(
        paths,
        libraries,
        remappings,
        era_solc::StandardJsonInputOptimizer::default(),
        codegen,
        evm_version,
        enable_eravm_extensions,
        era_solc::StandardJsonInputSelection::new_required(solc_codegen),
        era_solc::StandardJsonInputMetadata::new(use_literal_content, metadata_hash_type),
        llvm_options.clone(),
        suppressed_errors,
        suppressed_warnings,
        false,
        false,
    )?;

    let mut solc_output = solc_compiler.standard_json(
        &mut solc_input,
        messages,
        base_path,
        include_paths,
        allow_paths,
    )?;
    solc_output.take_and_write_warnings();
    solc_output.check_errors()?;

    let linker_symbols = solc_input.settings.libraries.as_linker_symbols()?;

    let project = Project::try_from_solc_output(
        solc_input.settings.libraries,
        solc_codegen,
        &mut solc_output,
        solc_compiler,
        debug_config.as_ref(),
    )?;
    solc_output.take_and_write_warnings();
    solc_output.check_errors()?;

    let mut build = project.compile_to_eravm(
        messages,
        enable_eravm_extensions,
        metadata_hash_type,
        optimizer_settings,
        llvm_options,
        output_assembly,
        debug_config,
    )?;
    build.take_and_write_warnings();
    build.check_errors()?;

    let mut build = build.link(linker_symbols);
    build.take_and_write_warnings();
    build.check_errors()?;
    Ok(build)
}

///
/// Runs the standard output mode for the EVM target.
///
pub fn standard_output_evm(
    paths: &[PathBuf],
    libraries: &[String],
    solc_compiler: &era_solc::Compiler,
    messages: &mut Vec<era_solc::StandardJsonOutputError>,
    codegen: Option<era_solc::StandardJsonInputCodegen>,
    evm_version: Option<era_compiler_common::EVMVersion>,
    metadata_hash_type: era_compiler_common::HashType,
    use_literal_content: bool,
    base_path: Option<String>,
    include_paths: Vec<String>,
    allow_paths: Option<String>,
    remappings: BTreeSet<String>,
    optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
    llvm_options: Vec<String>,
    threads: Option<usize>,
    debug_config: Option<era_compiler_llvm_context::DebugConfig>,
) -> anyhow::Result<EVMBuild> {
    let solc_version = solc_compiler.version.to_owned();
    let solc_codegen = era_solc::StandardJsonInputCodegen::new(&solc_version, codegen);

    let mut solc_input = era_solc::StandardJsonInput::try_from_solidity_paths(
        paths,
        libraries,
        remappings,
        era_solc::StandardJsonInputOptimizer::default(),
        codegen,
        evm_version,
        false,
        era_solc::StandardJsonInputSelection::new_required(solc_codegen),
        era_solc::StandardJsonInputMetadata::new(use_literal_content, metadata_hash_type),
        llvm_options.clone(),
        vec![],
        vec![],
        false,
        false,
    )?;

    let mut solc_output = solc_compiler.standard_json(
        &mut solc_input,
        messages,
        base_path,
        include_paths,
        allow_paths,
    )?;
    solc_output.take_and_write_warnings();
    solc_output.check_errors()?;

    let project = Project::try_from_solc_output(
        solc_input.settings.libraries,
        solc_codegen,
        &mut solc_output,
        solc_compiler,
        debug_config.as_ref(),
    )?;
    solc_output.take_and_write_warnings();
    solc_output.check_errors()?;

    let build = project.compile_to_evm(
        messages,
        metadata_hash_type,
        optimizer_settings,
        llvm_options,
        threads,
        debug_config,
    )?;
    Ok(build)
}

///
/// Runs the standard JSON mode for the EraVM target.
///
pub fn standard_json_eravm(
    solc_compiler: Option<era_solc::Compiler>,
    codegen: Option<era_solc::StandardJsonInputCodegen>,
    enable_eravm_extensions: bool,
    detect_missing_libraries: bool,
    json_path: Option<PathBuf>,
    messages: &mut Vec<era_solc::StandardJsonOutputError>,
    base_path: Option<String>,
    include_paths: Vec<String>,
    allow_paths: Option<String>,
    debug_config: Option<era_compiler_llvm_context::DebugConfig>,
) -> anyhow::Result<()> {
    let mut solc_input = era_solc::StandardJsonInput::try_from(json_path.as_deref())?;
    let language = solc_input.language;
    let prune_output = solc_input.settings.selection_to_prune();
    let linker_symbols = solc_input.settings.libraries.as_linker_symbols()?;

    let mut optimizer_settings = era_compiler_llvm_context::OptimizerSettings::try_from_cli(
        solc_input.settings.optimizer.mode,
    )?;
    if solc_input
        .settings
        .optimizer
        .fallback_to_optimizing_for_size
    {
        optimizer_settings.enable_fallback_to_size();
    }
    let llvm_options = solc_input.settings.llvm_options.clone();

    let codegen = if solc_input.settings.force_evmla {
        Some(era_solc::StandardJsonInputCodegen::EVMLA)
    } else {
        codegen
    };
    let enable_eravm_extensions =
        solc_input.settings.enable_eravm_extensions || enable_eravm_extensions;
    let detect_missing_libraries =
        solc_input.settings.detect_missing_libraries || detect_missing_libraries;
    let metadata_hash_type = solc_input.settings.metadata.hash_type;
    let output_assembly = solc_input
        .settings
        .output_selection
        .contains(&era_solc::StandardJsonInputSelector::EraVMAssembly);

    let (mut solc_output, solc_version, project) = match (language, solc_compiler) {
        (era_solc::StandardJsonInputLanguage::Solidity, solc_compiler) => {
            let solc_compiler = match solc_compiler {
                Some(solc_compiler) => solc_compiler,
                None => era_solc::Compiler::try_from_default()?,
            };

            let solc_codegen =
                era_solc::StandardJsonInputCodegen::new(&solc_compiler.version, codegen);
            solc_input.extend_selection(era_solc::StandardJsonInputSelection::new_required(
                solc_codegen,
            ));

            let mut solc_output = solc_compiler.standard_json(
                &mut solc_input,
                messages,
                base_path,
                include_paths,
                allow_paths,
            )?;
            if solc_output.has_errors() {
                solc_output.write_and_exit(prune_output);
            }

            let project = Project::try_from_solc_output(
                solc_input.settings.libraries,
                solc_codegen,
                &mut solc_output,
                &solc_compiler,
                debug_config.as_ref(),
            )?;
            if solc_output.has_errors() {
                solc_output.write_and_exit(prune_output);
            }

            (solc_output, Some(solc_compiler.version), project)
        }
        (era_solc::StandardJsonInputLanguage::Yul, Some(solc_compiler)) => {
            let mut solc_output =
                solc_compiler.validate_yul_standard_json(&mut solc_input, messages)?;
            if solc_output.has_errors() {
                solc_output.write_and_exit(prune_output);
            }

            let project = Project::try_from_yul_sources(
                solc_input.sources,
                solc_input.settings.libraries,
                Some(&mut solc_output),
                Some(&solc_compiler.version),
                debug_config.as_ref(),
            )?;
            if solc_output.has_errors() {
                solc_output.write_and_exit(prune_output);
            }

            (solc_output, Some(solc_compiler.version), project)
        }
        (era_solc::StandardJsonInputLanguage::Yul, None) => {
            let mut solc_output = era_solc::StandardJsonOutput::new(&solc_input.sources, messages);

            let project = Project::try_from_yul_sources(
                solc_input.sources,
                solc_input.settings.libraries,
                Some(&mut solc_output),
                None,
                debug_config.as_ref(),
            )?;
            if solc_output.has_errors() {
                solc_output.write_and_exit(prune_output);
            }

            (solc_output, None, project)
        }
        (era_solc::StandardJsonInputLanguage::LLVMIR, Some(_)) => {
            anyhow::bail!("LLVM IR projects cannot be compiled with `solc`.")
        }
        (era_solc::StandardJsonInputLanguage::LLVMIR, None) => {
            let mut solc_output = era_solc::StandardJsonOutput::new(&solc_input.sources, messages);

            let project = Project::try_from_llvm_ir_sources(
                solc_input.sources,
                solc_input.settings.libraries,
                Some(&mut solc_output),
            )?;
            if solc_output.has_errors() {
                solc_output.write_and_exit(prune_output);
            }

            (solc_output, None, project)
        }
        (era_solc::StandardJsonInputLanguage::EraVMAssembly, Some(_)) => {
            anyhow::bail!("EraVM assembly projects cannot be compiled with `solc`.")
        }
        (era_solc::StandardJsonInputLanguage::EraVMAssembly, None) => {
            let mut solc_output = era_solc::StandardJsonOutput::new(&solc_input.sources, messages);

            let project = Project::try_from_eravm_assembly_sources(
                solc_input.sources,
                Some(&mut solc_output),
            )?;
            if solc_output.has_errors() {
                solc_output.write_and_exit(prune_output);
            }

            (solc_output, None, project)
        }
    };

    let missing_libraries = project.get_missing_libraries();
    if detect_missing_libraries {
        missing_libraries.write_to_standard_json(&mut solc_output, solc_version.as_ref());
        solc_output.write_and_exit(prune_output);
    }

    let build = project.compile_to_eravm(
        messages,
        enable_eravm_extensions,
        metadata_hash_type,
        optimizer_settings,
        llvm_options,
        output_assembly,
        debug_config,
    )?;
    if build.has_errors() {
        build.write_to_standard_json(&mut solc_output, solc_version.as_ref())?;
        solc_output.write_and_exit(prune_output);
    }

    let build = build.link(linker_symbols);
    build.write_to_standard_json(&mut solc_output, solc_version.as_ref())?;
    missing_libraries.write_to_standard_json(&mut solc_output, solc_version.as_ref());
    solc_output.write_and_exit(prune_output);
}

///
/// Runs the standard JSON mode for the EVM target.
///
pub fn standard_json_evm(
    solc_compiler: Option<era_solc::Compiler>,
    codegen: Option<era_solc::StandardJsonInputCodegen>,
    json_path: Option<PathBuf>,
    messages: &mut Vec<era_solc::StandardJsonOutputError>,
    base_path: Option<String>,
    include_paths: Vec<String>,
    allow_paths: Option<String>,
    threads: Option<usize>,
    debug_config: Option<era_compiler_llvm_context::DebugConfig>,
) -> anyhow::Result<()> {
    let mut solc_input = era_solc::StandardJsonInput::try_from(json_path.as_deref())?;
    let language = solc_input.language;
    let prune_output = solc_input.settings.selection_to_prune();

    let mut optimizer_settings = era_compiler_llvm_context::OptimizerSettings::try_from_cli(
        solc_input.settings.optimizer.mode,
    )?;
    if solc_input
        .settings
        .optimizer
        .fallback_to_optimizing_for_size
    {
        optimizer_settings.enable_fallback_to_size();
    }
    let llvm_options = solc_input.settings.llvm_options.clone();

    let metadata_hash_type = solc_input.settings.metadata.hash_type;

    let (mut solc_output, solc_version, project) = match (language, solc_compiler) {
        (era_solc::StandardJsonInputLanguage::Solidity, solc_compiler) => {
            let solc_compiler = match solc_compiler {
                Some(solc_compiler) => solc_compiler,
                None => era_solc::Compiler::try_from_default()?,
            };

            let solc_codegen =
                era_solc::StandardJsonInputCodegen::new(&solc_compiler.version, codegen);
            solc_input.extend_selection(era_solc::StandardJsonInputSelection::new_required(
                solc_codegen,
            ));

            let mut solc_output = solc_compiler.standard_json(
                &mut solc_input,
                messages,
                base_path,
                include_paths,
                allow_paths,
            )?;
            if solc_output.has_errors() {
                solc_output.write_and_exit(prune_output);
            }

            let project = Project::try_from_solc_output(
                solc_input.settings.libraries,
                solc_codegen,
                &mut solc_output,
                &solc_compiler,
                debug_config.as_ref(),
            )?;
            if solc_output.has_errors() {
                solc_output.write_and_exit(prune_output);
            }

            (solc_output, Some(solc_compiler.version), project)
        }
        (era_solc::StandardJsonInputLanguage::Yul, Some(solc_compiler)) => {
            let mut solc_output =
                solc_compiler.validate_yul_standard_json(&mut solc_input, messages)?;
            if solc_output.has_errors() {
                solc_output.write_and_exit(prune_output);
            }

            let project = Project::try_from_yul_sources(
                solc_input.sources,
                solc_input.settings.libraries,
                Some(&mut solc_output),
                Some(&solc_compiler.version),
                debug_config.as_ref(),
            )?;
            if solc_output.has_errors() {
                solc_output.write_and_exit(prune_output);
            }

            (solc_output, Some(solc_compiler.version), project)
        }
        (era_solc::StandardJsonInputLanguage::Yul, None) => {
            let mut solc_output = era_solc::StandardJsonOutput::new(&solc_input.sources, messages);

            let project = Project::try_from_yul_sources(
                solc_input.sources,
                solc_input.settings.libraries,
                Some(&mut solc_output),
                None,
                debug_config.as_ref(),
            )?;
            if solc_output.has_errors() {
                solc_output.write_and_exit(prune_output);
            }

            (solc_output, None, project)
        }
        (era_solc::StandardJsonInputLanguage::LLVMIR, Some(_)) => {
            anyhow::bail!("LLVM IR projects cannot be compiled with `solc`.")
        }
        (era_solc::StandardJsonInputLanguage::LLVMIR, None) => {
            let mut solc_output = era_solc::StandardJsonOutput::new(&solc_input.sources, messages);

            let project = Project::try_from_llvm_ir_sources(
                solc_input.sources,
                solc_input.settings.libraries,
                Some(&mut solc_output),
            )?;
            if solc_output.has_errors() {
                solc_output.write_and_exit(prune_output);
            }

            (solc_output, None, project)
        }
        (era_solc::StandardJsonInputLanguage::EraVMAssembly, Some(_)) => {
            anyhow::bail!("EraVM assembly projects cannot be compiled with `solc`.")
        }
        (era_solc::StandardJsonInputLanguage::EraVMAssembly, None) => {
            let mut solc_output = era_solc::StandardJsonOutput::new(&solc_input.sources, messages);

            let project = Project::try_from_eravm_assembly_sources(
                solc_input.sources,
                Some(&mut solc_output),
            )?;
            if solc_output.has_errors() {
                solc_output.write_and_exit(prune_output);
            }

            (solc_output, None, project)
        }
    };

    let build = project.compile_to_evm(
        messages,
        metadata_hash_type,
        optimizer_settings,
        llvm_options,
        threads,
        debug_config,
    )?;
    build.write_to_standard_json(&mut solc_output, solc_version.as_ref())?;
    solc_output.write_and_exit(prune_output);
}

///
/// Runs the combined JSON mode for the EraVM target.
///
pub fn combined_json_eravm(
    format: String,
    paths: &[PathBuf],
    libraries: &[String],
    solc_compiler: &era_solc::Compiler,
    messages: &mut Vec<era_solc::StandardJsonOutputError>,
    codegen: Option<era_solc::StandardJsonInputCodegen>,
    evm_version: Option<era_compiler_common::EVMVersion>,
    enable_eravm_extensions: bool,
    metadata_hash_type: era_compiler_common::HashType,
    use_literal_content: bool,
    base_path: Option<String>,
    include_paths: Vec<String>,
    allow_paths: Option<String>,
    remappings: BTreeSet<String>,
    output_directory: Option<PathBuf>,
    overwrite: bool,
    optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
    llvm_options: Vec<String>,
    suppressed_errors: Vec<era_solc::StandardJsonInputErrorType>,
    suppressed_warnings: Vec<era_solc::StandardJsonInputWarningType>,
    debug_config: Option<era_compiler_llvm_context::DebugConfig>,
) -> anyhow::Result<()> {
    let selector_results = era_solc::CombinedJsonSelector::from_cli(format.as_str());
    let mut selectors = HashSet::with_capacity(selector_results.len());
    for result in selector_results.into_iter() {
        match result {
            Ok(selector) => {
                selectors.insert(selector);
            }
            Err(selector) => {
                messages.push(era_solc::StandardJsonOutputError::new_warning(
                    format!("The selector `{selector}` is not supported, and therefore ignored."),
                    None,
                    None,
                ));
            }
        }
    }
    if !selectors.contains(&era_solc::CombinedJsonSelector::Bytecode) {
        messages.push(era_solc::StandardJsonOutputError::new_warning(
            format!("The `{}` selector will become mandatory in future releases of `zksolc`. For now, bytecode is always emitted even if the selector is not provided.", era_solc::CombinedJsonSelector::Bytecode),
            None,
            None,
        ));
    }
    if selectors.contains(&era_solc::CombinedJsonSelector::BytecodeRuntime) {
        messages.push(era_solc::StandardJsonOutputError::new_warning(
            format!("The `{}` selector does not make sense for the {} target, since there is only one bytecode segment. The eponymous output field will be removed in future releases of `zksolc`.", era_solc::CombinedJsonSelector::BytecodeRuntime, era_compiler_common::Target::EraVM),
            None,
            None,
        ));
    }

    let build = standard_output_eravm(
        paths,
        libraries,
        solc_compiler,
        messages,
        codegen,
        evm_version,
        enable_eravm_extensions,
        metadata_hash_type,
        use_literal_content,
        base_path,
        include_paths,
        allow_paths,
        remappings,
        optimizer_settings,
        llvm_options,
        selectors.contains(&era_solc::CombinedJsonSelector::EraVMAssembly),
        suppressed_errors,
        suppressed_warnings,
        debug_config,
    )?;

    let mut combined_json = solc_compiler.combined_json(paths, selectors)?;
    build.write_to_combined_json(&mut combined_json)?;

    match output_directory {
        Some(output_directory) => {
            std::fs::create_dir_all(output_directory.as_path())?;
            combined_json.write_to_directory(output_directory.as_path(), overwrite)?;

            writeln!(
                std::io::stderr(),
                "Compiler run successful. Artifact(s) can be found in directory {output_directory:?}."
            )?;
        }
        None => {
            serde_json::to_writer(std::io::stdout(), &combined_json)?;
        }
    }
    std::process::exit(era_compiler_common::EXIT_CODE_SUCCESS);
}

///
/// Runs the combined JSON mode for the EVM target.
///
pub fn combined_json_evm(
    format: String,
    paths: &[PathBuf],
    libraries: &[String],
    solc_compiler: &era_solc::Compiler,
    messages: &mut Vec<era_solc::StandardJsonOutputError>,
    codegen: Option<era_solc::StandardJsonInputCodegen>,
    evm_version: Option<era_compiler_common::EVMVersion>,
    metadata_hash_type: era_compiler_common::HashType,
    use_literal_content: bool,
    base_path: Option<String>,
    include_paths: Vec<String>,
    allow_paths: Option<String>,
    remappings: BTreeSet<String>,
    output_directory: Option<PathBuf>,
    overwrite: bool,
    optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
    llvm_options: Vec<String>,
    threads: Option<usize>,
    debug_config: Option<era_compiler_llvm_context::DebugConfig>,
) -> anyhow::Result<()> {
    let selector_results = era_solc::CombinedJsonSelector::from_cli(format.as_str());
    let mut selectors = HashSet::with_capacity(selector_results.len());
    for result in selector_results.into_iter() {
        match result {
            Ok(selector) => {
                selectors.insert(selector);
            }
            Err(selector) => {
                messages.push(era_solc::StandardJsonOutputError::new_warning(
                    format!("The selector `{selector}` is not supported, and therefore ignored."),
                    None,
                    None,
                ));
            }
        }
    }

    let build = standard_output_evm(
        paths,
        libraries,
        solc_compiler,
        messages,
        codegen,
        evm_version,
        metadata_hash_type,
        use_literal_content,
        base_path,
        include_paths,
        allow_paths,
        remappings,
        optimizer_settings,
        llvm_options,
        threads,
        debug_config,
    )?;

    let mut combined_json = solc_compiler.combined_json(paths, selectors)?;
    build.write_to_combined_json(&mut combined_json)?;

    match output_directory {
        Some(output_directory) => {
            std::fs::create_dir_all(output_directory.as_path())?;
            combined_json.write_to_directory(output_directory.as_path(), overwrite)?;

            writeln!(
                std::io::stderr(),
                "Compiler run successful. Artifact(s) can be found in directory {output_directory:?}."
            )?;
        }
        None => {
            serde_json::to_writer(std::io::stdout(), &combined_json)?;
        }
    }
    std::process::exit(era_compiler_common::EXIT_CODE_SUCCESS);
}

///
/// Runs the disassembler for EraVM bytecode file and prints the output to stdout.
///
pub fn disassemble_eravm(paths: Vec<String>) -> anyhow::Result<()> {
    let bytecodes = paths
        .into_par_iter()
        .map(|path| {
            let pathbuf = PathBuf::from(path.as_str());
            let string = std::fs::read_to_string(pathbuf)?;
            let hexadecimal_string = string.trim().strip_prefix("0x").unwrap_or(string.as_str());
            let bytecode = hex::decode(hexadecimal_string)?;
            Ok((path, bytecode))
        })
        .collect::<anyhow::Result<BTreeMap<String, Vec<u8>>>>()?;

    let target_machine = era_compiler_llvm_context::TargetMachine::new(
        era_compiler_common::Target::EraVM,
        &era_compiler_llvm_context::OptimizerSettings::cycles(),
        &[],
    )?;

    let disassemblies: Vec<(String, String)> = bytecodes
        .into_iter()
        .map(|(path, bytecode)| {
            let bytecode_buffer = inkwell::memory_buffer::MemoryBuffer::create_from_memory_range(
                bytecode.as_slice(),
                path.as_str(),
                false,
            );
            let disassembly =
                era_compiler_llvm_context::eravm_disassemble(&target_machine, &bytecode_buffer)?;
            Ok((path, disassembly))
        })
        .collect::<anyhow::Result<Vec<(String, String)>>>()?;

    for (path, disassembly) in disassemblies.into_iter() {
        writeln!(std::io::stderr(), "File `{path}` disassembly:\n\n")?;
        writeln!(std::io::stdout(), "{disassembly}")?;
        writeln!(std::io::stderr(), "\n\n")?;
    }
    std::process::exit(era_compiler_common::EXIT_CODE_SUCCESS);
}

///
/// Links EraVM bytecode files.
///
pub fn link_eravm(paths: Vec<String>, libraries: Vec<String>) -> anyhow::Result<()> {
    let bytecodes = paths
        .into_par_iter()
        .map(|path| {
            let bytecode = std::fs::read_to_string(path.as_str())?;
            Ok((path, bytecode))
        })
        .collect::<anyhow::Result<BTreeMap<String, String>>>()?;

    let input = LinkerInput::new(bytecodes, libraries);
    let output = Linker::link_eravm(input)?;

    output
        .linked
        .par_iter()
        .map(|(path, contract)| {
            std::fs::write(path, contract.bytecode.as_bytes())?;
            Ok(())
        })
        .collect::<anyhow::Result<()>>()?;

    serde_json::to_writer(std::io::stdout(), &output)?;
    std::process::exit(era_compiler_common::EXIT_CODE_SUCCESS);
}

///
/// Links EraVM bytecode files received as JSON input.
///
pub fn link_eravm_json(path: Option<String>) -> anyhow::Result<()> {
    let input_json = match path.map(PathBuf::from) {
        Some(path) => std::fs::read_to_string(path.as_path())
            .map_err(|error| anyhow::anyhow!("JSON file {path:?} reading: {error}")),
        None => std::io::read_to_string(std::io::stdin())
            .map_err(|error| anyhow::anyhow!("JSON stdin reading: {error}")),
    }?;

    let input = era_compiler_common::deserialize_from_str::<LinkerInput>(input_json.as_str())
        .map_err(|error| anyhow::anyhow!("JSON parsing: {error}"))?;
    let output = Linker::link_eravm(input)?;

    serde_json::to_writer(std::io::stdout(), &output)?;
    std::process::exit(era_compiler_common::EXIT_CODE_SUCCESS);
}

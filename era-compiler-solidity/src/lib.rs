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
pub mod missing_libraries;
pub mod process;
pub mod project;
pub mod yul;

pub use self::build_eravm::contract::Contract as EraVMContractBuild;
pub use self::build_eravm::Build as EraVMBuild;
pub use self::build_evm::contract::Contract as EVMContractBuild;
pub use self::build_evm::Build as EVMBuild;
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
use std::io::Write;
use std::path::PathBuf;

use rayon::iter::IntoParallelIterator;
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
    threads: Option<usize>,
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

    let build = project.compile_to_eravm(
        messages,
        enable_eravm_extensions,
        linker_symbols,
        metadata_hash_type,
        optimizer_settings,
        llvm_options,
        output_assembly,
        threads,
        debug_config,
    )?;
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
        optimizer_settings,
        llvm_options,
        metadata_hash_type,
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
    threads: Option<usize>,
    debug_config: Option<era_compiler_llvm_context::DebugConfig>,
) -> anyhow::Result<EraVMBuild> {
    let libraries = era_solc::StandardJsonInputLibraries::try_from(libraries)?;
    let linker_symbols = libraries.as_linker_symbols()?;

    let project = Project::try_from_llvm_ir_paths(paths, libraries, None)?;

    let build = project.compile_to_eravm(
        messages,
        false,
        linker_symbols,
        metadata_hash_type,
        optimizer_settings,
        llvm_options,
        output_assembly,
        threads,
        debug_config,
    )?;
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
        optimizer_settings,
        llvm_options,
        metadata_hash_type,
        threads,
        debug_config,
    )?;
    Ok(build)
}

///
/// Runs the EraVM assembly mode.
///
pub fn eravm_assembly(
    paths: &[PathBuf],
    messages: &mut Vec<era_solc::StandardJsonOutputError>,
    metadata_hash_type: era_compiler_common::HashType,
    llvm_options: Vec<String>,
    output_assembly: bool,
    threads: Option<usize>,
    debug_config: Option<era_compiler_llvm_context::DebugConfig>,
) -> anyhow::Result<EraVMBuild> {
    let project = Project::try_from_eravm_assembly_paths(paths, None)?;

    let optimizer_settings = era_compiler_llvm_context::OptimizerSettings::none();
    let build = project.compile_to_eravm(
        messages,
        false,
        BTreeMap::new(),
        metadata_hash_type,
        optimizer_settings,
        llvm_options,
        output_assembly,
        threads,
        debug_config,
    )?;
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
    threads: Option<usize>,
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
        Some(solc_codegen),
        messages,
        base_path,
        include_paths,
        allow_paths,
    )?;
    solc_output.take_and_write_warnings();
    solc_output.collect_errors()?;

    let linker_symbols = solc_input.settings.libraries.as_linker_symbols()?;

    let project = Project::try_from_solc_output(
        solc_input.settings.libraries,
        solc_codegen,
        &mut solc_output,
        solc_compiler,
        debug_config.as_ref(),
    )?;
    solc_output.take_and_write_warnings();
    solc_output.collect_errors()?;

    let build = project.compile_to_eravm(
        messages,
        enable_eravm_extensions,
        linker_symbols,
        metadata_hash_type,
        optimizer_settings,
        llvm_options,
        output_assembly,
        threads,
        debug_config,
    )?;
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
        Some(solc_codegen),
        messages,
        base_path,
        include_paths,
        allow_paths,
    )?;
    solc_output.take_and_write_warnings();
    solc_output.collect_errors()?;

    let project = Project::try_from_solc_output(
        solc_input.settings.libraries,
        solc_codegen,
        &mut solc_output,
        solc_compiler,
        debug_config.as_ref(),
    )?;
    solc_output.take_and_write_warnings();
    solc_output.collect_errors()?;

    let build = project.compile_to_evm(
        messages,
        optimizer_settings,
        llvm_options,
        metadata_hash_type,
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
    threads: Option<usize>,
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
        .contains(&era_solc::StandardJsonInputSelectionFlag::EraVMAssembly);

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
                Some(solc_codegen),
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
            anyhow::bail!("LLVM IR projects cannot be compiled with `solc`")
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
            anyhow::bail!("EraVM assembly projects cannot be compiled with `solc`")
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

    if detect_missing_libraries {
        let missing_libraries = project.get_missing_libraries();
        missing_libraries.write_to_standard_json(&mut solc_output, solc_version.as_ref());
    } else {
        let build = project.compile_to_eravm(
            messages,
            enable_eravm_extensions,
            linker_symbols,
            metadata_hash_type,
            optimizer_settings,
            llvm_options,
            output_assembly,
            threads,
            debug_config,
        )?;
        build.write_to_standard_json(&mut solc_output, solc_version.as_ref())?;
    }
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
                Some(solc_codegen),
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
            anyhow::bail!("LLVM IR projects cannot be compiled with `solc`")
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
        (era_solc::StandardJsonInputLanguage::EraVMAssembly, _) => {
            anyhow::bail!("Compiling EraVM assembly to EVM is not supported")
        }
    };

    let build = project.compile_to_evm(
        messages,
        optimizer_settings,
        llvm_options,
        metadata_hash_type,
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
    output_assembly: bool,
    suppressed_errors: Vec<era_solc::StandardJsonInputErrorType>,
    suppressed_warnings: Vec<era_solc::StandardJsonInputWarningType>,
    threads: Option<usize>,
    debug_config: Option<era_compiler_llvm_context::DebugConfig>,
) -> anyhow::Result<()> {
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
        output_assembly,
        suppressed_errors,
        suppressed_warnings,
        threads,
        debug_config,
    )?;

    let mut combined_json = solc_compiler.combined_json(paths, format.as_str())?;
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

    let mut combined_json = solc_compiler.combined_json(paths, format.as_str())?;
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
            let bytecode = match pathbuf.extension().and_then(|extension| extension.to_str()) {
                Some("hex") => {
                    let string = std::fs::read_to_string(pathbuf)?;
                    let hexadecimal_string =
                        string.trim().strip_prefix("0x").unwrap_or(string.as_str());
                    hex::decode(hexadecimal_string)?
                }
                Some("zbin") => std::fs::read(pathbuf)?,
                Some(extension) => anyhow::bail!(
                    "Invalid file extension: {extension}. Supported extensions: *.hex, *.zbin"
                ),
                None => {
                    anyhow::bail!("Missing file extension. Supported extensions: *.hex, *.zbin")
                }
            };
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
            let disassembly =
                era_compiler_llvm_context::eravm_disassemble(&target_machine, bytecode.as_slice())?;
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
/// Runs the linker for EraVM bytecode file, modifying it in place.
///
pub fn link_eravm(paths: Vec<String>, libraries: &[String]) -> anyhow::Result<()> {
    let bytecodes = paths
        .into_par_iter()
        .map(|path| {
            let bytecode_string = std::fs::read_to_string(path.as_str())?;
            let bytecode = hex::decode(
                bytecode_string
                    .strip_prefix("0x")
                    .unwrap_or(bytecode_string.as_str()),
            )?;
            Ok((path, bytecode))
        })
        .collect::<anyhow::Result<BTreeMap<String, Vec<u8>>>>()?;

    let linker_symbols =
        era_solc::StandardJsonInputLibraries::try_from(libraries)?.as_linker_symbols()?;
    let mut linked_objects = serde_json::Map::new();
    let mut unlinked_objects = serde_json::Map::new();
    let mut ignored_objects = serde_json::Map::new();

    bytecodes
        .into_iter()
        .try_for_each(|(path, bytecode)| -> anyhow::Result<()> {
            let memory_buffer = inkwell::memory_buffer::MemoryBuffer::create_from_memory_range(
                bytecode.as_slice(),
                "bytecode",
                false,
            );
            let already_linked = !memory_buffer.is_elf_eravm();

            let (memory_buffer_linked, bytecode_hash) =
                era_compiler_llvm_context::eravm_link(memory_buffer, &linker_symbols)?;

            if let Some(bytecode_hash) = bytecode_hash {
                if already_linked {
                    ignored_objects.insert(
                        path.clone(),
                        serde_json::Value::String(hex::encode(bytecode_hash)),
                    );
                } else {
                    linked_objects.insert(
                        path.clone(),
                        serde_json::Value::String(hex::encode(bytecode_hash)),
                    );
                }
            }
            if memory_buffer_linked.is_elf_eravm() {
                unlinked_objects.insert(
                    path.clone(),
                    serde_json::Value::Array(
                        memory_buffer_linked
                            .get_undefined_symbols_eravm()
                            .iter()
                            .map(|symbol| serde_json::Value::String(symbol.to_string()))
                            .collect(),
                    ),
                );
            }

            std::fs::write(path, hex::encode(memory_buffer_linked.as_slice()))?;

            Ok(())
        })?;

    serde_json::to_writer(
        std::io::stdout(),
        &serde_json::json!({
            "linked": linked_objects,
            "unlinked": unlinked_objects,
            "ignored": ignored_objects,
        }),
    )?;
    std::process::exit(era_compiler_common::EXIT_CODE_SUCCESS);
}

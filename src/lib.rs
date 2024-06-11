//!
//! Solidity to EraVM compiler library.
//!

#![allow(non_camel_case_types)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::enum_variant_names)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::should_implement_trait)]

pub(crate) mod build_eravm;
pub(crate) mod build_evm;
pub(crate) mod r#const;
pub(crate) mod evmla;
pub(crate) mod missing_libraries;
pub(crate) mod process;
pub(crate) mod project;
pub(crate) mod solc;
pub(crate) mod warning;
pub(crate) mod yul;

pub use self::build_eravm::contract::Contract as EraVMContractBuild;
pub use self::build_eravm::Build as EraVMBuild;
pub use self::build_evm::contract::Contract as EVMContractBuild;
pub use self::build_evm::Build as EVMBuild;
pub use self::process::input_eravm::Input as EraVMProcessInput;
pub use self::process::input_evm::Input as EVMProcessInput;
pub use self::process::output_eravm::Output as EraVMProcessOutput;
pub use self::process::output_evm::Output as EVMProcessOutput;
pub use self::process::run as run_process;
pub use self::process::EXECUTABLE;
pub use self::project::contract::Contract as ProjectContract;
pub use self::project::Project;
pub use self::r#const::*;
pub use self::solc::combined_json::contract::Contract as SolcCombinedJsonContract;
pub use self::solc::combined_json::CombinedJson as SolcCombinedJson;
pub use self::solc::pipeline::Pipeline as SolcPipeline;
pub use self::solc::standard_json::input::language::Language as SolcStandardJsonInputLanguage;
pub use self::solc::standard_json::input::settings::metadata::Metadata as SolcStandardJsonInputSettingsMetadata;
pub use self::solc::standard_json::input::settings::optimizer::Optimizer as SolcStandardJsonInputSettingsOptimizer;
pub use self::solc::standard_json::input::settings::selection::file::flag::Flag as SolcStandardJsonInputSettingsSelectionFileFlag;
pub use self::solc::standard_json::input::settings::selection::file::File as SolcStandardJsonInputSettingsSelectionFile;
pub use self::solc::standard_json::input::settings::selection::Selection as SolcStandardJsonInputSettingsSelection;
pub use self::solc::standard_json::input::settings::Settings as SolcStandardJsonInputSettings;
pub use self::solc::standard_json::input::source::Source as SolcStandardJsonInputSource;
pub use self::solc::standard_json::input::Input as SolcStandardJsonInput;
pub use self::solc::standard_json::output::contract::evm::bytecode::Bytecode as SolcStandardJsonOutputContractEVMBytecode;
pub use self::solc::standard_json::output::contract::evm::EVM as SolcStandardJsonOutputContractEVM;
pub use self::solc::standard_json::output::contract::Contract as SolcStandardJsonOutputContract;
pub use self::solc::standard_json::output::Output as SolcStandardJsonOutput;
pub use self::solc::version::Version as SolcVersion;
pub use self::solc::Compiler as SolcCompiler;
pub use self::warning::Warning;

mod tests;

use std::collections::BTreeSet;
use std::path::PathBuf;

///
/// Runs the Yul mode for the EraVM target.
///
pub fn yul_to_eravm(
    paths: &[PathBuf],
    libraries: Vec<String>,
    solc_path: Option<String>,
    optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
    llvm_options: Vec<String>,
    enable_eravm_extensions: bool,
    include_metadata_hash: bool,
    debug_config: Option<era_compiler_llvm_context::DebugConfig>,
) -> anyhow::Result<EraVMBuild> {
    let libraries = SolcStandardJsonInputSettings::parse_libraries(libraries)?;

    let solc_version = match solc_path {
        Some(solc_path) => {
            if enable_eravm_extensions {
                anyhow::bail!("Yul validation cannot be done if EraVM extensions are enabled. Consider compiling without `solc`.")
            }
            let solc_compiler = SolcCompiler::new(solc_path.as_str())?;
            solc_compiler.validate_yul_paths(paths, libraries.clone())?;
            Some(solc_compiler.version)
        }
        None => None,
    };

    let project = Project::try_from_yul_paths(
        paths,
        libraries,
        solc_version.as_ref(),
        debug_config.as_ref(),
    )?;

    let build = project.compile_to_eravm(
        optimizer_settings,
        llvm_options,
        enable_eravm_extensions,
        include_metadata_hash,
        zkevm_assembly::RunningVmEncodingMode::Production,
        debug_config,
    )?;
    build.check_errors()?;
    Ok(build)
}

///
/// Runs the Yul mode for the EVM target.
///
pub fn yul_to_evm(
    paths: &[PathBuf],
    libraries: Vec<String>,
    solc_path: Option<String>,
    optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
    llvm_options: Vec<String>,
    include_metadata_hash: bool,
    debug_config: Option<era_compiler_llvm_context::DebugConfig>,
) -> anyhow::Result<EVMBuild> {
    let libraries = SolcStandardJsonInputSettings::parse_libraries(libraries)?;

    let solc_version = match solc_path {
        Some(solc_path) => {
            let solc_compiler = SolcCompiler::new(solc_path.as_str())?;
            solc_compiler.validate_yul_paths(paths, libraries.clone())?;
            Some(solc_compiler.version)
        }
        None => None,
    };

    let project = Project::try_from_yul_paths(
        paths,
        libraries,
        solc_version.as_ref(),
        debug_config.as_ref(),
    )?;

    let build = project.compile_to_evm(
        optimizer_settings,
        llvm_options,
        include_metadata_hash,
        debug_config,
    )?;
    build.check_errors()?;
    Ok(build)
}

///
/// Runs the LLVM IR mode for the EraVM target.
///
pub fn llvm_ir_to_eravm(
    paths: &[PathBuf],
    optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
    llvm_options: Vec<String>,
    include_metadata_hash: bool,
    debug_config: Option<era_compiler_llvm_context::DebugConfig>,
) -> anyhow::Result<EraVMBuild> {
    let project = Project::try_from_llvm_ir_paths(paths)?;

    let build = project.compile_to_eravm(
        optimizer_settings,
        llvm_options,
        false,
        include_metadata_hash,
        zkevm_assembly::RunningVmEncodingMode::Production,
        debug_config,
    )?;
    build.check_errors()?;
    Ok(build)
}

///
/// Runs the LLVM IR mode for the EVM target.
///
pub fn llvm_ir_to_evm(
    paths: &[PathBuf],
    optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
    llvm_options: Vec<String>,
    include_metadata_hash: bool,
    debug_config: Option<era_compiler_llvm_context::DebugConfig>,
) -> anyhow::Result<EVMBuild> {
    let project = Project::try_from_llvm_ir_paths(paths)?;

    let build = project.compile_to_evm(
        optimizer_settings,
        llvm_options,
        include_metadata_hash,
        debug_config,
    )?;
    build.check_errors()?;
    Ok(build)
}

///
/// Runs the EraVM assembly mode.
///
pub fn eravm_assembly(
    paths: &[PathBuf],
    llvm_options: Vec<String>,
    include_metadata_hash: bool,
    debug_config: Option<era_compiler_llvm_context::DebugConfig>,
) -> anyhow::Result<EraVMBuild> {
    let project = Project::try_from_eravm_assembly_paths(paths)?;

    let optimizer_settings = era_compiler_llvm_context::OptimizerSettings::none();
    let build = project.compile_to_eravm(
        optimizer_settings,
        llvm_options,
        false,
        include_metadata_hash,
        zkevm_assembly::RunningVmEncodingMode::Production,
        debug_config,
    )?;
    build.check_errors()?;
    Ok(build)
}

///
/// Runs the standard output mode for EraVM.
///
pub fn standard_output_eravm(
    paths: &[PathBuf],
    libraries: Vec<String>,
    solc_compiler: &SolcCompiler,
    evm_version: Option<era_compiler_common::EVMVersion>,
    solc_optimizer_enabled: bool,
    force_evmla: bool,
    enable_eravm_extensions: bool,
    include_metadata_hash: bool,
    use_literal_content: bool,
    base_path: Option<String>,
    include_paths: Vec<String>,
    allow_paths: Option<String>,
    remappings: Option<BTreeSet<String>>,
    optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
    llvm_options: Vec<String>,
    suppressed_warnings: Option<Vec<Warning>>,
    debug_config: Option<era_compiler_llvm_context::DebugConfig>,
) -> anyhow::Result<EraVMBuild> {
    let solc_version = solc_compiler.version.to_owned();
    let solc_pipeline = SolcPipeline::new(&solc_version, force_evmla);

    let solc_input = SolcStandardJsonInput::try_from_solidity_paths(
        SolcStandardJsonInputLanguage::Solidity,
        evm_version,
        paths,
        libraries,
        remappings,
        SolcStandardJsonInputSettingsSelection::new_required(Some(solc_pipeline)),
        SolcStandardJsonInputSettingsOptimizer::new(
            solc_optimizer_enabled,
            None,
            &solc_version.default,
            optimizer_settings.is_fallback_to_size_enabled(),
        ),
        Some(SolcStandardJsonInputSettingsMetadata::new(
            era_compiler_llvm_context::EraVMMetadataHash::None,
            use_literal_content,
        )),
        force_evmla,
        false,
        enable_eravm_extensions,
        false,
        llvm_options.clone(),
        suppressed_warnings,
    )?;
    let sources = solc_input.sources()?;
    let libraries = solc_input.settings.libraries.clone().unwrap_or_default();
    let mut solc_output = solc_compiler.standard_json(
        solc_input,
        Some(solc_pipeline),
        base_path,
        include_paths,
        allow_paths,
    )?;
    solc_output.check_errors()?;

    let project = Project::try_from_solidity_sources(
        &mut solc_output,
        sources,
        libraries,
        solc_pipeline,
        solc_compiler,
        debug_config.as_ref(),
    )?;

    let build = project.compile_to_eravm(
        optimizer_settings,
        llvm_options,
        enable_eravm_extensions,
        include_metadata_hash,
        zkevm_assembly::RunningVmEncodingMode::Production,
        debug_config,
    )?;

    Ok(build)
}

///
/// Runs the standard output mode for EVM.
///
pub fn standard_output_evm(
    paths: &[PathBuf],
    libraries: Vec<String>,
    solc_compiler: &SolcCompiler,
    evm_version: Option<era_compiler_common::EVMVersion>,
    solc_optimizer_enabled: bool,
    force_evmla: bool,
    include_metadata_hash: bool,
    use_literal_content: bool,
    base_path: Option<String>,
    include_paths: Vec<String>,
    allow_paths: Option<String>,
    remappings: Option<BTreeSet<String>>,
    optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
    llvm_options: Vec<String>,
    debug_config: Option<era_compiler_llvm_context::DebugConfig>,
) -> anyhow::Result<EVMBuild> {
    let solc_version = solc_compiler.version.to_owned();
    let solc_pipeline = SolcPipeline::new(&solc_version, force_evmla);

    let solc_input = SolcStandardJsonInput::try_from_solidity_paths(
        SolcStandardJsonInputLanguage::Solidity,
        evm_version,
        paths,
        libraries,
        remappings,
        SolcStandardJsonInputSettingsSelection::new_required(Some(solc_pipeline)),
        SolcStandardJsonInputSettingsOptimizer::new(
            solc_optimizer_enabled,
            None,
            &solc_version.default,
            optimizer_settings.is_fallback_to_size_enabled(),
        ),
        Some(SolcStandardJsonInputSettingsMetadata::new(
            era_compiler_llvm_context::EraVMMetadataHash::None,
            use_literal_content,
        )),
        force_evmla,
        false,
        false,
        false,
        llvm_options.clone(),
        None,
    )?;
    let sources = solc_input.sources()?;
    let libraries = solc_input.settings.libraries.clone().unwrap_or_default();
    let mut solc_output = solc_compiler.standard_json(
        solc_input,
        Some(solc_pipeline),
        base_path,
        include_paths,
        allow_paths,
    )?;
    solc_output.check_errors()?;

    let project = Project::try_from_solidity_sources(
        &mut solc_output,
        sources,
        libraries,
        solc_pipeline,
        solc_compiler,
        debug_config.as_ref(),
    )?;

    let build = project.compile_to_evm(
        optimizer_settings,
        llvm_options,
        include_metadata_hash,
        debug_config,
    )?;

    Ok(build)
}

///
/// Runs the standard JSON mode for EVM.
///
pub fn standard_json_eravm(
    solc_compiler: Option<&SolcCompiler>,
    force_evmla: bool,
    enable_eravm_extensions: bool,
    detect_missing_libraries: bool,
    json_path: Option<PathBuf>,
    base_path: Option<String>,
    include_paths: Vec<String>,
    allow_paths: Option<String>,
    debug_config: Option<era_compiler_llvm_context::DebugConfig>,
) -> anyhow::Result<()> {
    let zksolc_version = semver::Version::parse(env!("CARGO_PKG_VERSION")).expect("Always valid");

    let mut solc_input = SolcStandardJsonInput::try_from(json_path.as_deref())?;
    let language = solc_input.language;
    let sources = solc_input.sources()?;
    let libraries = solc_input.settings.libraries.clone().unwrap_or_default();

    let optimizer_settings =
        era_compiler_llvm_context::OptimizerSettings::try_from(&solc_input.settings.optimizer)?;
    let llvm_options = solc_input.settings.llvm_options.take().unwrap_or_default();

    let force_evmla = solc_input.settings.force_evmla.unwrap_or_default() || force_evmla;
    let enable_eravm_extensions = solc_input
        .settings
        .enable_eravm_extensions
        .unwrap_or_default()
        || enable_eravm_extensions;
    let detect_missing_libraries = solc_input
        .settings
        .detect_missing_libraries
        .unwrap_or_default()
        || detect_missing_libraries;
    let include_metadata_hash = match solc_input.settings.metadata {
        Some(ref metadata) => {
            metadata.bytecode_hash != Some(era_compiler_llvm_context::EraVMMetadataHash::None)
        }
        None => true,
    };

    let (mut solc_output, solc_version, project) = match (language, solc_compiler) {
        (SolcStandardJsonInputLanguage::Solidity, Some(solc_compiler)) => {
            let solc_pipeline = SolcPipeline::new(&solc_compiler.version, force_evmla);
            solc_input.normalize(&solc_compiler.version.default, Some(solc_pipeline));

            let mut solc_output = solc_compiler.standard_json(
                solc_input,
                Some(solc_pipeline),
                base_path,
                include_paths,
                allow_paths,
            )?;
            if solc_output.check_errors().is_err() {
                serde_json::to_writer(std::io::stdout(), &solc_output)?;
                std::process::exit(era_compiler_common::EXIT_CODE_SUCCESS);
            }

            let project = Project::try_from_solidity_sources(
                &mut solc_output,
                sources,
                libraries,
                solc_pipeline,
                solc_compiler,
                debug_config.as_ref(),
            )?;

            (solc_output, Some(&solc_compiler.version), project)
        }
        (SolcStandardJsonInputLanguage::Solidity, None) => {
            anyhow::bail!("Compiling Solidity without `solc` is not supported")
        }
        (SolcStandardJsonInputLanguage::Yul, Some(solc_compiler)) => {
            let solc_output = solc_compiler.validate_yul_standard_json(solc_input)?;

            let project = Project::try_from_yul_sources(
                sources,
                libraries,
                Some(&solc_compiler.version),
                debug_config.as_ref(),
            )?;

            (solc_output, Some(&solc_compiler.version), project)
        }
        (SolcStandardJsonInputLanguage::Yul, None) => {
            let solc_output = SolcStandardJsonOutput::new(&sources);
            let project =
                Project::try_from_yul_sources(sources, libraries, None, debug_config.as_ref())?;
            (solc_output, None, project)
        }
        (SolcStandardJsonInputLanguage::LLVMIR, Some(_)) => {
            anyhow::bail!("LLVM IR projects cannot be compiled with `solc`")
        }
        (SolcStandardJsonInputLanguage::LLVMIR, None) => {
            let solc_output = SolcStandardJsonOutput::new(&sources);
            let project = Project::try_from_llvm_ir_sources(sources)?;
            (solc_output, None, project)
        }
        (SolcStandardJsonInputLanguage::EraVMAssembly, Some(_)) => {
            anyhow::bail!("EraVM assembly projects cannot be compiled with `solc`")
        }
        (SolcStandardJsonInputLanguage::EraVMAssembly, None) => {
            let solc_output = SolcStandardJsonOutput::new(&sources);
            let project = Project::try_from_eravm_assembly_sources(sources)?;
            (solc_output, None, project)
        }
    };

    if detect_missing_libraries {
        let missing_libraries = project.get_missing_libraries();
        missing_libraries.write_to_standard_json(
            &mut solc_output,
            solc_version,
            &zksolc_version,
        )?;
    } else {
        let build = project.compile_to_eravm(
            optimizer_settings,
            llvm_options,
            enable_eravm_extensions,
            include_metadata_hash,
            zkevm_assembly::RunningVmEncodingMode::Production,
            debug_config,
        )?;
        build.write_to_standard_json(&mut solc_output, solc_version, &zksolc_version)?;
    }
    serde_json::to_writer(std::io::stdout(), &solc_output)?;
    std::process::exit(era_compiler_common::EXIT_CODE_SUCCESS);
}

///
/// Runs the standard JSON mode for EVM.
///
pub fn standard_json_evm(
    solc_compiler: Option<&SolcCompiler>,
    force_evmla: bool,
    json_path: Option<PathBuf>,
    base_path: Option<String>,
    include_paths: Vec<String>,
    allow_paths: Option<String>,
    debug_config: Option<era_compiler_llvm_context::DebugConfig>,
) -> anyhow::Result<()> {
    let zksolc_version = semver::Version::parse(env!("CARGO_PKG_VERSION")).expect("Always valid");

    let mut solc_input = SolcStandardJsonInput::try_from(json_path.as_deref())?;
    let language = solc_input.language;
    let sources = solc_input.sources()?;
    let libraries = solc_input.settings.libraries.clone().unwrap_or_default();

    let optimizer_settings =
        era_compiler_llvm_context::OptimizerSettings::try_from(&solc_input.settings.optimizer)?;
    let llvm_options = solc_input.settings.llvm_options.take().unwrap_or_default();

    let include_metadata_hash = match solc_input.settings.metadata {
        Some(ref metadata) => {
            metadata.bytecode_hash != Some(era_compiler_llvm_context::EraVMMetadataHash::None)
        }
        None => true,
    };

    let (mut solc_output, solc_version, project) = match (language, solc_compiler) {
        (SolcStandardJsonInputLanguage::Solidity, Some(solc_compiler)) => {
            let solc_pipeline = SolcPipeline::new(&solc_compiler.version, force_evmla);
            solc_input.normalize(&solc_compiler.version.default, Some(solc_pipeline));

            let mut solc_output = solc_compiler.standard_json(
                solc_input,
                Some(solc_pipeline),
                base_path,
                include_paths,
                allow_paths,
            )?;
            if solc_output.check_errors().is_err() {
                serde_json::to_writer(std::io::stdout(), &solc_output)?;
                std::process::exit(era_compiler_common::EXIT_CODE_SUCCESS);
            }

            let project = Project::try_from_solidity_sources(
                &mut solc_output,
                sources,
                libraries,
                solc_pipeline,
                solc_compiler,
                debug_config.as_ref(),
            )?;

            (solc_output, Some(&solc_compiler.version), project)
        }
        (SolcStandardJsonInputLanguage::Solidity, None) => {
            anyhow::bail!("Compiling Solidity without `solc` is not supported")
        }
        (SolcStandardJsonInputLanguage::Yul, Some(solc_compiler)) => {
            let solc_output = solc_compiler.validate_yul_standard_json(solc_input)?;

            let project = Project::try_from_yul_sources(
                sources,
                libraries,
                Some(&solc_compiler.version),
                debug_config.as_ref(),
            )?;

            (solc_output, Some(&solc_compiler.version), project)
        }
        (SolcStandardJsonInputLanguage::Yul, None) => {
            let solc_output = SolcStandardJsonOutput::new(&sources);
            let project =
                Project::try_from_yul_sources(sources, libraries, None, debug_config.as_ref())?;
            (solc_output, None, project)
        }
        (SolcStandardJsonInputLanguage::LLVMIR, Some(_)) => {
            anyhow::bail!("LLVM IR projects cannot be compiled with `solc`")
        }
        (SolcStandardJsonInputLanguage::LLVMIR, None) => {
            let solc_output = SolcStandardJsonOutput::new(&sources);
            let project = Project::try_from_llvm_ir_sources(sources)?;
            (solc_output, None, project)
        }
        (SolcStandardJsonInputLanguage::EraVMAssembly, _) => {
            anyhow::bail!("Compiling EraVM assembly to EVM is not supported")
        }
    };

    let build = project.compile_to_evm(
        optimizer_settings,
        llvm_options,
        include_metadata_hash,
        debug_config,
    )?;
    build.write_to_standard_json(&mut solc_output, solc_version, &zksolc_version)?;
    serde_json::to_writer(std::io::stdout(), &solc_output)?;
    std::process::exit(era_compiler_common::EXIT_CODE_SUCCESS);
}

///
/// Runs the combined JSON mode for EraVM.
///
pub fn combined_json_eravm(
    format: String,
    paths: &[PathBuf],
    libraries: Vec<String>,
    solc_compiler: &SolcCompiler,
    evm_version: Option<era_compiler_common::EVMVersion>,
    solc_optimizer_enabled: bool,
    force_evmla: bool,
    enable_eravm_extensions: bool,
    include_metadata_hash: bool,
    use_literal_content: bool,
    base_path: Option<String>,
    include_paths: Vec<String>,
    allow_paths: Option<String>,
    remappings: Option<BTreeSet<String>>,
    output_directory: Option<PathBuf>,
    overwrite: bool,
    optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
    llvm_options: Vec<String>,
    suppressed_warnings: Option<Vec<Warning>>,
    debug_config: Option<era_compiler_llvm_context::DebugConfig>,
) -> anyhow::Result<()> {
    let zksolc_version = semver::Version::parse(env!("CARGO_PKG_VERSION")).expect("Always valid");

    let build = standard_output_eravm(
        paths,
        libraries,
        solc_compiler,
        evm_version,
        solc_optimizer_enabled,
        force_evmla,
        enable_eravm_extensions,
        include_metadata_hash,
        use_literal_content,
        base_path,
        include_paths,
        allow_paths,
        remappings,
        optimizer_settings,
        llvm_options,
        suppressed_warnings,
        debug_config,
    )?;

    let mut combined_json = solc_compiler.combined_json(paths, format.as_str())?;
    build.write_to_combined_json(&mut combined_json, &zksolc_version)?;

    match output_directory {
        Some(output_directory) => {
            std::fs::create_dir_all(output_directory.as_path())?;
            combined_json.write_to_directory(output_directory.as_path(), overwrite)?;
        }
        None => {
            serde_json::to_writer(std::io::stdout(), &combined_json)?;
        }
    }
    std::process::exit(era_compiler_common::EXIT_CODE_SUCCESS);
}

///
/// Runs the combined JSON mode for EVM.
///
pub fn combined_json_evm(
    format: String,
    paths: &[PathBuf],
    libraries: Vec<String>,
    solc_compiler: &SolcCompiler,
    evm_version: Option<era_compiler_common::EVMVersion>,
    solc_optimizer_enabled: bool,
    force_evmla: bool,
    include_metadata_hash: bool,
    use_literal_content: bool,
    base_path: Option<String>,
    include_paths: Vec<String>,
    allow_paths: Option<String>,
    remappings: Option<BTreeSet<String>>,
    output_directory: Option<PathBuf>,
    overwrite: bool,
    optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
    llvm_options: Vec<String>,
    debug_config: Option<era_compiler_llvm_context::DebugConfig>,
) -> anyhow::Result<()> {
    let zksolc_version = semver::Version::parse(env!("CARGO_PKG_VERSION")).expect("Always valid");

    let build = standard_output_evm(
        paths,
        libraries,
        solc_compiler,
        evm_version,
        solc_optimizer_enabled,
        force_evmla,
        include_metadata_hash,
        use_literal_content,
        base_path,
        include_paths,
        allow_paths,
        remappings,
        optimizer_settings,
        llvm_options,
        debug_config,
    )?;

    let mut combined_json = solc_compiler.combined_json(paths, format.as_str())?;
    build.write_to_combined_json(&mut combined_json, &zksolc_version)?;

    match output_directory {
        Some(output_directory) => {
            std::fs::create_dir_all(output_directory.as_path())?;
            combined_json.write_to_directory(output_directory.as_path(), overwrite)?;
        }
        None => {
            serde_json::to_writer(std::io::stdout(), &combined_json)?;
        }
    }
    std::process::exit(era_compiler_common::EXIT_CODE_SUCCESS);
}

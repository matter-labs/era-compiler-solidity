//!
//! Solidity to zkEVM compiler library.
//!

pub(crate) mod build;
pub(crate) mod r#const;
pub(crate) mod evmla;
pub(crate) mod project;
pub(crate) mod solc;
pub(crate) mod yul;

pub use self::build::contract::Contract as ContractBuild;
pub use self::build::Build;
pub use self::project::contract::state::State as ContractState;
pub use self::project::contract::Contract as ProjectContract;
pub use self::project::Project;
pub use self::solc::combined_json::contract::Contract as SolcCombinedJsonContract;
pub use self::solc::combined_json::CombinedJson as SolcCombinedJson;
pub use self::solc::pipeline::Pipeline as SolcPipeline;
pub use self::solc::standard_json::input::language::Language as SolcStandardJsonInputLanguage;
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

use std::path::PathBuf;

///
/// Runs the Yul mode.
///
pub fn yul(
    input_files: &[PathBuf],
    solc: &SolcCompiler,
    optimizer_settings: compiler_llvm_context::OptimizerSettings,
    is_system_mode: bool,
    debug_config: Option<compiler_llvm_context::DebugConfig>,
) -> anyhow::Result<Build> {
    let path = match input_files.len() {
        1 => input_files.first().expect("Always exists"),
        0 => anyhow::bail!("The input file is missing"),
        length => anyhow::bail!(
            "Only one input file is allowed in the Yul mode, but found {}",
            length,
        ),
    };

    if !is_system_mode {
        solc.yul(input_files)?;
    }

    let project = Project::try_from_yul_path(path)?;

    let target_machine = compiler_llvm_context::TargetMachine::new(&optimizer_settings)?;
    let build = project.compile_all(
        target_machine,
        optimizer_settings,
        is_system_mode,
        debug_config,
    )?;

    Ok(build)
}

///
/// Runs the LLVM IR mode.
///
pub fn llvm_ir(
    input_files: &[PathBuf],
    optimizer_settings: compiler_llvm_context::OptimizerSettings,
    is_system_mode: bool,
    debug_config: Option<compiler_llvm_context::DebugConfig>,
) -> anyhow::Result<Build> {
    let path = match input_files.len() {
        1 => input_files.first().expect("Always exists"),
        0 => anyhow::bail!("The input file is missing"),
        length => anyhow::bail!(
            "Only one input file is allowed in the LLVM IR mode, but found {}",
            length,
        ),
    };

    let project = Project::try_from_llvm_ir_path(path)?;

    let target_machine = compiler_llvm_context::TargetMachine::new(&optimizer_settings)?;
    let build = project.compile_all(
        target_machine,
        optimizer_settings,
        is_system_mode,
        debug_config,
    )?;

    Ok(build)
}

///
/// Runs the standard output mode.
///
#[allow(clippy::too_many_arguments)]
pub fn standard_output(
    input_files: &[PathBuf],
    libraries: Vec<String>,
    solc: &SolcCompiler,
    optimizer_settings: compiler_llvm_context::OptimizerSettings,
    force_evmla: bool,
    is_system_mode: bool,
    base_path: Option<String>,
    include_paths: Vec<String>,
    allow_paths: Option<String>,
    debug_config: Option<compiler_llvm_context::DebugConfig>,
) -> anyhow::Result<Build> {
    let solc_version = solc.version()?;
    let solc_pipeline = SolcPipeline::new(&solc_version, force_evmla);

    let solc_input = SolcStandardJsonInput::try_from_paths(
        SolcStandardJsonInputLanguage::Solidity,
        input_files,
        libraries,
        SolcStandardJsonInputSettingsSelection::new_required(solc_pipeline),
        SolcStandardJsonInputSettingsOptimizer::new(true, None),
    )?;
    let source_code_files = solc_input
        .sources
        .iter()
        .map(|(path, source)| (path.to_owned(), source.content.to_owned()))
        .collect();

    let libraries = solc_input.settings.libraries.clone().unwrap_or_default();
    let mut solc_output = solc.standard_json(solc_input, base_path, include_paths, allow_paths)?;

    if let Some(errors) = solc_output.errors.as_deref() {
        let mut has_errors = false;

        for error in errors.iter() {
            if error.severity.as_str() == "error" {
                has_errors = true;
            }

            eprintln!("{error}");
        }

        if has_errors {
            anyhow::bail!("Error(s) found. Compilation aborted");
        }
    }

    let project = solc_output.try_to_project(
        source_code_files,
        libraries,
        solc_pipeline,
        &solc_version.default,
        debug_config.as_ref(),
    )?;

    let target_machine = compiler_llvm_context::TargetMachine::new(&optimizer_settings)?;
    let build = project.compile_all(
        target_machine,
        optimizer_settings,
        is_system_mode,
        debug_config,
    )?;

    Ok(build)
}

///
/// Runs the standard JSON mode.
///
#[allow(clippy::too_many_arguments)]
pub fn standard_json(
    solc: &SolcCompiler,
    force_evmla: bool,
    is_system_mode: bool,
    base_path: Option<String>,
    include_paths: Vec<String>,
    allow_paths: Option<String>,
    debug_config: Option<compiler_llvm_context::DebugConfig>,
) -> anyhow::Result<()> {
    let solc_version = solc.version()?;
    let solc_pipeline = SolcPipeline::new(&solc_version, force_evmla);

    let zksolc_version = semver::Version::parse(env!("CARGO_PKG_VERSION")).expect("Always valid");

    let solc_input = SolcStandardJsonInput::try_from_stdin(solc_pipeline)?;
    let source_code_files = solc_input
        .sources
        .iter()
        .map(|(path, source)| (path.to_owned(), source.content.to_owned()))
        .collect();

    let optimizer_settings =
        compiler_llvm_context::OptimizerSettings::try_from(&solc_input.settings.optimizer)?;

    let libraries = solc_input.settings.libraries.clone().unwrap_or_default();
    let mut solc_output = solc.standard_json(solc_input, base_path, include_paths, allow_paths)?;

    if let Some(errors) = solc_output.errors.as_deref() {
        for error in errors.iter() {
            if error.severity.as_str() == "error" {
                serde_json::to_writer(std::io::stdout(), &solc_output)?;
                std::process::exit(0);
            }
        }
    }

    let project = solc_output.try_to_project(
        source_code_files,
        libraries,
        solc_pipeline,
        &solc_version.default,
        debug_config.as_ref(),
    )?;

    let target_machine = compiler_llvm_context::TargetMachine::new(&optimizer_settings)?;
    let build = project.compile_all(
        target_machine,
        optimizer_settings,
        is_system_mode,
        debug_config,
    )?;

    build.write_to_standard_json(&mut solc_output, &solc_version, &zksolc_version)?;
    serde_json::to_writer(std::io::stdout(), &solc_output)?;
    std::process::exit(0);
}

///
/// Runs the combined JSON mode.
///
#[allow(clippy::too_many_arguments)]
pub fn combined_json(
    format: String,
    input_files: &[PathBuf],
    libraries: Vec<String>,
    solc: &SolcCompiler,
    optimizer_settings: compiler_llvm_context::OptimizerSettings,
    force_evmla: bool,
    is_system_mode: bool,
    base_path: Option<String>,
    include_paths: Vec<String>,
    allow_paths: Option<String>,
    debug_config: Option<compiler_llvm_context::DebugConfig>,
    output_directory: Option<PathBuf>,
    overwrite: bool,
) -> anyhow::Result<()> {
    let build = standard_output(
        input_files,
        libraries,
        solc,
        optimizer_settings,
        force_evmla,
        is_system_mode,
        base_path,
        include_paths,
        allow_paths,
        debug_config,
    )?;

    let zksolc_version = semver::Version::parse(env!("CARGO_PKG_VERSION")).expect("Always valid");

    let mut combined_json = solc.combined_json(input_files, format.as_str())?;
    build.write_to_combined_json(&mut combined_json, &zksolc_version)?;

    match output_directory {
        Some(output_directory) => {
            std::fs::create_dir_all(output_directory.as_path())?;

            combined_json.write_to_directory(output_directory.as_path(), overwrite)?;
        }
        None => {
            println!(
                "{}",
                serde_json::to_string(&combined_json).expect("Always valid")
            );
        }
    }
    std::process::exit(0);
}

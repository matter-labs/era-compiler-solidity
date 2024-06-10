//!
//! The Solidity compiler unit tests.
//!

#![cfg(test)]

mod factory_dependency;
mod ir_artifacts;
mod libraries;
mod messages;
mod optimizer;
mod remappings;
mod runtime_code;
mod standard_json;
mod unsupported_opcodes;

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::PathBuf;
use std::str::FromStr;

use crate::project::Project;
use crate::solc::pipeline::Pipeline as SolcPipeline;
use crate::solc::standard_json::input::settings::optimizer::Optimizer as SolcStandardJsonInputSettingsOptimizer;
use crate::solc::standard_json::input::settings::selection::Selection as SolcStandardJsonInputSettingsSelection;
use crate::solc::standard_json::input::Input as SolcStandardJsonInput;
use crate::solc::standard_json::output::Output as SolcStandardJsonOutput;
use crate::solc::Compiler as SolcCompiler;
use crate::warning::Warning;

///
/// Builds the Solidity project and returns the standard JSON output.
///
pub fn build_solidity(
    sources: BTreeMap<String, String>,
    libraries: BTreeMap<String, BTreeMap<String, String>>,
    remappings: Option<BTreeSet<String>>,
    pipeline: SolcPipeline,
    optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
) -> anyhow::Result<SolcStandardJsonOutput> {
    check_dependencies();

    inkwell::support::enable_llvm_pretty_stack_trace();
    era_compiler_llvm_context::initialize_target(era_compiler_llvm_context::Target::EraVM);
    let _ = crate::process::EXECUTABLE.set(PathBuf::from(crate::r#const::DEFAULT_EXECUTABLE_NAME));

    let solc_compiler = SolcCompiler::new(SolcCompiler::DEFAULT_EXECUTABLE_NAME)?;

    let solc_input = SolcStandardJsonInput::try_from_solidity_sources(
        None,
        sources.clone(),
        libraries.clone(),
        remappings,
        SolcStandardJsonInputSettingsSelection::new_required(Some(pipeline)),
        SolcStandardJsonInputSettingsOptimizer::new(
            true,
            None,
            &solc_compiler.version.default,
            false,
        ),
        None,
        pipeline == SolcPipeline::EVMLA,
        false,
        true,
        false,
        vec![],
        None,
    )?;

    let mut solc_output =
        solc_compiler.standard_json(solc_input, Some(pipeline), None, vec![], None)?;

    let project = Project::try_from_solidity_sources(
        &mut solc_output,
        sources,
        libraries,
        pipeline,
        &solc_compiler,
        None,
    )?;

    let build = project.compile_to_eravm(
        optimizer_settings,
        vec![],
        false,
        false,
        zkevm_assembly::RunningVmEncodingMode::Production,
        None,
    )?;
    build.write_to_standard_json(
        &mut solc_output,
        Some(&solc_compiler.version),
        &semver::Version::from_str(env!("CARGO_PKG_VERSION"))?,
    )?;

    solc_output.check_errors()?;
    Ok(solc_output)
}

///
/// Builds the Solidity project and returns the standard JSON output.
///
pub fn build_solidity_and_detect_missing_libraries(
    sources: BTreeMap<String, String>,
    libraries: BTreeMap<String, BTreeMap<String, String>>,
    pipeline: SolcPipeline,
) -> anyhow::Result<SolcStandardJsonOutput> {
    check_dependencies();

    inkwell::support::enable_llvm_pretty_stack_trace();
    era_compiler_llvm_context::initialize_target(era_compiler_llvm_context::Target::EraVM);
    let _ = crate::process::EXECUTABLE.set(PathBuf::from(crate::r#const::DEFAULT_EXECUTABLE_NAME));

    let solc_compiler = SolcCompiler::new(SolcCompiler::DEFAULT_EXECUTABLE_NAME)?;

    let solc_input = SolcStandardJsonInput::try_from_solidity_sources(
        None,
        sources.clone(),
        libraries.clone(),
        None,
        SolcStandardJsonInputSettingsSelection::new_required(Some(pipeline)),
        SolcStandardJsonInputSettingsOptimizer::new(
            true,
            None,
            &solc_compiler.version.default,
            false,
        ),
        None,
        pipeline == SolcPipeline::EVMLA,
        false,
        false,
        false,
        vec![],
        None,
    )?;

    let mut solc_output =
        solc_compiler.standard_json(solc_input, Some(pipeline), None, vec![], None)?;

    let project = Project::try_from_solidity_sources(
        &mut solc_output,
        sources,
        libraries,
        pipeline,
        &solc_compiler,
        None,
    )?;

    let missing_libraries = project.get_missing_libraries();
    missing_libraries.write_to_standard_json(
        &mut solc_output,
        Some(&solc_compiler.version),
        &semver::Version::from_str(env!("CARGO_PKG_VERSION"))?,
    )?;

    solc_output.check_errors()?;
    Ok(solc_output)
}

///
/// Builds the Yul `sources` and returns the standard JSON output.
///
pub fn build_yul(sources: BTreeMap<String, String>) -> anyhow::Result<SolcStandardJsonOutput> {
    check_dependencies();

    inkwell::support::enable_llvm_pretty_stack_trace();
    era_compiler_llvm_context::initialize_target(era_compiler_llvm_context::Target::EraVM);
    let _ = crate::process::EXECUTABLE.set(PathBuf::from(crate::r#const::DEFAULT_EXECUTABLE_NAME));

    let zksolc_version = semver::Version::parse(env!("CARGO_PKG_VERSION")).expect("Always valid");
    let optimizer_settings = era_compiler_llvm_context::OptimizerSettings::none();

    let mut solc_output = SolcStandardJsonOutput::new(&sources);

    let project = Project::try_from_yul_sources(sources, BTreeMap::new(), None, None)?;
    let build = project.compile_to_eravm(
        optimizer_settings,
        vec![],
        false,
        false,
        zkevm_assembly::RunningVmEncodingMode::Production,
        None,
    )?;
    build.write_to_standard_json(&mut solc_output, None, &zksolc_version)?;

    solc_output.check_errors()?;
    Ok(solc_output)
}

///
/// Builds the Yul standard JSON and returns the standard JSON output.
///
/// If `solc_compiler` is set, the standard JSON is validated with `solc`.
///
pub fn build_yul_standard_json(
    solc_input: SolcStandardJsonInput,
    solc_compiler: Option<&SolcCompiler>,
) -> anyhow::Result<SolcStandardJsonOutput> {
    check_dependencies();

    inkwell::support::enable_llvm_pretty_stack_trace();
    era_compiler_llvm_context::initialize_target(era_compiler_llvm_context::Target::EraVM);
    let _ = crate::process::EXECUTABLE.set(PathBuf::from(crate::r#const::DEFAULT_EXECUTABLE_NAME));

    let zksolc_version = semver::Version::parse(env!("CARGO_PKG_VERSION")).expect("Always valid");
    let optimizer_settings = era_compiler_llvm_context::OptimizerSettings::try_from_cli(
        solc_input.settings.optimizer.mode.unwrap_or('0'),
    )?;

    let sources = solc_input.sources()?;
    let (solc_version, mut solc_output) = match solc_compiler {
        Some(solc_compiler) => {
            let solc_output = solc_compiler.validate_yul_standard_json(solc_input)?;
            (Some(&solc_compiler.version), solc_output)
        }
        None => (None, SolcStandardJsonOutput::new(&sources)),
    };

    let project = Project::try_from_yul_sources(sources, BTreeMap::new(), solc_version, None)?;
    let build = project.compile_to_eravm(
        optimizer_settings,
        vec![],
        solc_compiler.is_none(),
        false,
        zkevm_assembly::RunningVmEncodingMode::Production,
        None,
    )?;
    build.write_to_standard_json(&mut solc_output, solc_version, &zksolc_version)?;

    solc_output.check_errors()?;
    Ok(solc_output)
}

///
/// Builds the LLVM IR standard JSON and returns the standard JSON output.
///
pub fn build_llvm_ir_standard_json(
    solc_input: SolcStandardJsonInput,
) -> anyhow::Result<SolcStandardJsonOutput> {
    check_dependencies();

    inkwell::support::enable_llvm_pretty_stack_trace();
    era_compiler_llvm_context::initialize_target(era_compiler_llvm_context::Target::EraVM);
    let _ = crate::process::EXECUTABLE.set(PathBuf::from(crate::r#const::DEFAULT_EXECUTABLE_NAME));

    let zksolc_version = semver::Version::parse(env!("CARGO_PKG_VERSION")).expect("Always valid");
    let optimizer_settings = era_compiler_llvm_context::OptimizerSettings::try_from_cli(
        solc_input.settings.optimizer.mode.unwrap_or('0'),
    )?;

    let sources = solc_input.sources()?;
    let mut solc_output = SolcStandardJsonOutput::new(&sources);

    let project = Project::try_from_llvm_ir_sources(sources)?;
    let build = project.compile_to_eravm(
        optimizer_settings,
        vec![],
        true,
        false,
        zkevm_assembly::RunningVmEncodingMode::Production,
        None,
    )?;
    build.write_to_standard_json(&mut solc_output, None, &zksolc_version)?;

    solc_output.check_errors()?;
    Ok(solc_output)
}

///
/// Builds the EraVM assembly standard JSON and returns the standard JSON output.
///
pub fn build_eravm_assembly_standard_json(
    solc_input: SolcStandardJsonInput,
) -> anyhow::Result<SolcStandardJsonOutput> {
    check_dependencies();

    inkwell::support::enable_llvm_pretty_stack_trace();
    era_compiler_llvm_context::initialize_target(era_compiler_llvm_context::Target::EraVM);
    let _ = crate::process::EXECUTABLE.set(PathBuf::from(crate::r#const::DEFAULT_EXECUTABLE_NAME));

    let zksolc_version = semver::Version::parse(env!("CARGO_PKG_VERSION")).expect("Always valid");
    let optimizer_settings = era_compiler_llvm_context::OptimizerSettings::try_from_cli(
        solc_input.settings.optimizer.mode.unwrap_or('0'),
    )?;

    let sources = solc_input.sources()?;
    let mut solc_output = SolcStandardJsonOutput::new(&sources);

    let project = Project::try_from_eravm_assembly_sources(sources)?;
    let build = project.compile_to_eravm(
        optimizer_settings,
        vec![],
        true,
        false,
        zkevm_assembly::RunningVmEncodingMode::Production,
        None,
    )?;
    build.write_to_standard_json(&mut solc_output, None, &zksolc_version)?;

    solc_output.check_errors()?;
    Ok(solc_output)
}

///
/// Checks if the built Solidity project contains the given warning.
///
pub fn check_solidity_warning(
    source_code: &str,
    warning_substring: &str,
    libraries: BTreeMap<String, BTreeMap<String, String>>,
    pipeline: SolcPipeline,
    skip_for_zkvm_edition: bool,
    suppressed_warnings: Option<Vec<Warning>>,
) -> anyhow::Result<bool> {
    check_dependencies();

    let solc = SolcCompiler::new(SolcCompiler::DEFAULT_EXECUTABLE_NAME)?;
    let solc_version = solc.version.to_owned();
    if skip_for_zkvm_edition && solc_version.l2_revision.is_some() {
        return Ok(true);
    }

    let mut sources = BTreeMap::new();
    sources.insert("test.sol".to_string(), source_code.to_string());
    let solc_input = SolcStandardJsonInput::try_from_solidity_sources(
        None,
        sources.clone(),
        libraries,
        None,
        SolcStandardJsonInputSettingsSelection::new_required(Some(pipeline)),
        SolcStandardJsonInputSettingsOptimizer::new(true, None, &solc_version.default, false),
        None,
        pipeline == SolcPipeline::EVMLA,
        false,
        false,
        false,
        vec![],
        suppressed_warnings,
    )?;

    let solc_output = solc.standard_json(solc_input, Some(pipeline), None, vec![], None)?;
    let contains_warning = solc_output
        .errors
        .ok_or_else(|| anyhow::anyhow!("Solidity compiler messages not found"))?
        .iter()
        .any(|error| error.formatted_message.contains(warning_substring));

    Ok(contains_warning)
}

///
/// Checks if the required executables are present in `${PATH}`.
///
fn check_dependencies() {
    for executable in [
        crate::r#const::DEFAULT_EXECUTABLE_NAME,
        SolcCompiler::DEFAULT_EXECUTABLE_NAME,
    ]
    .iter()
    {
        assert!(
            which::which(executable).is_ok(),
            "The `{executable}` executable not found in ${{PATH}}"
        );
    }
}

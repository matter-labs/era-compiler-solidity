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
mod standard_json;
mod unsupported_instructions;

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::PathBuf;
use std::str::FromStr;

use crate::message_type::MessageType;
use crate::project::Project;
use crate::solc::pipeline::Pipeline as SolcPipeline;
use crate::solc::standard_json::input::settings::optimizer::Optimizer as SolcStandardJsonInputSettingsOptimizer;
use crate::solc::standard_json::input::settings::selection::Selection as SolcStandardJsonInputSettingsSelection;
use crate::solc::standard_json::input::source::Source as SolcStandardJsonInputSource;
use crate::solc::standard_json::input::Input as SolcStandardJsonInput;
use crate::solc::standard_json::output::error::collectable::Collectable as CollectableError;
use crate::solc::standard_json::output::Output as SolcStandardJsonOutput;
use crate::solc::Compiler as SolcCompiler;

///
/// Builds the Solidity project and returns the standard JSON output.
///
pub fn build_solidity(
    sources: BTreeMap<String, String>,
    libraries: BTreeMap<String, BTreeMap<String, String>>,
    remappings: Option<BTreeSet<String>>,
    solc_version: &semver::Version,
    solc_pipeline: SolcPipeline,
    optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
) -> anyhow::Result<SolcStandardJsonOutput> {
    check_dependencies(Some(solc_version));

    inkwell::support::enable_llvm_pretty_stack_trace();
    era_compiler_llvm_context::initialize_target(era_compiler_common::Target::EraVM);
    let _ = crate::process::EXECUTABLE.set(PathBuf::from(crate::r#const::DEFAULT_EXECUTABLE_NAME));

    let solc_compiler = SolcCompiler::new(
        format!(
            "{}-{}{}",
            SolcCompiler::DEFAULT_EXECUTABLE_NAME,
            solc_version,
            std::env::consts::EXE_SUFFIX,
        )
        .as_str(),
    )?;

    let mut solc_input = SolcStandardJsonInput::try_from_solidity_sources(
        None,
        sources.clone(),
        libraries.clone(),
        remappings,
        SolcStandardJsonInputSettingsSelection::new_required(Some(solc_pipeline)),
        SolcStandardJsonInputSettingsOptimizer::new(
            true,
            None,
            &solc_compiler.version.default,
            false,
        ),
        None,
        solc_pipeline == SolcPipeline::EVMLA,
        false,
        true,
        false,
        vec![],
        vec![],
        vec![],
    )?;

    let mut solc_output = solc_compiler.standard_json(
        &mut solc_input,
        Some(solc_pipeline),
        &mut vec![],
        None,
        vec![],
        None,
    )?;
    solc_output.take_and_write_warnings();
    solc_output.collect_errors()?;

    let project = Project::try_from_solc_output(
        libraries,
        solc_pipeline,
        &mut solc_output,
        &solc_compiler,
        None,
    )?;
    solc_output.take_and_write_warnings();
    solc_output.collect_errors()?;

    let build = project.compile_to_eravm(
        &mut vec![],
        true,
        true,
        optimizer_settings,
        vec![],
        false,
        None,
        None,
    )?;
    build.write_to_standard_json(
        &mut solc_output,
        Some(&solc_compiler.version),
        &semver::Version::from_str(env!("CARGO_PKG_VERSION"))?,
    )?;

    solc_output.take_and_write_warnings();
    solc_output.collect_errors()?;
    Ok(solc_output)
}

///
/// Builds the Solidity project and returns the standard JSON output.
///
pub fn build_solidity_and_detect_missing_libraries(
    sources: BTreeMap<String, String>,
    libraries: BTreeMap<String, BTreeMap<String, String>>,
    solc_version: &semver::Version,
    solc_pipeline: SolcPipeline,
) -> anyhow::Result<SolcStandardJsonOutput> {
    check_dependencies(Some(solc_version));

    inkwell::support::enable_llvm_pretty_stack_trace();
    era_compiler_llvm_context::initialize_target(era_compiler_common::Target::EraVM);
    let _ = crate::process::EXECUTABLE.set(PathBuf::from(crate::r#const::DEFAULT_EXECUTABLE_NAME));

    let solc_compiler = SolcCompiler::new(
        format!(
            "{}-{}{}",
            SolcCompiler::DEFAULT_EXECUTABLE_NAME,
            solc_version,
            std::env::consts::EXE_SUFFIX,
        )
        .as_str(),
    )?;

    let mut solc_input = SolcStandardJsonInput::try_from_solidity_sources(
        None,
        sources.clone(),
        libraries.clone(),
        None,
        SolcStandardJsonInputSettingsSelection::new_required(Some(solc_pipeline)),
        SolcStandardJsonInputSettingsOptimizer::new(
            true,
            None,
            &solc_compiler.version.default,
            false,
        ),
        None,
        solc_pipeline == SolcPipeline::EVMLA,
        false,
        false,
        false,
        vec![],
        vec![],
        vec![],
    )?;

    let mut solc_output = solc_compiler.standard_json(
        &mut solc_input,
        Some(solc_pipeline),
        &mut vec![],
        None,
        vec![],
        None,
    )?;

    let project = Project::try_from_solc_output(
        libraries,
        solc_pipeline,
        &mut solc_output,
        &solc_compiler,
        None,
    )?;

    let missing_libraries = project.get_missing_libraries();
    missing_libraries.write_to_standard_json(
        &mut solc_output,
        Some(&solc_compiler.version),
        &semver::Version::from_str(env!("CARGO_PKG_VERSION"))?,
    )?;

    solc_output.take_and_write_warnings();
    solc_output.collect_errors()?;
    Ok(solc_output)
}

///
/// Builds the Yul `sources` and returns the standard JSON output.
///
pub fn build_yul(sources: BTreeMap<String, String>) -> anyhow::Result<SolcStandardJsonOutput> {
    check_dependencies(None);

    inkwell::support::enable_llvm_pretty_stack_trace();
    era_compiler_llvm_context::initialize_target(era_compiler_common::Target::EraVM);
    let _ = crate::process::EXECUTABLE.set(PathBuf::from(crate::r#const::DEFAULT_EXECUTABLE_NAME));

    let zksolc_version = semver::Version::parse(env!("CARGO_PKG_VERSION")).expect("Always valid");
    let optimizer_settings = era_compiler_llvm_context::OptimizerSettings::none();

    let sources = sources
        .into_iter()
        .map(|(path, source)| (path, SolcStandardJsonInputSource::from(source)))
        .collect();

    let mut solc_output = SolcStandardJsonOutput::new(&sources, &mut vec![]);

    let project = Project::try_from_yul_sources(
        sources,
        BTreeMap::new(),
        Some(&mut solc_output),
        None,
        None,
    )?;
    let build = project.compile_to_eravm(
        &mut vec![],
        true,
        true,
        optimizer_settings,
        vec![],
        false,
        None,
        None,
    )?;
    build.write_to_standard_json(&mut solc_output, None, &zksolc_version)?;

    solc_output.take_and_write_warnings();
    solc_output.collect_errors()?;
    Ok(solc_output)
}

///
/// Builds the Yul standard JSON and returns the standard JSON output.
///
/// If `solc_compiler` is set, the standard JSON is validated with `solc`.
///
pub fn build_yul_standard_json(
    mut solc_input: SolcStandardJsonInput,
    solc_compiler: Option<&SolcCompiler>,
) -> anyhow::Result<SolcStandardJsonOutput> {
    check_dependencies(solc_compiler.map(|compiler| &compiler.version.default));

    inkwell::support::enable_llvm_pretty_stack_trace();
    era_compiler_llvm_context::initialize_target(era_compiler_common::Target::EraVM);
    let _ = crate::process::EXECUTABLE.set(PathBuf::from(crate::r#const::DEFAULT_EXECUTABLE_NAME));

    let zksolc_version = semver::Version::parse(env!("CARGO_PKG_VERSION")).expect("Always valid");
    let optimizer_settings = era_compiler_llvm_context::OptimizerSettings::try_from_cli(
        solc_input.settings.optimizer.mode.unwrap_or('0'),
    )?;

    let (solc_version, mut solc_output) = match solc_compiler {
        Some(solc_compiler) => {
            let solc_output =
                solc_compiler.validate_yul_standard_json(&mut solc_input, &mut vec![])?;
            (Some(&solc_compiler.version), solc_output)
        }
        None => (
            None,
            SolcStandardJsonOutput::new(&solc_input.sources, &mut vec![]),
        ),
    };

    let project = Project::try_from_yul_sources(
        solc_input.sources,
        BTreeMap::new(),
        Some(&mut solc_output),
        solc_version,
        None,
    )?;
    let build = project.compile_to_eravm(
        &mut vec![],
        solc_compiler.is_none(),
        true,
        optimizer_settings,
        vec![],
        false,
        None,
        None,
    )?;
    build.write_to_standard_json(&mut solc_output, solc_version, &zksolc_version)?;

    solc_output.take_and_write_warnings();
    solc_output.collect_errors()?;
    Ok(solc_output)
}

///
/// Builds the LLVM IR standard JSON and returns the standard JSON output.
///
pub fn build_llvm_ir_standard_json(
    input: SolcStandardJsonInput,
) -> anyhow::Result<SolcStandardJsonOutput> {
    check_dependencies(None);

    inkwell::support::enable_llvm_pretty_stack_trace();
    era_compiler_llvm_context::initialize_target(era_compiler_common::Target::EraVM);
    let _ = crate::process::EXECUTABLE.set(PathBuf::from(crate::r#const::DEFAULT_EXECUTABLE_NAME));

    let zksolc_version = semver::Version::parse(env!("CARGO_PKG_VERSION")).expect("Always valid");
    let optimizer_settings = era_compiler_llvm_context::OptimizerSettings::try_from_cli(
        input.settings.optimizer.mode.unwrap_or('0'),
    )?;

    let mut output = SolcStandardJsonOutput::new(&BTreeMap::new(), &mut vec![]);

    let project = Project::try_from_llvm_ir_sources(input.sources, Some(&mut output))?;
    let build = project.compile_to_eravm(
        &mut vec![],
        true,
        true,
        optimizer_settings,
        vec![],
        false,
        None,
        None,
    )?;
    build.write_to_standard_json(&mut output, None, &zksolc_version)?;

    output.take_and_write_warnings();
    output.collect_errors()?;
    Ok(output)
}

///
/// Builds the EraVM assembly standard JSON and returns the standard JSON output.
///
pub fn build_eravm_assembly_standard_json(
    input: SolcStandardJsonInput,
) -> anyhow::Result<SolcStandardJsonOutput> {
    check_dependencies(None);

    inkwell::support::enable_llvm_pretty_stack_trace();
    era_compiler_llvm_context::initialize_target(era_compiler_common::Target::EraVM);
    let _ = crate::process::EXECUTABLE.set(PathBuf::from(crate::r#const::DEFAULT_EXECUTABLE_NAME));

    let zksolc_version = semver::Version::parse(env!("CARGO_PKG_VERSION")).expect("Always valid");
    let optimizer_settings = era_compiler_llvm_context::OptimizerSettings::try_from_cli(
        input.settings.optimizer.mode.unwrap_or('0'),
    )?;

    let mut output = SolcStandardJsonOutput::new(&BTreeMap::new(), &mut vec![]);

    let project = Project::try_from_eravm_assembly_sources(input.sources, Some(&mut output))?;
    let build = project.compile_to_eravm(
        &mut vec![],
        true,
        true,
        optimizer_settings,
        vec![],
        false,
        None,
        None,
    )?;
    build.write_to_standard_json(&mut output, None, &zksolc_version)?;

    output.take_and_write_warnings();
    output.collect_errors()?;
    Ok(output)
}

///
/// Checks if the built Solidity project contains the given warning.
///
pub fn check_solidity_message(
    source_code: &str,
    warning_substring: &str,
    libraries: BTreeMap<String, BTreeMap<String, String>>,
    solc_version: &semver::Version,
    solc_pipeline: SolcPipeline,
    skip_for_zksync_edition: bool,
    suppressed_warnings: Vec<MessageType>,
) -> anyhow::Result<bool> {
    check_dependencies(Some(solc_version));

    let solc_compiler = SolcCompiler::new(
        format!(
            "{}-{}{}",
            SolcCompiler::DEFAULT_EXECUTABLE_NAME,
            solc_version,
            std::env::consts::EXE_SUFFIX,
        )
        .as_str(),
    )?;
    if skip_for_zksync_edition && solc_compiler.version.l2_revision.is_some() {
        return Ok(true);
    }

    let mut sources = BTreeMap::new();
    sources.insert("test.sol".to_string(), source_code.to_string());
    let mut solc_input = SolcStandardJsonInput::try_from_solidity_sources(
        None,
        sources.clone(),
        libraries,
        None,
        SolcStandardJsonInputSettingsSelection::new_required(Some(solc_pipeline)),
        SolcStandardJsonInputSettingsOptimizer::new(true, None, solc_version, false),
        None,
        solc_pipeline == SolcPipeline::EVMLA,
        false,
        false,
        false,
        vec![],
        vec![],
        suppressed_warnings,
    )?;

    let solc_output = solc_compiler.standard_json(
        &mut solc_input,
        Some(solc_pipeline),
        &mut vec![],
        None,
        vec![],
        None,
    )?;
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
fn check_dependencies(solc_version: Option<&semver::Version>) {
    let mut executables = vec![crate::r#const::DEFAULT_EXECUTABLE_NAME.to_owned()];
    if let Some(solc_version) = solc_version {
        executables.push(format!(
            "{}-{}{}",
            SolcCompiler::DEFAULT_EXECUTABLE_NAME,
            solc_version,
            std::env::consts::EXE_SUFFIX,
        ));
    }
    for executable in executables.into_iter() {
        assert!(
            which::which(executable.as_str()).is_ok(),
            "The `{executable}` executable not found in ${{PATH}}"
        );
    }
}

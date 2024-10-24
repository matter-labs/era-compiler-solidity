//!
//! The Solidity compiler unit tests.
//!

#![allow(dead_code)]

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Once;
use std::time::Duration;

use assert_cmd::Command;

use era_compiler_solidity::error_type::ErrorType;
use era_compiler_solidity::project::Project;
use era_compiler_solidity::solc::codegen::Codegen as SolcCodegen;
use era_compiler_solidity::solc::standard_json::input::settings::metadata::Metadata as SolcStandardJsonInputSettingsMetadata;
use era_compiler_solidity::solc::standard_json::input::settings::optimizer::Optimizer as SolcStandardJsonInputSettingsOptimizer;
use era_compiler_solidity::solc::standard_json::input::settings::selection::Selection as SolcStandardJsonInputSettingsSelection;
use era_compiler_solidity::solc::standard_json::input::source::Source as SolcStandardJsonInputSource;
use era_compiler_solidity::solc::standard_json::input::Input as SolcStandardJsonInput;
use era_compiler_solidity::solc::standard_json::output::error::collectable::Collectable as CollectableError;
use era_compiler_solidity::solc::standard_json::output::Output as SolcStandardJsonOutput;
use era_compiler_solidity::solc::Compiler as SolcCompiler;
use era_compiler_solidity::warning_type::WarningType;

/// Synchronization for `solc` downloads.
static DOWNLOAD_SOLC: Once = Once::new();

/// Download directory for `solc` binaries.
pub const SOLC_DOWNLOAD_DIRECTORY: &str = "solc-bin";

/// Path to the `solc` binary configuration file.
pub const SOLC_BIN_CONFIG_PATH: &str = "tests/solc-bin.json";

///
/// Returns the `solc` compiler for the given version.
///
pub fn get_solc_compiler(solc_version: &semver::Version) -> anyhow::Result<SolcCompiler> {
    let solc_path = PathBuf::from(SOLC_DOWNLOAD_DIRECTORY).join(format!(
        "{}-{solc_version}{}",
        SolcCompiler::DEFAULT_EXECUTABLE_NAME,
        std::env::consts::EXE_SUFFIX,
    ));

    SolcCompiler::new(solc_path.to_str().unwrap())
}

///
/// Downloads the necessary compiler binaries.
///
pub fn download_binaries() -> anyhow::Result<()> {
    let mut http_client_builder = reqwest::blocking::ClientBuilder::new();
    http_client_builder = http_client_builder.connect_timeout(Duration::from_secs(60));
    http_client_builder = http_client_builder.pool_idle_timeout(Duration::from_secs(60));
    http_client_builder = http_client_builder.timeout(Duration::from_secs(60));
    let http_client = http_client_builder.build()?;

    let config_path = Path::new(SOLC_BIN_CONFIG_PATH);
    era_compiler_downloader::Downloader::new(http_client.clone()).download(config_path)?;

    // Copy the latest `solc-*` binary to `solc` for CLI tests
    let latest_solc =
        PathBuf::from(get_solc_compiler(&SolcCompiler::LAST_SUPPORTED_VERSION)?.executable);
    let mut solc = latest_solc.clone();
    solc.set_file_name(format!("solc{}", std::env::consts::EXE_SUFFIX));
    std::fs::copy(latest_solc, solc)?;

    Ok(())
}

///
/// Setup required test dependencies.
///
pub fn setup() -> anyhow::Result<()> {
    // Download `solc` binaries once
    DOWNLOAD_SOLC.call_once(|| {
        download_binaries().expect("Unable to download solc binaries. Aborting...");
    });

    // Set the `zksolc` binary path
    let zksolc_bin = Command::cargo_bin(era_compiler_solidity::DEFAULT_EXECUTABLE_NAME)?;
    let _ = era_compiler_solidity::process::EXECUTABLE.set(PathBuf::from(zksolc_bin.get_program()));

    // Enable LLVM pretty stack trace
    inkwell::support::enable_llvm_pretty_stack_trace();
    Ok(())
}

///
/// Builds the Solidity project and returns the standard JSON output.
///
pub fn build_solidity(
    sources: BTreeMap<String, String>,
    libraries: BTreeMap<String, BTreeMap<String, String>>,
    remappings: BTreeSet<String>,
    solc_version: &semver::Version,
    solc_pipeline: SolcCodegen,
    optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
) -> anyhow::Result<SolcStandardJsonOutput> {
    self::setup()?;

    let solc_compiler = get_solc_compiler(solc_version)?;

    era_compiler_llvm_context::initialize_target(era_compiler_common::Target::EraVM);

    let sources: BTreeMap<String, SolcStandardJsonInputSource> = sources
        .into_iter()
        .map(|(path, source)| (path, SolcStandardJsonInputSource::from(source)))
        .collect();

    let mut solc_input = SolcStandardJsonInput::try_from_solidity_sources(
        sources,
        libraries.clone(),
        remappings,
        SolcStandardJsonInputSettingsOptimizer::default(),
        Some(solc_pipeline),
        None,
        true,
        SolcStandardJsonInputSettingsSelection::new_required(Some(solc_pipeline)),
        SolcStandardJsonInputSettingsMetadata::default(),
        vec![],
        vec![],
        vec![],
        false,
        false,
    )?;

    let mut solc_output = solc_compiler.standard_json(
        &mut solc_input,
        Some(solc_pipeline),
        &mut vec![],
        None,
        vec![],
        None,
    )?;
    solc_output.collect_errors()?;

    let project = Project::try_from_solc_output(
        libraries,
        solc_pipeline,
        &mut solc_output,
        &solc_compiler,
        None,
    )?;
    solc_output.collect_errors()?;

    let build = project.compile_to_eravm(
        &mut vec![],
        true,
        era_compiler_common::HashType::Ipfs,
        optimizer_settings,
        vec![],
        false,
        None,
        None,
    )?;
    build.write_to_standard_json(&mut solc_output, Some(&solc_compiler.version))?;

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
    solc_pipeline: SolcCodegen,
) -> anyhow::Result<SolcStandardJsonOutput> {
    self::setup()?;

    let solc_compiler = get_solc_compiler(solc_version)?;

    era_compiler_llvm_context::initialize_target(era_compiler_common::Target::EraVM);

    let sources: BTreeMap<String, SolcStandardJsonInputSource> = sources
        .into_iter()
        .map(|(path, source)| (path, SolcStandardJsonInputSource::from(source)))
        .collect();

    let mut solc_input = SolcStandardJsonInput::try_from_solidity_sources(
        sources,
        libraries.clone(),
        BTreeSet::new(),
        SolcStandardJsonInputSettingsOptimizer::default(),
        Some(solc_pipeline),
        None,
        false,
        SolcStandardJsonInputSettingsSelection::new_required(Some(solc_pipeline)),
        SolcStandardJsonInputSettingsMetadata::default(),
        vec![],
        vec![],
        vec![],
        false,
        false,
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
    missing_libraries.write_to_standard_json(&mut solc_output, Some(&solc_compiler.version));

    solc_output.collect_errors()?;
    Ok(solc_output)
}

///
/// Builds the Yul `sources` and returns the standard JSON output.
///
pub fn build_yul(sources: BTreeMap<String, String>) -> anyhow::Result<SolcStandardJsonOutput> {
    self::setup()?;

    era_compiler_llvm_context::initialize_target(era_compiler_common::Target::EraVM);

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
        era_compiler_common::HashType::Ipfs,
        optimizer_settings,
        vec![],
        false,
        None,
        None,
    )?;
    build.write_to_standard_json(&mut solc_output, None)?;

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
    self::setup()?;

    era_compiler_llvm_context::initialize_target(era_compiler_common::Target::EraVM);

    let optimizer_settings = era_compiler_llvm_context::OptimizerSettings::try_from_cli(
        solc_input.settings.optimizer.mode,
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
        era_compiler_common::HashType::Ipfs,
        optimizer_settings,
        vec![],
        false,
        None,
        None,
    )?;
    build.write_to_standard_json(&mut solc_output, solc_version)?;

    solc_output.collect_errors()?;
    Ok(solc_output)
}

///
/// Builds the LLVM IR standard JSON and returns the standard JSON output.
///
pub fn build_llvm_ir_standard_json(
    input: SolcStandardJsonInput,
) -> anyhow::Result<SolcStandardJsonOutput> {
    self::setup()?;

    era_compiler_llvm_context::initialize_target(era_compiler_common::Target::EraVM);

    let optimizer_settings =
        era_compiler_llvm_context::OptimizerSettings::try_from_cli(input.settings.optimizer.mode)?;

    let mut output = SolcStandardJsonOutput::new(&BTreeMap::new(), &mut vec![]);

    let project = Project::try_from_llvm_ir_sources(input.sources, Some(&mut output))?;
    let build = project.compile_to_eravm(
        &mut vec![],
        true,
        era_compiler_common::HashType::Ipfs,
        optimizer_settings,
        vec![],
        false,
        None,
        None,
    )?;
    build.write_to_standard_json(&mut output, None)?;

    output.collect_errors()?;
    Ok(output)
}

///
/// Builds the EraVM assembly standard JSON and returns the standard JSON output.
///
pub fn build_eravm_assembly_standard_json(
    input: SolcStandardJsonInput,
) -> anyhow::Result<SolcStandardJsonOutput> {
    self::setup()?;

    era_compiler_llvm_context::initialize_target(era_compiler_common::Target::EraVM);

    let optimizer_settings =
        era_compiler_llvm_context::OptimizerSettings::try_from_cli(input.settings.optimizer.mode)?;

    let mut output = SolcStandardJsonOutput::new(&BTreeMap::new(), &mut vec![]);

    let project = Project::try_from_eravm_assembly_sources(input.sources, Some(&mut output))?;
    let build = project.compile_to_eravm(
        &mut vec![],
        true,
        era_compiler_common::HashType::Ipfs,
        optimizer_settings,
        vec![],
        false,
        None,
        None,
    )?;
    build.write_to_standard_json(&mut output, None)?;

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
    solc_pipeline: SolcCodegen,
    skip_for_zksync_edition: bool,
    suppressed_errors: Vec<ErrorType>,
    suppressed_warnings: Vec<WarningType>,
) -> anyhow::Result<bool> {
    self::setup()?;

    let solc_compiler = get_solc_compiler(solc_version)?;

    if skip_for_zksync_edition && solc_compiler.version.l2_revision.is_some() {
        return Ok(true);
    }

    let mut sources = BTreeMap::new();
    sources.insert(
        "test.sol".to_string(),
        SolcStandardJsonInputSource::from(source_code.to_string()),
    );

    let mut solc_input = SolcStandardJsonInput::try_from_solidity_sources(
        sources,
        libraries,
        BTreeSet::new(),
        SolcStandardJsonInputSettingsOptimizer::default(),
        Some(solc_pipeline),
        None,
        false,
        SolcStandardJsonInputSettingsSelection::new_required(Some(solc_pipeline)),
        SolcStandardJsonInputSettingsMetadata::default(),
        vec![],
        suppressed_errors,
        suppressed_warnings,
        false,
        false,
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

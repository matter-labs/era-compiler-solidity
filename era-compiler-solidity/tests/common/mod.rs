//!
//! Unit test common utilities.
//!

#![allow(dead_code)]

pub mod r#const;

pub use self::r#const::*;

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Once;
use std::time::Duration;

use assert_cmd::Command;

use era_compiler_solidity::project::Project;
use era_solc::CollectableError;

/// Synchronization for `solc` downloads.
static DOWNLOAD_SOLC: Once = Once::new();

/// Synchronization for upstream `solc` downloads.
static DOWNLOAD_SOLC_UPSTREAM: Once = Once::new();

///
/// Setup required test dependencies.
///
pub fn setup() -> anyhow::Result<()> {
    // Download `solc` executables once
    DOWNLOAD_SOLC.call_once(|| {
        download_executables(SOLC_BIN_CONFIG_PATH, true)
            .expect("Unable to download `solc` executables");
    });

    // Set the `zksolc` binary path
    let zksolc_bin = Command::cargo_bin(era_compiler_solidity::DEFAULT_EXECUTABLE_NAME)?;
    let _ = era_compiler_solidity::process::EXECUTABLE.set(PathBuf::from(zksolc_bin.get_program()));

    // Enable LLVM pretty stack trace
    inkwell::support::enable_llvm_pretty_stack_trace();

    Ok(())
}

///
/// Downloads the necessary compiler executables.
///
pub fn download_executables(config_path: &str, create_alias: bool) -> anyhow::Result<()> {
    let mut http_client_builder = reqwest::blocking::ClientBuilder::new();
    http_client_builder = http_client_builder.connect_timeout(Duration::from_secs(60));
    http_client_builder = http_client_builder.pool_idle_timeout(Duration::from_secs(60));
    http_client_builder = http_client_builder.timeout(Duration::from_secs(60));
    let http_client = http_client_builder.build()?;

    let config_path = Path::new(config_path);
    era_compiler_downloader::Downloader::new(http_client.clone()).download(config_path)?;

    if create_alias {
        // Copy the latest `solc-*` binary to `solc` for CLI tests
        let latest_solc = PathBuf::from(
            get_solc_compiler(&era_solc::Compiler::LAST_SUPPORTED_VERSION)?.executable,
        );
        let mut solc = latest_solc.clone();
        solc.set_file_name(format!("solc{}", std::env::consts::EXE_SUFFIX));
        std::fs::copy(latest_solc, solc)?;
    }

    Ok(())
}

///
/// Returns the `solc` compiler for the given version.
///
pub fn get_solc_compiler(version: &semver::Version) -> anyhow::Result<era_solc::Compiler> {
    let solc_path = PathBuf::from(SOLC_DOWNLOAD_DIRECTORY).join(format!(
        "{}-{version}{}",
        era_solc::Compiler::DEFAULT_EXECUTABLE_NAME,
        std::env::consts::EXE_SUFFIX,
    ));
    era_solc::Compiler::try_from_path(solc_path.to_str().expect("Always valid"))
}

///
/// Reads source code files from the disk.
///
pub fn read_sources(paths: &[&str]) -> BTreeMap<String, String> {
    paths
        .into_iter()
        .map(|path| {
            let result = std::fs::read_to_string(path).map_err(|error| anyhow::anyhow!(error));
            result.map(|result| ((*path).to_owned(), result))
        })
        .collect::<anyhow::Result<BTreeMap<String, String>>>()
        .expect("Source reading failure")
}

///
/// Builds the Solidity project and returns the standard JSON output.
///
pub fn build_solidity_standard_json(
    sources: BTreeMap<String, String>,
    libraries: era_compiler_common::Libraries,
    metadata_hash_type: era_compiler_common::HashType,
    remappings: BTreeSet<String>,
    solc_version: &semver::Version,
    solc_codegen: era_solc::StandardJsonInputCodegen,
    optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
) -> anyhow::Result<era_solc::StandardJsonOutput> {
    self::setup()?;

    let solc_compiler = get_solc_compiler(solc_version)?;

    era_compiler_llvm_context::initialize_target(era_compiler_common::Target::EraVM);

    let sources: BTreeMap<String, era_solc::StandardJsonInputSource> = sources
        .into_iter()
        .map(|(path, source)| (path, era_solc::StandardJsonInputSource::from(source)))
        .collect();

    let mut solc_input = era_solc::StandardJsonInput::try_from_solidity_sources(
        sources,
        libraries.clone(),
        remappings,
        era_solc::StandardJsonInputOptimizer::default(),
        Some(solc_codegen),
        None,
        true,
        era_solc::StandardJsonInputSelection::new_required(solc_codegen),
        era_solc::StandardJsonInputMetadata::default(),
        vec![],
        vec![],
        vec![],
        false,
        false,
    )?;

    let mut solc_output = solc_compiler.standard_json(
        era_compiler_common::Target::EraVM,
        &mut solc_input,
        &mut vec![],
        None,
        vec![],
        None,
    )?;
    solc_output.check_errors()?;

    let linker_symbols = libraries.as_linker_symbols()?;

    let project = Project::try_from_solc_output(
        libraries,
        solc_codegen,
        &mut solc_output,
        &solc_compiler,
        None,
    )?;
    solc_output.check_errors()?;

    let build = project.compile_to_eravm(
        &mut vec![],
        true,
        metadata_hash_type,
        optimizer_settings,
        vec![],
        false,
        None,
    )?;
    build.check_errors()?;

    let build = build.link(linker_symbols);
    build.check_errors()?;

    build.write_to_standard_json(&mut solc_output, Some(&solc_compiler.version))?;
    solc_output.check_errors()?;
    Ok(solc_output)
}

///
/// Builds the Solidity project and returns the combined JSON output.
///
pub fn build_solidity_combined_json(
    sources: BTreeMap<String, String>,
    libraries: era_compiler_common::Libraries,
    selectors: Vec<era_solc::CombinedJsonSelector>,
    metadata_hash_type: era_compiler_common::HashType,
    solc_version: &semver::Version,
    solc_codegen: era_solc::StandardJsonInputCodegen,
    optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
) -> anyhow::Result<era_solc::CombinedJson> {
    self::setup()?;

    era_compiler_llvm_context::initialize_target(era_compiler_common::Target::EraVM);

    let solc_compiler = get_solc_compiler(solc_version)?;
    let paths: Vec<PathBuf> = sources.keys().map(PathBuf::from).collect();

    let mut solc_output = self::build_solidity_standard_json(
        sources,
        libraries.clone(),
        metadata_hash_type,
        BTreeSet::new(),
        solc_version,
        solc_codegen,
        optimizer_settings.clone(),
    )?;

    let project = Project::try_from_solc_output(
        libraries,
        solc_codegen,
        &mut solc_output,
        &solc_compiler,
        None,
    )?;
    solc_output.check_errors()?;

    let build = project.compile_to_eravm(
        &mut vec![],
        true,
        metadata_hash_type,
        optimizer_settings,
        vec![],
        selectors.contains(&era_solc::CombinedJsonSelector::Assembly),
        None,
    )?;
    build.check_errors()?;

    let mut combined_json = solc_compiler.combined_json(
        paths.as_slice(),
        selectors.into_iter().collect(),
        Some(solc_codegen),
    )?;
    build.write_to_combined_json(&mut combined_json)?;
    Ok(combined_json)
}

///
/// Builds the Solidity project and returns the standard JSON output.
///
pub fn build_solidity_and_detect_missing_libraries(
    sources: BTreeMap<String, String>,
    libraries: era_compiler_common::Libraries,
    solc_version: &semver::Version,
    solc_codegen: era_solc::StandardJsonInputCodegen,
) -> anyhow::Result<era_solc::StandardJsonOutput> {
    self::setup()?;

    let solc_compiler = get_solc_compiler(solc_version)?;

    era_compiler_llvm_context::initialize_target(era_compiler_common::Target::EraVM);

    let sources: BTreeMap<String, era_solc::StandardJsonInputSource> = sources
        .into_iter()
        .map(|(path, source)| (path, era_solc::StandardJsonInputSource::from(source)))
        .collect();
    let deployed_libraries = libraries.as_paths();

    let mut solc_input = era_solc::StandardJsonInput::try_from_solidity_sources(
        sources,
        libraries.clone(),
        BTreeSet::new(),
        era_solc::StandardJsonInputOptimizer::default(),
        Some(solc_codegen),
        None,
        false,
        era_solc::StandardJsonInputSelection::new_required(solc_codegen),
        era_solc::StandardJsonInputMetadata::default(),
        vec![],
        vec![],
        vec![],
        false,
        false,
    )?;

    let mut solc_output = solc_compiler.standard_json(
        era_compiler_common::Target::EraVM,
        &mut solc_input,
        &mut vec![],
        None,
        vec![],
        None,
    )?;

    let project = Project::try_from_solc_output(
        libraries,
        solc_codegen,
        &mut solc_output,
        &solc_compiler,
        None,
    )?;

    let missing_libraries = project.get_missing_libraries(&deployed_libraries);
    missing_libraries.write_to_standard_json(&mut solc_output, Some(&solc_compiler.version));

    solc_output.check_errors()?;
    Ok(solc_output)
}

///
/// Builds the Yul `sources` and returns the standard JSON output.
///
pub fn build_yul(
    sources: BTreeMap<String, String>,
) -> anyhow::Result<era_solc::StandardJsonOutput> {
    self::setup()?;

    era_compiler_llvm_context::initialize_target(era_compiler_common::Target::EraVM);

    let optimizer_settings = era_compiler_llvm_context::OptimizerSettings::none();

    let sources = sources
        .into_iter()
        .map(|(path, source)| (path, era_solc::StandardJsonInputSource::from(source)))
        .collect();

    let mut solc_output = era_solc::StandardJsonOutput::new(&sources, &mut vec![]);

    let project = Project::try_from_yul_sources(
        sources,
        era_compiler_common::Libraries::default(),
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
    )?;
    build.check_errors()?;

    let build = build.link(BTreeMap::new());
    build.check_errors()?;

    build.write_to_standard_json(&mut solc_output, None)?;
    solc_output.check_errors()?;
    Ok(solc_output)
}

///
/// Builds the Yul standard JSON and returns the standard JSON output.
///
/// If `solc_compiler` is set, the standard JSON is validated with `solc`.
///
pub fn build_yul_standard_json(
    mut solc_input: era_solc::StandardJsonInput,
    solc_compiler: Option<&era_solc::Compiler>,
) -> anyhow::Result<era_solc::StandardJsonOutput> {
    self::setup()?;

    era_compiler_llvm_context::initialize_target(era_compiler_common::Target::EraVM);

    let optimizer_settings = era_compiler_llvm_context::OptimizerSettings::try_from_cli(
        solc_input.settings.optimizer.mode,
    )?;

    let (solc_version, mut solc_output) = match solc_compiler {
        Some(solc_compiler) => {
            let solc_output = solc_compiler.validate_yul_standard_json(
                era_compiler_common::Target::EraVM,
                &mut solc_input,
                &mut vec![],
            )?;
            (Some(&solc_compiler.version), solc_output)
        }
        None => (
            None,
            era_solc::StandardJsonOutput::new(&solc_input.sources, &mut vec![]),
        ),
    };

    let project = Project::try_from_yul_sources(
        solc_input.sources,
        era_compiler_common::Libraries::default(),
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
    )?;
    build.check_errors()?;

    let build = build.link(BTreeMap::new());
    build.check_errors()?;

    build.write_to_standard_json(&mut solc_output, solc_version)?;
    solc_output.check_errors()?;
    Ok(solc_output)
}

///
/// Builds the LLVM IR standard JSON and returns the standard JSON output.
///
pub fn build_llvm_ir_standard_json(
    input: era_solc::StandardJsonInput,
) -> anyhow::Result<era_solc::StandardJsonOutput> {
    self::setup()?;

    era_compiler_llvm_context::initialize_target(era_compiler_common::Target::EraVM);

    let optimizer_settings =
        era_compiler_llvm_context::OptimizerSettings::try_from_cli(input.settings.optimizer.mode)?;

    let mut output = era_solc::StandardJsonOutput::new(&BTreeMap::new(), &mut vec![]);

    let project = Project::try_from_llvm_ir_sources(
        input.sources,
        era_compiler_common::Libraries::default(),
        Some(&mut output),
    )?;
    let build = project.compile_to_eravm(
        &mut vec![],
        true,
        era_compiler_common::HashType::Ipfs,
        optimizer_settings,
        vec![],
        false,
        None,
    )?;
    build.check_errors()?;

    let build = build.link(BTreeMap::new());
    build.check_errors()?;

    build.write_to_standard_json(&mut output, None)?;
    output.check_errors()?;
    Ok(output)
}

///
/// Builds the EraVM assembly standard JSON and returns the standard JSON output.
///
pub fn build_eravm_assembly_standard_json(
    input: era_solc::StandardJsonInput,
) -> anyhow::Result<era_solc::StandardJsonOutput> {
    self::setup()?;

    era_compiler_llvm_context::initialize_target(era_compiler_common::Target::EraVM);

    let optimizer_settings =
        era_compiler_llvm_context::OptimizerSettings::try_from_cli(input.settings.optimizer.mode)?;

    let mut output = era_solc::StandardJsonOutput::new(&BTreeMap::new(), &mut vec![]);

    let project = Project::try_from_eravm_assembly_sources(input.sources, Some(&mut output))?;
    let build = project.compile_to_eravm(
        &mut vec![],
        true,
        era_compiler_common::HashType::Ipfs,
        optimizer_settings,
        vec![],
        false,
        None,
    )?;
    build.check_errors()?;

    let build = build.link(BTreeMap::new());
    build.check_errors()?;

    build.write_to_standard_json(&mut output, None)?;
    output.check_errors()?;
    Ok(output)
}

///
/// Checks if the built Solidity project contains the given warning.
///
pub fn check_solidity_message(
    source_code: &str,
    warning_substring: &str,
    libraries: era_compiler_common::Libraries,
    solc_version: &semver::Version,
    solc_codegen: era_solc::StandardJsonInputCodegen,
    suppressed_errors: Vec<era_solc::StandardJsonInputErrorType>,
    suppressed_warnings: Vec<era_solc::StandardJsonInputWarningType>,
) -> anyhow::Result<bool> {
    self::setup()?;

    let solc_compiler = get_solc_compiler(solc_version)?;

    let mut sources = BTreeMap::new();
    sources.insert(
        "test.sol".to_string(),
        era_solc::StandardJsonInputSource::from(source_code.to_string()),
    );

    let mut solc_input = era_solc::StandardJsonInput::try_from_solidity_sources(
        sources,
        libraries,
        BTreeSet::new(),
        era_solc::StandardJsonInputOptimizer::default(),
        Some(solc_codegen),
        None,
        false,
        era_solc::StandardJsonInputSelection::new_required(solc_codegen),
        era_solc::StandardJsonInputMetadata::default(),
        vec![],
        suppressed_errors,
        suppressed_warnings,
        false,
        false,
    )?;

    let solc_output = solc_compiler.standard_json(
        era_compiler_common::Target::EraVM,
        &mut solc_input,
        &mut vec![],
        None,
        vec![],
        None,
    )?;
    let contains_warning = solc_output
        .errors
        .iter()
        .any(|error| error.formatted_message.contains(warning_substring));

    Ok(contains_warning)
}

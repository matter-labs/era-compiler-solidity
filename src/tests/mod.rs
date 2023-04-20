//!
//! The Solidity compiler unit tests.
//!

#![cfg(test)]

mod libraries;
mod messages;
mod runtime_code;
mod unsupported_opcodes;

use std::collections::BTreeMap;

use crate::project::Project;
use crate::solc::pipeline::Pipeline as SolcPipeline;
use crate::solc::standard_json::input::settings::optimizer::Optimizer as SolcStandardJsonInputSettingsOptimizer;
use crate::solc::standard_json::input::settings::selection::Selection as SolcStandardJsonInputSettingsSelection;
use crate::solc::standard_json::input::Input as SolcStandardJsonInput;
use crate::solc::Compiler as SolcCompiler;

pub fn build_solidity(
    source_code: &str,
    libraries: BTreeMap<String, BTreeMap<String, String>>,
    pipeline: SolcPipeline,
) -> anyhow::Result<()> {
    inkwell::support::enable_llvm_pretty_stack_trace();
    compiler_llvm_context::initialize_target();
    let optimizer_settings = compiler_llvm_context::OptimizerSettings::none();

    let mut sources = BTreeMap::new();
    sources.insert("test.sol".to_string(), source_code.to_string());
    let input = SolcStandardJsonInput::try_from_sources(
        sources.clone(),
        libraries.clone(),
        SolcStandardJsonInputSettingsSelection::new_required(pipeline),
        SolcStandardJsonInputSettingsOptimizer::new(true, None),
        None,
        pipeline == SolcPipeline::Yul,
    )?;

    let solc = SolcCompiler::new("solc".to_owned());
    let mut output = solc.standard_json(input, pipeline, None, vec![], None)?;

    let project = output.try_to_project(
        sources,
        libraries,
        pipeline,
        &SolcCompiler::LAST_SUPPORTED_VERSION,
        None,
    )?;
    let _build = project.compile_all(
        compiler_llvm_context::TargetMachine::new(&optimizer_settings)?,
        optimizer_settings,
        false,
        false,
        None,
    )?;

    Ok(())
}

pub fn check_solidity_warning(
    source_code: &str,
    warning_substring: &str,
    libraries: BTreeMap<String, BTreeMap<String, String>>,
    pipeline: SolcPipeline,
) -> anyhow::Result<bool> {
    let mut sources = BTreeMap::new();
    sources.insert("test.sol".to_string(), source_code.to_string());
    let input = SolcStandardJsonInput::try_from_sources(
        sources.clone(),
        libraries.clone(),
        SolcStandardJsonInputSettingsSelection::new_required(pipeline),
        SolcStandardJsonInputSettingsOptimizer::new(true, None),
        None,
        pipeline == SolcPipeline::Yul,
    )?;

    let solc = SolcCompiler::new("solc".to_owned());
    let output = solc.standard_json(input, pipeline, None, vec![], None)?;
    let contains_warning = output
        .errors
        .ok_or_else(|| anyhow::anyhow!("Solidity compiler messages not found"))?
        .iter()
        .any(|error| error.formatted_message.contains(warning_substring));

    Ok(contains_warning)
}

pub fn build_yul(source_code: &str) -> anyhow::Result<()> {
    inkwell::support::enable_llvm_pretty_stack_trace();
    compiler_llvm_context::initialize_target();
    let optimizer_settings = compiler_llvm_context::OptimizerSettings::none();

    let project = Project::try_from_yul_string("test.yul", source_code)?;
    let _build = project.compile_all(
        compiler_llvm_context::TargetMachine::new(&optimizer_settings)?,
        optimizer_settings,
        false,
        false,
        None,
    )?;

    Ok(())
}

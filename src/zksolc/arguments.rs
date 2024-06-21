//!
//! Solidity to EraVM compiler arguments.
//!

use std::collections::BTreeSet;
use std::path::Path;
use std::path::PathBuf;

use path_slash::PathExt;
use structopt::StructOpt;

use era_compiler_solidity::SolcStandardJsonOutputError;

///
/// Compiles the provided Solidity input files (or use the standard input if no files
/// are given or "-" is specified as a file name). Outputs the components based on the
/// chosen options, either to the standard output or to files within the designated
/// output directory.
///
/// Example: zksolc ERC20.sol -O3 --bin --output-dir "./build/"
///
#[derive(Debug, StructOpt)]
#[structopt(
    name = "Solidity compiler for ZKsync",
    global_settings = &[structopt::clap::AppSettings::ArgRequiredElseHelp],
)]
pub struct Arguments {
    /// Print the version and exit.
    #[structopt(long = "version")]
    pub version: bool,

    /// Specify the input paths and remappings.
    /// If an argument contains a '=', it is considered a remapping.
    /// Multiple Solidity files can be passed in the default Solidity mode.
    /// Yul, LLVM IR, and EraVM Assembly modes currently support only a single file.
    pub inputs: Vec<String>,

    /// Set the given path as the root of the source tree instead of the root of the filesystem.
    /// Passed to `solc` without changes.
    #[structopt(long = "base-path")]
    pub base_path: Option<String>,

    /// Make an additional source directory available to the default import callback.
    /// Can be used multiple times. Can only be used if the base path has a non-empty value.
    /// Passed to `solc` without changes.
    #[structopt(long = "include-path")]
    pub include_paths: Vec<String>,

    /// Allow a given path for imports. A list of paths can be supplied by separating them with a comma.
    /// Passed to `solc` without changes.
    #[structopt(long = "allow-paths")]
    pub allow_paths: Option<String>,

    /// Create one file per component and contract/file at the specified directory, if given.
    #[structopt(short = "o", long = "output-dir")]
    pub output_directory: Option<PathBuf>,

    /// Overwrite existing files (used together with -o).
    #[structopt(long = "overwrite")]
    pub overwrite: bool,

    /// Set the optimization parameter -O[0 | 1 | 2 | 3 | s | z].
    /// Use `3` for best performance and `z` for minimal size.
    #[structopt(short = "O", long = "optimization")]
    pub optimization: Option<char>,

    /// Try to recompile with -Oz if the bytecode is too large.
    #[structopt(long = "fallback-Oz")]
    pub fallback_to_optimizing_for_size: bool,

    /// Pass arbitary space-separated options to LLVM.
    #[structopt(long = "llvm-options")]
    pub llvm_options: Option<String>,

    /// Disable the `solc` optimizer.
    /// Use it if your project uses the `MSIZE` instruction, or in other cases.
    /// Beware that it will prevent libraries from being inlined.
    #[structopt(long = "disable-solc-optimizer")]
    pub disable_solc_optimizer: bool,

    /// Specify the path to the `solc` executable. By default, the one in `${PATH}` is used.
    /// Yul mode: `solc` is used for source code validation, as `zksolc` itself assumes that the input Yul is valid.
    /// LLVM IR mode: `solc` is unused.
    #[structopt(long = "solc")]
    pub solc: Option<String>,

    /// The EVM target version to generate IR for.
    /// See https://github.com/matter-labs/era-compiler-common/blob/main/src/evm_version.rs for reference.
    #[structopt(long = "evm-version")]
    pub evm_version: Option<String>,

    /// Specify addresses of deployable libraries. Syntax: `<libraryName>=<address> [, or whitespace] ...`.
    /// Addresses are interpreted as hexadecimal strings prefixed with `0x`.
    #[structopt(short = "l", long = "libraries")]
    pub libraries: Vec<String>,

    /// Output a single JSON document containing the specified information.
    /// Available arguments: `abi`, `hashes`, `metadata`, `devdoc`, `userdoc`, `storage-layout`, `ast`, `asm`, `bin`, `bin-runtime`.
    #[structopt(long = "combined-json")]
    pub combined_json: Option<String>,

    /// Switch to standard JSON input/output mode. Read from stdin or specified file, write the result to stdout.
    /// This is the default used by the Hardhat plugin.
    #[structopt(long = "standard-json")]
    pub standard_json: Option<Option<String>>,

    /// Specify the target machine.
    /// Available arguments: `eravm`, `evm`.
    /// The default is `eravm`.
    #[structopt(long = "target")]
    pub target: Option<String>,

    /// Sets the number of threads, which execute the tests concurrently.
    #[structopt(short = "t", long = "threads")]
    pub threads: Option<usize>,

    /// Switch to missing deployable libraries detection mode.
    /// Only available for standard JSON input/output mode.
    /// Contracts are not compiled in this mode, and all compilation artifacts are not included.
    #[structopt(long = "detect-missing-libraries")]
    pub detect_missing_libraries: bool,

    /// Switch to Yul mode.
    /// Only one input Yul file is allowed.
    /// Cannot be used with combined and standard JSON modes.
    #[structopt(long = "yul")]
    pub yul: bool,

    /// Switch to LLVM IR mode.
    /// Only one input LLVM IR file is allowed.
    /// Cannot be used with combined and standard JSON modes.
    /// Use this mode at your own risk, as LLVM IR input validation is not implemented.
    #[structopt(long = "llvm-ir")]
    pub llvm_ir: bool,

    /// Switch to EraVM assembly mode.
    /// Only one input EraVM assembly file is allowed.
    /// Cannot be used with combined JSON modes.
    /// Use this mode at your own risk, as EraVM assembly input validation is not implemented.
    #[structopt(long = "eravm-assembly")]
    pub eravm_assembly: bool,

    /// Forcibly switch to EVM legacy assembly pipeline.
    /// It is useful for older revisions of `solc` 0.8, where Yul was considered highly experimental
    /// and contained more bugs than today.
    #[structopt(long = "force-evmla")]
    pub force_evmla: bool,

    /// Deprecated.
    /// Use `--enable-eravm-extensions` instead.
    #[structopt(long = "system-mode")]
    pub system_mode: bool,

    /// Enable EraVM extensions.
    /// In this mode, calls to addresses `0xFFFF` and below are substituted by special EraVM instructions.
    /// In the Yul mode, the `verbatim_*` instruction family becomes available.
    #[structopt(long = "enable-eravm-extensions")]
    pub enable_eravm_extensions: bool,

    /// Set metadata hash mode.
    /// The only supported value is `none` that disables appending the metadata hash.
    /// Is enabled by default.
    #[structopt(long = "metadata-hash")]
    pub metadata_hash: Option<String>,

    /// Sets the literal content flag for contract metadata.
    /// If enabled, the metadata will contain the literal content of the source files.
    #[structopt(long = "metadata-literal")]
    pub metadata_literal: bool,

    /// Output EraVM assembly of the contracts.
    #[structopt(long = "asm")]
    pub output_assembly: bool,

    /// Output EraVM bytecode of the contracts.
    #[structopt(long = "bin")]
    pub output_binary: bool,

    /// Suppress specified errors.
    /// Available arguments: `sendtransfer`.
    #[structopt(long = "suppress-errors")]
    pub suppressed_errors: Option<Vec<String>>,

    /// Suppress specified warnings.
    /// Available arguments: `txorigin`.
    #[structopt(long = "suppress-warnings")]
    pub suppressed_warnings: Option<Vec<String>>,

    /// Dump all IRs to files in the specified directory.
    /// Only for testing and debugging.
    #[structopt(long = "debug-output-dir")]
    pub debug_output_directory: Option<PathBuf>,

    /// Set the verify-each option in LLVM.
    /// Only for testing and debugging.
    #[structopt(long = "llvm-verify-each")]
    pub llvm_verify_each: bool,

    /// Set the debug-logging option in LLVM.
    /// Only for testing and debugging.
    #[structopt(long = "llvm-debug-logging")]
    pub llvm_debug_logging: bool,

    /// Run this process recursively and provide JSON input to compile a single contract.
    /// Only for usage from within the compiler.
    #[structopt(long = "recursive-process")]
    pub recursive_process: bool,
}

impl Default for Arguments {
    fn default() -> Self {
        Self::new()
    }
}

impl Arguments {
    ///
    /// A shortcut constructor.
    ///
    pub fn new() -> Self {
        Self::from_args()
    }

    ///
    /// Validates the arguments.
    ///
    pub fn validate(&self) -> Vec<SolcStandardJsonOutputError> {
        let mut messages = vec![];

        if self.system_mode {
            messages.push(SolcStandardJsonOutputError::new_warning(
                "The `--system-mode` flag is deprecated. Please use `--enable-eravm-extensions` instead.",
                None, None,
            ));
        }

        if self.version && std::env::args().count() > 2 {
            messages.push(SolcStandardJsonOutputError::new_error(
                "No other options are allowed while getting the compiler version.",
                None,
                None,
            ));
        }

        if self.recursive_process
            && std::env::args().count() > 2 + (self.target.is_some() as usize) * 2
        {
            messages.push(SolcStandardJsonOutputError::new_error(
                "No other options are allowed in recursive mode.",
                None,
                None,
            ));
        }

        let modes_count = [
            self.yul,
            self.llvm_ir,
            self.eravm_assembly,
            self.combined_json.is_some(),
            self.standard_json.is_some(),
        ]
        .iter()
        .filter(|&&x| x)
        .count();
        if modes_count > 1 {
            messages.push(SolcStandardJsonOutputError::new_error(
                "Only one mode is allowed at the same time: Yul, LLVM IR, EraVM assembly, combined JSON, standard JSON.", None, None));
        }

        if self.yul || self.llvm_ir || self.eravm_assembly {
            if self.base_path.is_some() {
                messages.push(SolcStandardJsonOutputError::new_error(
                    "`base-path` is not used in Yul, LLVM IR and EraVM assembly modes.",
                    None,
                    None,
                ));
            }
            if !self.include_paths.is_empty() {
                messages.push(SolcStandardJsonOutputError::new_error(
                    "`include-paths` is not used in Yul, LLVM IR and EraVM assembly modes.",
                    None,
                    None,
                ));
            }
            if self.allow_paths.is_some() {
                messages.push(SolcStandardJsonOutputError::new_error(
                    "`allow-paths` is not used in Yul, LLVM IR and EraVM assembly modes.",
                    None,
                    None,
                ));
            }

            if self.evm_version.is_some() {
                messages.push(SolcStandardJsonOutputError::new_error(
                    "`evm-version` is not used in Yul, LLVM IR and EraVM assembly modes.",
                    None,
                    None,
                ));
            }

            if self.force_evmla {
                messages.push(SolcStandardJsonOutputError::new_error(
                "EVM legacy assembly codegen is not supported in Yul, LLVM IR and EraVM assembly modes.", None, None));
            }

            if self.disable_solc_optimizer {
                messages.push(SolcStandardJsonOutputError::new_error(
                "Disabling the solc optimizer is not supported in Yul, LLVM IR and EraVM assembly modes.", None, None));
            }
        }

        if self.llvm_ir || self.eravm_assembly {
            if !self.libraries.is_empty() {
                messages.push(SolcStandardJsonOutputError::new_error(
                    "Libraries are not supported in LLVM IR and EraVM assembly modes.",
                    None,
                    None,
                ));
            }

            if self.solc.is_some() {
                messages.push(SolcStandardJsonOutputError::new_error(
                    "`solc` is not used in LLVM IR and EraVM assembly modes.",
                    None,
                    None,
                ));
            }

            if self.enable_eravm_extensions || self.system_mode {
                messages.push(SolcStandardJsonOutputError::new_error(
                    "EraVM extensions are not supported in LLVM IR and EraVM assembly modes.",
                    None,
                    None,
                ));
            }
            if self.detect_missing_libraries {
                messages.push(SolcStandardJsonOutputError::new_error(
                "Missing deployable libraries detection mode is not supported in LLVM IR and EraVM assembly modes.", None, None
                ));
            }
        }

        if self.eravm_assembly {
            if Some(era_compiler_llvm_context::Target::EVM.to_string()) == self.target {
                messages.push(SolcStandardJsonOutputError::new_error(
                    "EraVM assembly cannot be compiled to EVM bytecode.",
                    None,
                    None,
                ));
            }
            if self.optimization.is_some() {
                messages.push(SolcStandardJsonOutputError::new_error(
                    "LLVM optimizations are not supported in EraVM assembly mode.",
                    None,
                    None,
                ));
            }
            if self.fallback_to_optimizing_for_size {
                messages.push(SolcStandardJsonOutputError::new_error(
                    "Falling back to -Oz is not supported in EraVM assembly mode.",
                    None,
                    None,
                ));
            }
            if self.llvm_options.is_some() {
                messages.push(SolcStandardJsonOutputError::new_error(
                    "LLVM options are not supported in EraVM assembly mode.",
                    None,
                    None,
                ));
            }
        }

        if self.combined_json.is_some() && (self.output_assembly || self.output_binary) {
            messages.push(SolcStandardJsonOutputError::new_error(
                "Cannot output assembly or binary outside of JSON in combined JSON mode.",
                None,
                None,
            ));
        }

        if self.standard_json.is_none() && self.detect_missing_libraries {
            messages.push(SolcStandardJsonOutputError::new_error(
                "Missing deployable libraries detection mode is only supported in standard JSON mode.", None, None
            ));
        }

        if self.standard_json.is_some() {
            if self.output_assembly || self.output_binary {
                messages.push(SolcStandardJsonOutputError::new_error(
                    "Cannot output assembly or binary outside of JSON in standard JSON mode.",
                    None,
                    None,
                ));
            }

            if !self.inputs.is_empty() {
                messages.push(SolcStandardJsonOutputError::new_error(
                    "Input files must be passed via standard JSON input.",
                    None,
                    None,
                ));
            }
            if !self.libraries.is_empty() {
                messages.push(SolcStandardJsonOutputError::new_error(
                    "Libraries must be passed via standard JSON input.",
                    None,
                    None,
                ));
            }
            if self.evm_version.is_some() {
                messages.push(SolcStandardJsonOutputError::new_error(
                    "EVM version must be passed via standard JSON input.",
                    None,
                    None,
                ));
            }

            if self.output_directory.is_some() {
                messages.push(SolcStandardJsonOutputError::new_error(
                    "Output directory cannot be used in standard JSON mode.",
                    None,
                    None,
                ));
            }
            if self.overwrite {
                messages.push(SolcStandardJsonOutputError::new_error(
                    "Overwriting flag cannot be used in standard JSON mode.",
                    None,
                    None,
                ));
            }
            if self.disable_solc_optimizer {
                messages.push(SolcStandardJsonOutputError::new_error(
                "Disabling the solc optimizer must be specified in standard JSON input settings.", None, None
                ));
            }
            if self.optimization.is_some() {
                messages.push(SolcStandardJsonOutputError::new_error(
                    "LLVM optimizations must be specified in standard JSON input settings.",
                    None,
                    None,
                ));
            }
            if self.fallback_to_optimizing_for_size {
                messages.push(SolcStandardJsonOutputError::new_error(
                    "Falling back to -Oz must be specified in standard JSON input settings.",
                    None,
                    None,
                ));
            }
            if self.llvm_options.is_some() {
                messages.push(SolcStandardJsonOutputError::new_error(
                    "LLVM options must be specified in standard JSON input settings.",
                    None,
                    None,
                ));
            }
            if self.metadata_hash.is_some() {
                messages.push(SolcStandardJsonOutputError::new_error(
                    "Metadata hash mode must be specified in standard JSON input settings.",
                    None,
                    None,
                ));
            }
            if self.metadata_literal {
                messages.push(SolcStandardJsonOutputError::new_error(
                    "Metadata literal content must be specified in standard JSON input settings.",
                    None,
                    None,
                ));
            }

            if self.enable_eravm_extensions || self.system_mode {
                messages.push(SolcStandardJsonOutputError::new_warning(
                "EraVM extensions CLI flag `--enable-eravm-extensions` (`--system-mode`) is deprecated in standard JSON mode and must be passed in JSON as `settings.enableEraVMExtensions`.", None, None
                ));
            }
            if self.force_evmla {
                messages.push(SolcStandardJsonOutputError::new_warning(
                "EVM legacy assembly pipeline CLI flag `--force-evmla` is deprecated in standard JSON mode and must be passed in JSON as `settings.forceEVMLA`.", None, None
                ));
            }
            if self.detect_missing_libraries {
                messages.push(SolcStandardJsonOutputError::new_warning(
                "Missing deployable libraries detection mode CLI flag `--detect-missing-libraries` is deprecated in standard JSON mode and must be passed in JSON as `settings.detectMissingLibraries`.", None, None
                ));
            }
        }

        messages
    }

    ///
    /// Returns remappings from input paths.
    ///
    pub fn split_input_files_and_remappings(
        &self,
    ) -> anyhow::Result<(Vec<PathBuf>, Option<BTreeSet<String>>)> {
        let mut input_files = Vec::with_capacity(self.inputs.len());
        let mut remappings = BTreeSet::new();

        for input in self.inputs.iter() {
            if input.contains('=') {
                let mut parts = Vec::with_capacity(2);
                for path in input.trim().split('=') {
                    let path = PathBuf::from(path);
                    parts.push(
                        Self::path_to_posix(path.as_path())?
                            .to_string_lossy()
                            .to_string(),
                    );
                }
                if parts.len() != 2 {
                    anyhow::bail!(
                        "invalid remapping `{}`: expected two parts separated by '='",
                        input
                    );
                }
                remappings.insert(parts.join("="));
            } else {
                let path = PathBuf::from(input.trim());
                let path = Self::path_to_posix(path.as_path())?;
                input_files.push(path);
            }
        }

        let remappings = if remappings.is_empty() {
            None
        } else {
            Some(remappings)
        };

        Ok((input_files, remappings))
    }

    ///
    /// Normalizes an input path by converting it to POSIX format.
    ///
    fn path_to_posix(path: &Path) -> anyhow::Result<PathBuf> {
        let path = path
            .to_slash()
            .ok_or_else(|| anyhow::anyhow!("input path {:?} POSIX conversion error", path))?
            .to_string();
        let path = PathBuf::from(path.as_str());
        Ok(path)
    }
}

//!
//! Solidity to EraVM compiler arguments.
//!

use std::collections::BTreeSet;
use std::path::Path;
use std::path::PathBuf;

use clap::Parser;
use path_slash::PathExt;

///
/// Compiles the provided Solidity input files (or use the standard input if no files
/// are given or "-" is specified as a file name). Outputs the components based on the
/// chosen options, either to the standard output or to files within the designated
/// output directory.
///
/// Example: zksolc ERC20.sol -O3 --bin --output-dir "./build/"
///
#[derive(Debug, Parser)]
#[command(about, long_about = None)]
pub struct Arguments {
    /// Print the version and exit.
    #[arg(long)]
    pub version: bool,

    /// Specify the input paths and remappings.
    /// If an argument contains a '=', it is considered a remapping.
    /// Multiple Solidity files can be passed in the default Solidity mode.
    /// Yul, LLVM IR, and EraVM Assembly modes currently support only a single file.
    pub inputs: Vec<String>,

    /// Set the given path as the root of the source tree instead of the root of the filesystem.
    /// Passed to `solc` without changes.
    #[arg(long)]
    pub base_path: Option<String>,

    /// Make an additional source directory available to the default import callback.
    /// Can be used multiple times. Can only be used if the base path has a non-empty value.
    /// Passed to `solc` without changes.
    #[arg(long, num_args = 1..)]
    pub include_path: Vec<String>,

    /// Allow a given path for imports. A list of paths can be supplied by separating them with a comma.
    /// Passed to `solc` without changes.
    #[arg(long)]
    pub allow_paths: Option<String>,

    /// Create one file per component and contract/file at the specified directory, if given.
    #[arg(short, long)]
    pub output_dir: Option<PathBuf>,

    /// Overwrite existing files (used together with -o).
    #[arg(long = "overwrite")]
    pub overwrite: bool,

    /// Set the optimization parameter -O[0 | 1 | 2 | 3 | s | z].
    /// Use `3` for best performance and `z` for minimal size.
    #[arg(short = 'O', long)]
    pub optimization: Option<char>,

    /// Try to recompile with -Oz if the bytecode is too large.
    #[arg(long = "fallback-Oz")]
    pub fallback_to_optimizing_for_size: bool,

    /// Pass arbitary space-separated options to LLVM.
    /// The argument must be a single quoted string following a `=` separator.
    /// Example: `--llvm-options='-eravm-jump-table-density-threshold=10'`.
    #[arg(long)]
    pub llvm_options: Option<String>,

    /// Deprecated.
    /// The `solc` optimizer is not used by `zksolc` anymore.
    #[arg(long)]
    pub disable_solc_optimizer: bool,

    /// Specify the path to a `solc` executable.
    /// Solidity mode: if not provided, `solc` is also searched in `${PATH}`.
    /// Yul mode: `solc` is optional for additional Yul validation, as `zksolc` has limited Yul verification capabilities.
    /// LLVM IR and EraVM assembly modes: `solc` is unused.
    #[arg(long)]
    pub solc: Option<String>,

    /// EVM version `solc` will produce Yul or EVM assembly for.
    /// The default is chosen by `solc`.
    #[arg(long)]
    pub evm_version: Option<era_compiler_common::EVMVersion>,

    /// Specify addresses of deployable libraries. Syntax: `<libraryFullPath1>=<address1> ... <libraryFullPathN>=<addressN>`.
    /// Addresses are interpreted as hexadecimal strings prefixed with `0x`.
    #[arg(short, long, num_args = 1..)]
    pub libraries: Vec<String>,

    /// Output a single JSON document containing the specified information.
    /// Available arguments: `abi`, `hashes`, `metadata`, `devdoc`, `userdoc`, `storage-layout`, `ast`, `asm`, `bin`, `bin-runtime`.
    #[arg(long)]
    pub combined_json: Option<String>,

    /// Switch to standard JSON input/output mode. Read from stdin or specified file, write the result to stdout.
    /// This is the default used by the Hardhat plugin.
    #[arg(long)]
    pub standard_json: Option<Option<String>>,

    /// Specify the target machine.
    /// Available arguments: `eravm`, `evm`.
    /// The default is `eravm`.
    #[arg(long)]
    pub target: Option<String>,

    /// Sets the number of threads, where each thread compiles its own translation unit in a child process.
    #[arg(short, long)]
    pub threads: Option<usize>,

    /// Switch to missing deployable libraries detection mode.
    /// Only available for standard JSON input/output mode.
    /// Contracts are not compiled in this mode, and all compilation artifacts are not included.
    #[arg(long)]
    pub detect_missing_libraries: bool,

    /// Switch to Yul mode.
    /// Only one input Yul file is allowed.
    /// Cannot be used with combined and standard JSON modes.
    #[arg(long)]
    pub yul: bool,

    /// Switch to LLVM IR mode.
    /// Only one input LLVM IR file is allowed.
    /// Cannot be used with combined and standard JSON modes.
    /// Use this mode at your own risk, as LLVM IR input validation is not implemented.
    #[arg(long)]
    pub llvm_ir: bool,

    /// Switch to EraVM assembly mode.
    /// Only one input EraVM assembly file is allowed.
    /// Cannot be used with combined and standard JSON modes.
    /// Use this mode at your own risk, as EraVM assembly input validation is not implemented.
    #[arg(long)]
    pub eravm_assembly: bool,

    /// Specify the bytecode file to disassemble.
    /// Two file types are allowed: raw binary bytecode (*.zbin), and hexadecimal string (*.hex).
    /// Cannot be used with combined and standard JSON modes.
    #[arg(long)]
    pub disassemble: bool,

    /// Specify the bytecode file to link.
    /// In default mode, input bytecode files and `--libraries` are required, and the input files are modified in place.
    /// In standard JSON mode, the result of linking is returned via stdout in a JSON.
    #[arg(long)]
    pub link: bool,

    /// Specify the `solc` codegen.
    /// Available options: `evmla`, `yul`.
    #[arg(long)]
    pub codegen: Option<era_solc::StandardJsonInputCodegen>,

    /// Forcibly switch to EVM legacy assembly codegen.
    /// It is useful for older revisions of `solc` 0.8, where Yul was considered highly experimental
    /// and contained more bugs than today.
    /// Deprecated: use `--codegen` instead.
    #[arg(long)]
    pub force_evmla: bool,

    /// Deprecated: use `--enable-eravm-extensions` instead.
    #[arg(long)]
    pub system_mode: bool,

    /// Enable EraVM extensions.
    /// In this mode, calls to addresses `0xFFFF` and below are substituted by special EraVM instructions.
    /// In the Yul mode, the `verbatim_*` instruction family becomes available.
    #[arg(long)]
    pub enable_eravm_extensions: bool,

    /// Set the metadata hash type.
    /// Available types: `none`, `keccak256`, `ipfs`.
    /// The default is `keccak256`.
    #[arg(long)]
    pub metadata_hash: Option<era_compiler_common::HashType>,

    /// Sets the literal content flag for contract metadata.
    /// If enabled, the metadata will contain the literal content of the source files.
    #[arg(long)]
    pub metadata_literal: bool,

    /// Output assembly of the compiled contracts.
    #[arg(long = "asm")]
    pub output_assembly: bool,

    /// Output metadata of the compiled project.
    #[arg(long = "metadata")]
    pub output_metadata: bool,

    /// Output bytecode of the compiled contracts.
    #[arg(long = "bin")]
    pub output_binary: bool,

    /// Suppress specified errors.
    /// Available arguments: `sendtransfer`.
    #[arg(long, num_args = 1..)]
    pub suppress_errors: Option<Vec<String>>,

    /// Suppress specified warnings.
    /// Available arguments: `txorigin`.
    #[arg(long, num_args = 1..)]
    pub suppress_warnings: Option<Vec<String>>,

    /// Dump all IRs to files in the specified directory.
    /// Only for testing and debugging.
    #[arg(long)]
    pub debug_output_dir: Option<PathBuf>,

    /// Set the verify-each option in LLVM.
    /// Only for testing and debugging.
    #[arg(long)]
    pub llvm_verify_each: bool,

    /// Set the debug-logging option in LLVM.
    /// Only for testing and debugging.
    #[arg(long)]
    pub llvm_debug_logging: bool,

    /// Run this process recursively and provide JSON input to compile a single contract.
    /// Only for usage from within the compiler.
    #[arg(long)]
    pub recursive_process: bool,
}

impl Arguments {
    ///
    /// Validates the arguments.
    ///
    pub fn validate(&self) -> Vec<era_solc::StandardJsonOutputError> {
        let mut messages = vec![];

        if self.system_mode {
            messages.push(era_solc::StandardJsonOutputError::new_warning(
                "`--system-mode` flag is deprecated: please use `--enable-eravm-extensions` instead.",
                None, None,
            ));
        }
        if self.disable_solc_optimizer {
            messages.push(era_solc::StandardJsonOutputError::new_warning(
                "`--disable-solc-optimizer` flag is deprecated: the `solc` optimizer is not used by `zksolc` anymore.",
                None,
                None,
            ));
        }
        if self.force_evmla {
            messages.push(era_solc::StandardJsonOutputError::new_warning(
                "`--force-evmla` flag is deprecated: please use `--codegen 'evmla'` instead.",
                None,
                None,
            ));
        }

        if self.version && std::env::args().count() > 2 {
            messages.push(era_solc::StandardJsonOutputError::new_error(
                "No other options are allowed while getting the compiler version.",
                None,
                None,
            ));
        }

        if self.recursive_process
            && std::env::args().count() > 2 + (self.target.is_some() as usize) * 2
        {
            messages.push(era_solc::StandardJsonOutputError::new_error(
                "No other options are allowed in recursive mode.",
                None,
                None,
            ));
        }

        let modes_count = [
            self.yul,
            self.llvm_ir,
            self.eravm_assembly,
            self.disassemble,
            self.link,
            self.combined_json.is_some(),
            self.standard_json.is_some(),
        ]
        .iter()
        .filter(|&&x| x)
        .count();
        if modes_count > 1 + ((self.link && self.standard_json.is_some()) as usize) {
            messages.push(era_solc::StandardJsonOutputError::new_error(
                "Only one mode is allowed at the same time: Yul, LLVM IR, EraVM Assembly, disassembler, combined JSON, standard JSON. Only linker can be used with `--standard-json`.", None, None));
        }

        if self.yul || self.llvm_ir || self.eravm_assembly || self.disassemble || self.link {
            if self.base_path.is_some() {
                messages.push(era_solc::StandardJsonOutputError::new_error(
                    "`base-path` is only allowed in Solidity mode.",
                    None,
                    None,
                ));
            }
            if !self.include_path.is_empty() {
                messages.push(era_solc::StandardJsonOutputError::new_error(
                    "`include-path` is only allowed in Solidity mode.",
                    None,
                    None,
                ));
            }
            if self.allow_paths.is_some() {
                messages.push(era_solc::StandardJsonOutputError::new_error(
                    "`allow-paths` is only allowed in Solidity mode.",
                    None,
                    None,
                ));
            }

            if self.evm_version.is_some() {
                messages.push(era_solc::StandardJsonOutputError::new_error(
                    "EVM version is only allowed in Solidity mode.",
                    None,
                    None,
                ));
            }

            if self.force_evmla || self.codegen.is_some() {
                messages.push(era_solc::StandardJsonOutputError::new_error(
                    "Codegen settings are only available in Solidity mode.",
                    None,
                    None,
                ));
            }
        }

        if self.llvm_ir || self.eravm_assembly || self.disassemble || self.link {
            if self.solc.is_some() {
                messages.push(era_solc::StandardJsonOutputError::new_error(
                    "Using `solc` is only allowed in Solidity and Yul modes.",
                    None,
                    None,
                ));
            }

            if self.enable_eravm_extensions || self.system_mode {
                messages.push(era_solc::StandardJsonOutputError::new_error(
                    "EraVM extensions are only supported in Solidity and Yul modes.",
                    None,
                    None,
                ));
            }
        }

        if (self.llvm_ir || self.eravm_assembly || self.disassemble) && !self.libraries.is_empty() {
            messages.push(era_solc::StandardJsonOutputError::new_error(
                "Libraries are only supported in Solidity, Yul, and linker modes.",
                None,
                None,
            ));
        }

        if self.eravm_assembly {
            if Some(era_compiler_common::Target::EVM.to_string()) == self.target {
                messages.push(era_solc::StandardJsonOutputError::new_error(
                    "EraVM assembly cannot be compiled to EVM bytecode.",
                    None,
                    None,
                ));
            }

            if self.optimization.is_some() {
                messages.push(era_solc::StandardJsonOutputError::new_error(
                    "LLVM optimizations are not supported in EraVM assembly mode.",
                    None,
                    None,
                ));
            }
            if self.fallback_to_optimizing_for_size {
                messages.push(era_solc::StandardJsonOutputError::new_error(
                    "Falling back to -Oz is not supported in EraVM assembly mode.",
                    None,
                    None,
                ));
            }
        }

        if self.disassemble
            && std::env::args().count()
                > 2 + self.inputs.len() + (self.target.is_some() as usize) * 2
        {
            messages.push(era_solc::StandardJsonOutputError::new_error(
                "No other options except input files and `--target` are allowed in disassembler mode.",
                None,
                None,
            ));
        }

        let mut linker_default_arguments_count = 2 + (self.target.is_some() as usize) * 2;
        linker_default_arguments_count += match self.standard_json {
            Some(Some(_)) => 2,
            Some(None) => 1,
            _ => self.inputs.len() + ((!self.libraries.is_empty()) as usize) + self.libraries.len(),
        };
        if self.link && std::env::args().count() > linker_default_arguments_count {
            messages.push(era_solc::StandardJsonOutputError::new_error(
                "Error: No other options except bytecode files, `--libraries`, `--standard-json`, `--target` are allowed in linker mode.",
                None,
                None,
            ));
        }

        if self.combined_json.is_some()
            && (self.output_assembly || self.output_metadata || self.output_binary)
        {
            messages.push(era_solc::StandardJsonOutputError::new_error(
                "Cannot output data outside of JSON in combined JSON mode.",
                None,
                None,
            ));
        }

        if self.standard_json.is_none() && self.detect_missing_libraries {
            messages.push(era_solc::StandardJsonOutputError::new_error(
                "Missing deployable libraries detection mode is only supported in standard JSON mode.", None, None
            ));
        }

        if self.standard_json.is_some() {
            if self.output_assembly || self.output_metadata || self.output_binary {
                messages.push(era_solc::StandardJsonOutputError::new_error(
                    "Cannot output data outside of JSON in standard JSON mode.",
                    None,
                    None,
                ));
            }

            if !self.inputs.is_empty() {
                messages.push(era_solc::StandardJsonOutputError::new_error(
                    "Input files must be passed via standard JSON input.",
                    None,
                    None,
                ));
            }
            if !self.libraries.is_empty() {
                messages.push(era_solc::StandardJsonOutputError::new_error(
                    "Libraries must be passed via standard JSON input.",
                    None,
                    None,
                ));
            }

            if self.codegen.is_some() {
                messages.push(era_solc::StandardJsonOutputError::new_error(
                    "Codegen must be passed via standard JSON input.",
                    None,
                    None,
                ));
            }
            if self.evm_version.is_some() {
                messages.push(era_solc::StandardJsonOutputError::new_error(
                    "EVM version must be passed via standard JSON input.",
                    None,
                    None,
                ));
            }

            if self.output_dir.is_some() {
                messages.push(era_solc::StandardJsonOutputError::new_error(
                    "Output directory cannot be used in standard JSON mode.",
                    None,
                    None,
                ));
            }
            if self.overwrite {
                messages.push(era_solc::StandardJsonOutputError::new_error(
                    "Overwriting flag cannot be used in standard JSON mode.",
                    None,
                    None,
                ));
            }
            if self.optimization.is_some() {
                messages.push(era_solc::StandardJsonOutputError::new_error(
                    "LLVM optimizations must be specified in standard JSON input settings.",
                    None,
                    None,
                ));
            }
            if self.fallback_to_optimizing_for_size {
                messages.push(era_solc::StandardJsonOutputError::new_error(
                    "Falling back to -Oz must be specified in standard JSON input settings.",
                    None,
                    None,
                ));
            }
            if self.llvm_options.is_some() {
                messages.push(era_solc::StandardJsonOutputError::new_error(
                    "LLVM options must be specified in standard JSON input settings.",
                    None,
                    None,
                ));
            }
            if self.metadata_hash.is_some() {
                messages.push(era_solc::StandardJsonOutputError::new_error(
                    "Metadata hash mode must be specified in standard JSON input settings.",
                    None,
                    None,
                ));
            }
            if self.metadata_literal {
                messages.push(era_solc::StandardJsonOutputError::new_error(
                    "Metadata literal content flag must be specified in standard JSON input settings.",
                    None,
                    None,
                ));
            }

            if self.suppress_errors.is_some() {
                messages.push(era_solc::StandardJsonOutputError::new_error(
                    "Suppressed errors must be specified in standard JSON input settings.",
                    None,
                    None,
                ));
            }
            if self.suppress_warnings.is_some() {
                messages.push(era_solc::StandardJsonOutputError::new_error(
                    "Suppressed warnings must be specified in standard JSON input settings.",
                    None,
                    None,
                ));
            }

            if self.enable_eravm_extensions || self.system_mode {
                messages.push(era_solc::StandardJsonOutputError::new_warning(
                "EraVM extensions CLI flag `--enable-eravm-extensions` (`--system-mode`) is deprecated in standard JSON mode and must be passed in JSON as `settings.enableEraVMExtensions`.", None, None
                ));
            }
            if self.force_evmla {
                messages.push(era_solc::StandardJsonOutputError::new_warning(
                "EVM legacy assembly codegen CLI flag `--force-evmla` is deprecated in standard JSON mode and must be passed in JSON as `settings.forceEVMLA`.", None, None
                ));
            }
            if self.detect_missing_libraries {
                messages.push(era_solc::StandardJsonOutputError::new_warning(
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
    ) -> anyhow::Result<(Vec<PathBuf>, BTreeSet<String>)> {
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
                        "Invalid remapping `{}`: expected two parts separated by '='.",
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

        Ok((input_files, remappings))
    }

    ///
    /// Normalizes an input path by converting it to POSIX format.
    ///
    fn path_to_posix(path: &Path) -> anyhow::Result<PathBuf> {
        let path = path
            .to_slash()
            .ok_or_else(|| anyhow::anyhow!("Input path {:?} POSIX conversion error.", path))?
            .to_string();
        let path = PathBuf::from(path.as_str());
        Ok(path)
    }
}

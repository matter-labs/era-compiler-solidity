//!
//! Solidity to EraVM compiler arguments.
//!

use std::path::PathBuf;

use structopt::StructOpt;

///
/// Compiles the provided Solidity input files (or use the standard input if no files
/// are given or "-" is specified as a file name). Outputs the components based on the
/// chosen options, either to the standard output or to files within the designated
/// output directory.
///
/// Example: zksolc ERC20.sol -O3 --bin --output-dir './build/'
///
#[derive(Debug, StructOpt)]
#[structopt(name = "The EraVM Solidity compiler")]
pub struct Arguments {
    /// Print the version and exit.
    #[structopt(long = "version")]
    pub version: bool,

    /// Specify the input file paths.
    /// Multiple Solidity files can be passed in the default Solidity mode.
    /// Yul and LLVM IR modes currently support only a single file.
    #[structopt(parse(from_os_str))]
    pub input_files: Vec<PathBuf>,

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

    /// Specify addresses of deployable libraries. Syntax: `<libraryName>=<address> [, or whitespace] ...`.
    /// Addresses are interpreted as hexadecimal strings prefixed with `0x`.
    #[structopt(short = "l", long = "libraries")]
    pub libraries: Vec<String>,

    /// Output a single JSON document containing the specified information.
    /// Available arguments: `abi`, `hashes`, `metadata`, `devdoc`, `userdoc`, `storage-layout`, `ast`, `asm`, `bin`, `bin-runtime`.
    #[structopt(long = "combined-json")]
    pub combined_json: Option<String>,

    /// Switch to standard JSON input/output mode. Read from stdin, write the result to stdout.
    /// This is the default used by the Hardhat plugin.
    #[structopt(long = "standard-json")]
    pub standard_json: bool,

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
    /// Cannot be used with combined and standard JSON modes.
    /// Use this mode at your own risk, as EraVM assembly input validation is not implemented.
    #[structopt(long = "zkasm")]
    pub zkasm: bool,

    /// Forcibly switch to EVM legacy assembly pipeline.
    /// It is useful for older revisions of `solc` 0.8, where Yul was considered highly experimental
    /// and contained more bugs than today.
    #[structopt(long = "force-evmla")]
    pub force_evmla: bool,

    /// Enable system contract compilation mode.
    /// In this mode EraVM extensions are enabled. For example, calls to addresses `0xFFFF` and below
    /// are substituted by special EraVM instructions.
    /// In the Yul mode, the `verbatim_*` instruction family is available.
    #[structopt(long = "system-mode")]
    pub is_system_mode: bool,

    /// Set metadata hash mode.
    /// The only supported value is `none` that disables appending the metadata hash.
    /// Is enabled by default.
    #[structopt(long = "metadata-hash")]
    pub metadata_hash: Option<String>,

    /// Output EraVM assembly of the contracts.
    #[structopt(long = "asm")]
    pub output_assembly: bool,

    /// Output EraVM bytecode of the contracts.
    #[structopt(long = "bin")]
    pub output_binary: bool,

    /// Suppress specified warnings.
    /// Available arguments: `ecrecover`, `sendtransfer`, `extcodesize`, `txorigin`, `blocktimestamp`, `blocknumber`, `blockhash`.
    #[structopt(long = "suppress-warnings")]
    pub suppress_warnings: Option<Vec<String>>,

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
    #[allow(clippy::collapsible_if)]
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.version && std::env::args().count() > 2 {
            anyhow::bail!("No other options are allowed while getting the compiler version.");
        }

        if self.recursive_process && std::env::args().count() > 2 {
            anyhow::bail!("No other options are allowed in recursive mode.");
        }

        let modes_count = [
            self.yul,
            self.llvm_ir,
            self.zkasm,
            self.combined_json.is_some(),
            self.standard_json,
        ]
        .iter()
        .filter(|&&x| x)
        .count();
        if modes_count > 1 {
            anyhow::bail!("Only one modes is allowed at the same time: Yul, LLVM IR, EraVM assembly, combined JSON, standard JSON.");
        }

        if self.yul || self.llvm_ir || self.zkasm {
            if self.base_path.is_some() {
                anyhow::bail!("`base-path` is not used in Yul, LLVM IR and EraVM assembly modes.");
            }
            if !self.include_paths.is_empty() {
                anyhow::bail!(
                    "`include-paths` is not used in Yul, LLVM IR and EraVM assembly modes."
                );
            }
            if self.allow_paths.is_some() {
                anyhow::bail!(
                    "`allow-paths` is not used in Yul, LLVM IR and EraVM assembly modes."
                );
            }
            if !self.libraries.is_empty() {
                anyhow::bail!(
                    "Libraries are not supported in Yul, LLVM IR and EraVM assembly modes."
                );
            }

            if self.force_evmla {
                anyhow::bail!("EVM legacy assembly mode is not supported in Yul, LLVM IR and EraVM assembly modes.");
            }

            if self.disable_solc_optimizer {
                anyhow::bail!("Disabling the solc optimizer is not supported in Yul, LLVM IR and EraVM assembly modes.");
            }
        }

        if self.llvm_ir || self.zkasm {
            if self.solc.is_some() {
                anyhow::bail!("`solc` is not used in LLVM IR and EraVM assembly modes.");
            }

            if self.is_system_mode {
                anyhow::bail!(
                    "System contract mode is not supported in LLVM IR and EraVM assembly modes."
                );
            }
        }

        if self.zkasm {
            if self.optimization.is_some() {
                anyhow::bail!("LLVM optimizations are not supported in EraVM assembly mode.");
            }
        }

        if self.combined_json.is_some() {
            if self.output_assembly || self.output_binary {
                anyhow::bail!(
                    "Cannot output assembly or binary outside of JSON in combined JSON mode."
                );
            }
        }

        if self.standard_json {
            if self.output_assembly || self.output_binary {
                anyhow::bail!(
                    "Cannot output assembly or binary outside of JSON in standard JSON mode."
                );
            }

            if !self.input_files.is_empty() {
                anyhow::bail!("Input files must be passed via standard JSON input.");
            }
            if !self.libraries.is_empty() {
                anyhow::bail!("Libraries must be passed via standard JSON input.");
            }

            if self.output_directory.is_some() {
                anyhow::bail!("Output directory cannot be used in standard JSON mode.");
            }
            if self.overwrite {
                anyhow::bail!("Overwriting flag cannot be used in standard JSON mode.");
            }
            if self.disable_solc_optimizer {
                anyhow::bail!(
                    "Disabling the solc optimizer must specified in standard JSON input settings."
                );
            }
            if self.optimization.is_some() {
                anyhow::bail!("LLVM optimizations must specified in standard JSON input settings.");
            }
            if self.metadata_hash.is_some() {
                anyhow::bail!("Metadata hash mode must specified in standard JSON input settings.");
            }
        }

        Ok(())
    }
}

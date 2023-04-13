//!
//! Solidity to zkEVM compiler arguments.
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
#[structopt(name = "The zkEVM Solidity compiler")]
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
    /// This is the default used by the hardhat plugin.
    #[structopt(long = "standard-json")]
    pub standard_json: bool,

    /// Switch to the Yul mode.
    /// Only one input Yul file is allowed.
    /// Cannot be used with the combined and standard JSON modes.
    #[structopt(long = "yul")]
    pub yul: bool,

    /// Switch to the LLVM IR mode.
    /// Only one input LLVM IR file is allowed.
    /// Cannot be used with the combined and standard JSON modes.
    #[structopt(long = "llvm-ir")]
    pub llvm_ir: bool,

    /// Forcibly switch to the EVM legacy assembly pipeline.
    /// It is useful for older revisions of `solc` 0.8, where Yul was considered highly experimental
    /// and contained more bugs than today.
    #[structopt(long = "force-evmla")]
    pub force_evmla: bool,

    /// Enable the system contract compilation mode.
    /// In this mode zkEVM extensions are enabled. For example, calls to addresses `0xFFFF` and below
    /// are substituted by special zkEVM instructions.
    /// In the Yul mode, the `verbatim_*` instruction family is available.
    #[structopt(long = "system-mode")]
    pub is_system_mode: bool,

    /// Output zkEVM assembly of the contracts.
    #[structopt(long = "asm")]
    pub output_assembly: bool,

    /// Output zkEVM bytecode of the contracts.
    #[structopt(long = "bin")]
    pub output_binary: bool,

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
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.yul || self.llvm_ir {
            if self.combined_json.is_some() {
                anyhow::bail!("The `--combined-json` option is invalid in IR modes");
            }
            if self.standard_json {
                anyhow::bail!("The `--standard-json` option is invalid in IR modes");
            }
        }

        Ok(())
    }
}

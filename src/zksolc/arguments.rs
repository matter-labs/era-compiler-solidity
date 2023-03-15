//!
//! Solidity to zkEVM compiler arguments.
//!

use std::path::PathBuf;

use structopt::StructOpt;

///
/// Compiles the given Solidity input files (or the standard input if none given or
/// "-" is used as a file name) and outputs the components specified in the options
/// at standard output or in files in the output directory, if specified.
/// Imports are automatically read from the filesystem.
///
/// Example: zksolc ERC20.sol --bin --optimize --output-dir './build/'
///
#[derive(Debug, StructOpt)]
#[structopt(name = "The zkEVM Solidity compiler")]
pub struct Arguments {
    /// Print the version and exit.
    #[structopt(long = "version")]
    pub version: bool,

    /// The input file paths.
    #[structopt(parse(from_os_str))]
    pub input_files: Vec<PathBuf>,

    /// Use the given path as the root of the source tree
    /// instead of the root of the filesystem.
    #[structopt(long = "base-path")]
    pub base_path: Option<String>,

    /// Make an additional source directory available to the
    /// default import callback. Use this option if you want to
    /// import contracts whose location is not fixed in relation
    /// to your main source tree, e.g. third-party libraries
    /// installed using a package manager. Can be used multiple
    /// times. Can only be used if base path has a non-empty
    /// value.
    #[structopt(long = "include-path")]
    pub include_paths: Vec<String>,

    /// Allow a given path for imports. A list of paths can be
    /// supplied by separating them with a comma.
    #[structopt(long = "allow-paths")]
    pub allow_paths: Option<String>,

    /// If given, creates one file per component and
    /// contract/file at the specified directory.
    #[structopt(short = "o", long = "output-dir")]
    pub output_directory: Option<PathBuf>,

    /// Overwrite existing files (used together with -o).
    #[structopt(long = "overwrite")]
    pub overwrite: bool,

    /// Sets the optimization parameter -O[0 | 1 | 2 | 3 | s | z].
    #[structopt(short = "O", long = "optimization")]
    pub optimization: Option<char>,

    /// Path to the `solc` executable. By default, the one in `${PATH}` is used.
    #[structopt(long = "solc")]
    pub solc: Option<String>,

    /// Direct string or file containing library addresses.
    /// Syntax: <libraryName>=<address> [, or whitespace] ...
    /// Address is interpreted as a hex string prefixed by 0x.
    #[structopt(short = "l", long = "libraries")]
    pub libraries: Vec<String>,

    /// Output a single json document containing the specified information.
    /// Available arguments: abi, hashes
    /// Example: solc --combined-json abi,hashes
    #[structopt(long = "combined-json")]
    pub combined_json: Option<String>,

    /// Switch to Standard JSON input / output mode.
    /// Reads from stdin, result is written to stdout.
    #[structopt(long = "standard-json")]
    pub standard_json: bool,

    /// Switch to the Yul mode.
    #[structopt(long = "yul")]
    pub yul: bool,

    /// Switch to the LLVM IR mode.
    #[structopt(long = "llvm-ir")]
    pub llvm_ir: bool,

    /// Sets the EVM legacy assembly pipeline forcibly.
    #[structopt(long = "force-evmla")]
    pub force_evmla: bool,

    /// Enables the system contract compilation mode.
    #[structopt(long = "system-mode")]
    pub is_system_mode: bool,

    /// Output zkEVM assembly of the contracts.
    #[structopt(long = "asm")]
    pub output_assembly: bool,

    /// Output zkEVM bytecode of the contracts.
    #[structopt(long = "bin")]
    pub output_binary: bool,

    /// Dump all IRs to files in the specified directory.
    #[structopt(long = "debug-output-dir")]
    pub debug_output_directory: Option<PathBuf>,

    /// Sets the `verify each` option in LLVM.
    #[structopt(long = "llvm-verify-each")]
    pub llvm_verify_each: bool,

    /// Sets the `debug logging` option in LLVM.
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

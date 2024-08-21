//!
//! Yul to EasyCrypt translation
//!

use std::path::Path;
use std::path::PathBuf;

use path_slash::PathExt;
use structopt::StructOpt;

/// Describes the accepted command line arguments.
#[derive(Debug, StructOpt)]
#[structopt(name = "The EraVM Yul to CoreYul transpiler")]
pub struct Arguments {
    /// Print the version and exit.
    #[structopt(long = "version")]
    pub version: bool,

    /// Input file name
    pub inputs: Vec<String>,

    /// Create one file per component and contract/file at the specified directory, if given.
    #[structopt(short = "o", long = "output-dir")]
    pub output_directory: Option<PathBuf>,

    /// Suppress specified warnings.
    /// Available arguments: `ecrecover`, `sendtransfer`, `extcodesize`, `txorigin`, `blocktimestamp`, `blocknumber`, `blockhash`.
    #[structopt(long = "suppress-warnings")]
    pub suppress_warnings: Option<Vec<String>>,

    /// Output the simplified Core Yul assembly if possible; it can be translated to EasyCrypt.
    #[structopt(long = "output-core-yul")]
    pub output_core_yul: bool,
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
            anyhow::bail!("No other options are allowed while getting the tool version.");
        }
        Ok(())
    }

    /// Returns the YUL input files paths provided by user
    pub fn input_files_paths(&self) -> anyhow::Result<Vec<PathBuf>> {
        let mut input_files = Vec::with_capacity(self.inputs.len());

        for input in self.inputs.iter() {
            let path = PathBuf::from(input.trim());
            let path = Self::path_to_posix(path.as_path())?;
            input_files.push(path);
        }

        Ok(input_files)
    }

    ///
    /// Normalizes an input path by converting it to POSIX format.
    ///
    fn path_to_posix(path: &Path) -> anyhow::Result<PathBuf> {
        let path = path
            .to_slash()
            .ok_or_else(|| anyhow::anyhow!("Input path {:?} POSIX conversion error", path))?
            .to_string();
        let path = PathBuf::from(path.as_str());
        Ok(path)
    }
}

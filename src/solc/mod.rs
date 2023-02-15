//!
//! The Solidity compiler.
//!

pub mod combined_json;
pub mod pipeline;
pub mod standard_json;
pub mod version;

use std::io::Write;
use std::path::PathBuf;

use self::combined_json::CombinedJson;
use self::standard_json::input::Input as StandardJsonInput;
use self::standard_json::output::Output as StandardJsonOutput;
use self::version::Version;

///
/// The Solidity compiler.
///
pub struct Compiler {
    /// The binary executable name.
    pub executable: String,
}

impl Compiler {
    /// The default executable name.
    pub const DEFAULT_EXECUTABLE_NAME: &'static str = "solc";

    /// The first supported version of `solc`.
    pub const FIRST_SUPPORTED_VERSION: semver::Version = semver::Version::new(0, 4, 10);

    /// The last supported version of `solc`.
    pub const LAST_SUPPORTED_VERSION: semver::Version = semver::Version::new(0, 8, 18);

    /// The first version of `solc`, where Yul is used by default.
    pub const FIRST_YUL_VERSION: semver::Version = semver::Version::new(0, 8, 0);

    ///
    /// A shortcut constructor.
    ///
    /// Different tools may use different `executable` names. For example, the integration tester
    /// uses `solc-<version>` format.
    ///
    pub fn new(executable: String) -> Self {
        Self { executable }
    }

    ///
    /// Compiles the Solidity `--standard-json` input into Yul IR.
    ///
    pub fn standard_json(
        &self,
        input: StandardJsonInput,
        base_path: Option<String>,
        include_paths: Vec<String>,
        allow_paths: Option<String>,
    ) -> anyhow::Result<StandardJsonOutput> {
        let mut command = std::process::Command::new(self.executable.as_str());
        command.stdin(std::process::Stdio::piped());
        command.stdout(std::process::Stdio::piped());
        command.arg("--standard-json");

        if let Some(base_path) = base_path {
            command.arg("--base-path");
            command.arg(base_path);
        }
        for include_path in include_paths.into_iter() {
            command.arg("--include-path");
            command.arg(include_path);
        }
        if let Some(allow_paths) = allow_paths {
            command.arg("--allow-paths");
            command.arg(allow_paths);
        }

        let input_json = serde_json::to_vec(&input).expect("Always valid");

        let process = command.spawn().map_err(|error| {
            anyhow::anyhow!("{} subprocess spawning error: {:?}", self.executable, error)
        })?;
        process
            .stdin
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("{} stdin getting error", self.executable))?
            .write_all(input_json.as_slice())
            .map_err(|error| {
                anyhow::anyhow!("{} stdin writing error: {:?}", self.executable, error)
            })?;

        let output = process.wait_with_output().map_err(|error| {
            anyhow::anyhow!("{} subprocess output error: {:?}", self.executable, error)
        })?;
        if !output.status.success() {
            anyhow::bail!(
                "{} error: {}",
                self.executable,
                String::from_utf8_lossy(output.stderr.as_slice()).to_string()
            );
        }

        let mut output: StandardJsonOutput = serde_json::from_slice(output.stdout.as_slice())
            .map_err(|error| {
                anyhow::anyhow!(
                    "{} subprocess output parsing error: {}\n{}",
                    self.executable,
                    error,
                    serde_json::from_slice::<serde_json::Value>(output.stdout.as_slice())
                        .map(|json| serde_json::to_string_pretty(&json).expect("Always valid"))
                        .unwrap_or_else(
                            |_| String::from_utf8_lossy(output.stdout.as_slice()).to_string()
                        ),
                )
            })?;
        output.preprocess_ast()?;

        Ok(output)
    }

    ///
    /// The `solc --combined-json abi,hashes...` mirror.
    ///
    pub fn combined_json(
        &self,
        paths: &[PathBuf],
        combined_json_argument: &str,
    ) -> anyhow::Result<CombinedJson> {
        let mut command = std::process::Command::new(self.executable.as_str());
        command.args(paths);
        command.arg("--combined-json");
        command.arg(combined_json_argument);
        let output = command.output().map_err(|error| {
            anyhow::anyhow!("{} subprocess error: {:?}", self.executable, error)
        })?;
        if !output.status.success() {
            println!("{}", String::from_utf8_lossy(output.stdout.as_slice()));
            println!("{}", String::from_utf8_lossy(output.stderr.as_slice()));
            anyhow::bail!(
                "{} error: {}",
                self.executable,
                String::from_utf8_lossy(output.stdout.as_slice()).to_string()
            );
        }

        let combined_json = serde_json::from_slice(output.stdout.as_slice()).map_err(|error| {
            anyhow::anyhow!(
                "{} subprocess output parsing error: {}\n{}",
                self.executable,
                error,
                serde_json::from_slice::<serde_json::Value>(output.stdout.as_slice())
                    .map(|json| serde_json::to_string_pretty(&json).expect("Always valid"))
                    .unwrap_or_else(
                        |_| String::from_utf8_lossy(output.stdout.as_slice()).to_string()
                    ),
            )
        })?;

        Ok(combined_json)
    }

    ///
    /// The `solc --abi --hashes ...` mirror.
    ///
    pub fn extra_output(
        &self,
        paths: &[PathBuf],
        output_abi: bool,
        output_hashes: bool,
    ) -> anyhow::Result<String> {
        let mut command = std::process::Command::new(self.executable.as_str());
        command.args(paths);
        if output_abi {
            command.arg("--abi");
        }
        if output_hashes {
            command.arg("--hashes");
        }
        let output = command.output().map_err(|error| {
            anyhow::anyhow!("{} subprocess error: {:?}", self.executable, error)
        })?;
        if !output.status.success() {
            anyhow::bail!(
                "{} error: {}",
                self.executable,
                String::from_utf8_lossy(output.stderr.as_slice()).to_string()
            );
        }

        Ok(String::from_utf8_lossy(output.stdout.as_slice()).to_string())
    }

    ///
    /// The `solc --version` mini-parser.
    ///
    pub fn version(&self) -> anyhow::Result<Version> {
        let mut command = std::process::Command::new(self.executable.as_str());
        command.arg("--version");
        let output = command.output().map_err(|error| {
            anyhow::anyhow!("{} subprocess error: {:?}", self.executable, error)
        })?;
        if !output.status.success() {
            anyhow::bail!(
                "{} error: {}",
                self.executable,
                String::from_utf8_lossy(output.stderr.as_slice()).to_string()
            );
        }

        let stdout = String::from_utf8_lossy(output.stdout.as_slice());
        let long = stdout
            .lines()
            .nth(1)
            .ok_or_else(|| {
                anyhow::anyhow!("{} version parsing: not enough lines", self.executable)
            })?
            .split(' ')
            .nth(1)
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "{} version parsing: not enough words in the 2nd line",
                    self.executable
                )
            })?
            .to_owned();
        let default: semver::Version = long
            .split('+')
            .next()
            .ok_or_else(|| {
                anyhow::anyhow!("{} version parsing: metadata dropping", self.executable)
            })?
            .parse()
            .map_err(|error| anyhow::anyhow!("{} version parsing: {}", self.executable, error))?;

        let version = Version::new(long, default);
        if version.default < Self::FIRST_SUPPORTED_VERSION {
            anyhow::bail!(
                "`solc` versions <{} are not supported, found {}",
                Self::FIRST_SUPPORTED_VERSION,
                version.default
            );
        }
        if version.default > Self::LAST_SUPPORTED_VERSION {
            anyhow::bail!(
                "`solc` versions >{} are not supported, found {}",
                Self::LAST_SUPPORTED_VERSION,
                version.default
            );
        }

        Ok(version)
    }
}

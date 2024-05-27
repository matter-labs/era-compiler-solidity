//!
//! The Solidity compiler.
//!

pub mod combined_json;
pub mod pipeline;
pub mod standard_json;
pub mod version;

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::OnceLock;
use std::sync::RwLock;

use self::combined_json::CombinedJson;
use self::pipeline::Pipeline;
use self::standard_json::input::settings::optimizer::Optimizer as StandardJsonInputSettingsOptimizer;
use self::standard_json::input::Input as StandardJsonInput;
use self::standard_json::output::Output as StandardJsonOutput;
use self::version::Version;

///
/// The Solidity compiler.
///
#[derive(Debug, Clone)]
pub struct Compiler {
    /// The binary executable name.
    pub executable: String,
    /// The `solc` compiler version.
    pub version: Version,
}

impl Compiler {
    /// The default executable name.
    pub const DEFAULT_EXECUTABLE_NAME: &'static str = "solc";

    /// The first version of `solc` with the support of standard JSON interface.
    pub const FIRST_SUPPORTED_VERSION: semver::Version = semver::Version::new(0, 4, 12);

    /// The first version of `solc`, where Yul codegen is considered robust enough.
    pub const FIRST_YUL_VERSION: semver::Version = semver::Version::new(0, 8, 0);

    /// The first version of `solc`, where `--via-ir` codegen mode is supported.
    pub const FIRST_VIA_IR_VERSION: semver::Version = semver::Version::new(0, 8, 13);

    /// The first version of `solc`, where EVM Cancun is supported.
    pub const FIRST_CANCUN_VERSION: semver::Version = semver::Version::new(0, 8, 24);

    /// The last supported version of `solc`.
    pub const LAST_SUPPORTED_VERSION: semver::Version = semver::Version::new(0, 8, 25);

    ///
    /// A shortcut constructor lazily using a thread-safe cell.
    ///
    /// Different tools may use different `executable` names. For example, the integration tester
    /// uses `solc-<version>` format.
    ///
    pub fn new(executable: &str) -> anyhow::Result<Self> {
        if let Some(executable) = Self::executables()
            .read()
            .expect("Sync")
            .get(executable)
            .cloned()
        {
            return Ok(executable);
        }
        let mut executables = Self::executables().write().expect("Sync");

        if let Err(error) = which::which(executable) {
            anyhow::bail!(
                "The `{executable}` executable not found in ${{PATH}}: {}",
                error
            );
        }
        let version = Self::parse_version(executable)?;
        let compiler = Self {
            executable: executable.to_owned(),
            version,
        };

        executables.insert(executable.to_owned(), compiler.clone());
        Ok(compiler)
    }

    ///
    /// The Solidity `--standard-json` mirror.
    ///
    pub fn standard_json(
        &self,
        mut input: StandardJsonInput,
        pipeline: Option<Pipeline>,
        base_path: Option<String>,
        include_paths: Vec<String>,
        allow_paths: Option<String>,
    ) -> anyhow::Result<StandardJsonOutput> {
        let executable = self.executable.to_owned();
        let suppressed_warnings = input.suppressed_warnings.take().unwrap_or_default();

        let mut command = std::process::Command::new(executable.as_str());
        command.stdin(std::process::Stdio::piped());
        command.stdout(std::process::Stdio::piped());
        command.stderr(std::process::Stdio::piped());
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

        let mut process = command.spawn().map_err(|error| {
            anyhow::anyhow!("{} subprocess spawning error: {:?}", executable, error)
        })?;
        let stdin = process
            .stdin
            .take()
            .ok_or_else(|| anyhow::anyhow!("{:?} subprocess stdin getting error", executable))?;
        let stdin_thread = std::thread::spawn(move || serde_json::to_writer(stdin, &input));

        let stdout = process
            .stdout
            .take()
            .ok_or_else(|| anyhow::anyhow!("{:?} subprocess stdout getting error", executable))?;
        let stdout_thread = std::thread::spawn(|| {
            era_compiler_common::deserialize_from_reader::<_, StandardJsonOutput>(stdout)
        });

        let stderr = process
            .stderr
            .take()
            .ok_or_else(|| anyhow::anyhow!("{:?} subprocess stderr getting error", executable))?;
        let stderr_thread = std::thread::spawn(|| std::io::read_to_string(stderr));

        let status = process.wait().map_err(|error| {
            anyhow::anyhow!("{executable} subprocess status reading error: {error:?}")
        })?;
        stdin_thread
            .join()
            .expect("Thread error")
            .map_err(|error| {
                anyhow::anyhow!("{executable} subprocess stdin writing error: {error:?}")
            })?;
        let stderr_message = stderr_thread
            .join()
            .expect("Thread error")
            .map_err(|error| {
                anyhow::anyhow!("{executable} subprocess stderr reading error: {error:?}")
            })?;
        let mut solc_output = stdout_thread
            .join()
            .expect("Thread error")
            .map_err(|error| {
                anyhow::anyhow!(
                    "{} subprocess stdout parsing error: {error:?} (stderr: {stderr_message})",
                    executable
                )
            })?;
        if !status.success() {
            anyhow::bail!("{} error: {}", executable, stderr_message);
        }

        if let Some(pipeline) = pipeline {
            solc_output.preprocess_ast(&self.version, pipeline, suppressed_warnings.as_slice())?;
        }
        solc_output.remove_evm();

        Ok(solc_output)
    }

    ///
    /// The `solc --combined-json abi,hashes...` mirror.
    ///
    pub fn combined_json(
        &self,
        paths: &[PathBuf],
        combined_json_argument: &str,
    ) -> anyhow::Result<CombinedJson> {
        let executable = self.executable.to_owned();

        let mut command = std::process::Command::new(executable.as_str());
        command.stdout(std::process::Stdio::piped());
        command.stderr(std::process::Stdio::piped());
        command.args(paths);

        let mut combined_json_flags = Vec::new();
        let mut combined_json_fake_flag_pushed = false;
        let mut filtered_flags = Vec::with_capacity(3);
        for flag in combined_json_argument.split(',') {
            match flag {
                flag @ "asm" | flag @ "bin" | flag @ "bin-runtime" => filtered_flags.push(flag),
                flag => combined_json_flags.push(flag),
            }
        }
        if combined_json_flags.is_empty() {
            combined_json_flags.push("ast");
            combined_json_fake_flag_pushed = true;
        }
        command.arg("--combined-json");
        command.arg(combined_json_flags.join(","));

        let mut process = command.spawn().map_err(|error| {
            anyhow::anyhow!("{} subprocess spawning error: {:?}", executable, error)
        })?;

        let stdout = process
            .stdout
            .take()
            .ok_or_else(|| anyhow::anyhow!("{:?} subprocess stdout getting error", executable))?;
        let stdout_thread = std::thread::spawn(|| {
            era_compiler_common::deserialize_from_reader::<_, CombinedJson>(stdout)
        });

        let stderr = process
            .stderr
            .take()
            .ok_or_else(|| anyhow::anyhow!("{:?} subprocess stderr getting error", executable))?;
        let stderr_thread = std::thread::spawn(|| std::io::read_to_string(stderr));

        let status = process.wait().map_err(|error| {
            anyhow::anyhow!("{executable} subprocess status reading error: {error:?}")
        })?;
        let stderr_message = stderr_thread
            .join()
            .expect("Thread error")
            .map_err(|error| {
                anyhow::anyhow!("{executable} subprocess stderr reading error: {error:?}")
            })?;
        let mut combined_json = stdout_thread
            .join()
            .expect("Thread error")
            .map_err(|error| {
                anyhow::anyhow!(
                    "{} subprocess stdout parsing error: {error:?} (stderr: {stderr_message})",
                    executable
                )
            })?;
        if !status.success() {
            anyhow::bail!("{executable} error: {stderr_message}");
        }

        for filtered_flag in filtered_flags.into_iter() {
            for (_path, contract) in combined_json.contracts.iter_mut() {
                match filtered_flag {
                    "asm" => contract.asm = Some(serde_json::Value::Null),
                    "bin" => contract.bin = Some("".to_owned()),
                    "bin-runtime" => contract.bin_runtime = Some("".to_owned()),
                    _ => continue,
                }
            }
        }
        if combined_json_fake_flag_pushed {
            combined_json.source_list = None;
            combined_json.sources = None;
        }
        combined_json.remove_evm();

        Ok(combined_json)
    }

    ///
    /// Validates the Yul project as paths and libraries.
    ///
    pub fn validate_yul_paths(
        &self,
        paths: &[PathBuf],
        libraries: BTreeMap<String, BTreeMap<String, String>>,
    ) -> anyhow::Result<StandardJsonOutput> {
        if self.version.default != Self::LAST_SUPPORTED_VERSION {
            anyhow::bail!(
                "Yul validation is only supported with the latest supported version of the Solidity compiler: {}",
                Self::LAST_SUPPORTED_VERSION,
            );
        }

        let solc_input = StandardJsonInput::from_yul_paths(
            paths,
            libraries.clone(),
            StandardJsonInputSettingsOptimizer::new_yul_validation(),
        );
        self.validate_yul_standard_json(solc_input)
    }

    ///
    /// Validates the Yul project as standard JSON input.
    ///
    pub fn validate_yul_standard_json(
        &self,
        mut solc_input: StandardJsonInput,
    ) -> anyhow::Result<StandardJsonOutput> {
        if self.version.default != Self::LAST_SUPPORTED_VERSION {
            anyhow::bail!(
                "Yul validation is only supported with the latest supported version of the Solidity compiler: {}",
                Self::LAST_SUPPORTED_VERSION,
            );
        }

        solc_input.normalize_yul_validation();
        let solc_output = self.standard_json(solc_input, None, None, vec![], None)?;
        if solc_output.contracts.is_none() {
            anyhow::bail!(
                "{}",
                solc_output
                    .errors
                    .as_ref()
                    .map(|errors| serde_json::to_string_pretty(errors).expect("Always valid"))
                    .unwrap_or_else(|| {
                        "Unknown Yul validation error: both `contracts` and `errors` are unset"
                            .to_owned()
                    })
            );
        }
        Ok(solc_output)
    }

    ///
    /// Returns the global shared array of `solc` executables.
    ///
    fn executables() -> &'static RwLock<HashMap<String, Self>> {
        static EXECUTABLES: OnceLock<RwLock<HashMap<String, Compiler>>> = OnceLock::new();
        EXECUTABLES.get_or_init(|| RwLock::new(HashMap::new()))
    }

    ///
    /// The `solc --version` mini-parser.
    ///
    fn parse_version(executable: &str) -> anyhow::Result<Version> {
        let mut command = std::process::Command::new(executable);
        command.arg("--version");
        let output = command
            .output()
            .map_err(|error| anyhow::anyhow!("{} subprocess error: {:?}", executable, error))?;
        if !output.status.success() {
            anyhow::bail!(
                "{} error: {}",
                executable,
                String::from_utf8_lossy(output.stderr.as_slice()).to_string()
            );
        }

        let stdout = String::from_utf8_lossy(output.stdout.as_slice());
        let long = stdout
            .lines()
            .nth(1)
            .ok_or_else(|| anyhow::anyhow!("{} version parsing: not enough lines", executable))?
            .split(' ')
            .nth(1)
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "{} version parsing: not enough words in the 2nd line",
                    executable
                )
            })?
            .to_owned();
        let default: semver::Version = long
            .split('+')
            .next()
            .ok_or_else(|| anyhow::anyhow!("{} version parsing: metadata dropping", executable))?
            .parse()
            .map_err(|error| anyhow::anyhow!("{} version parsing: {}", executable, error))?;

        let l2_revision: Option<semver::Version> = stdout
            .lines()
            .nth(2)
            .and_then(|line| line.split(' ').nth(1))
            .and_then(|line| line.split('-').last())
            .and_then(|version| version.parse().ok());

        let version = Version::new(long, default, l2_revision);
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

//!
//! The Solidity compiler.
//!

pub mod combined_json;
pub mod pipeline;
pub mod standard_json;
pub mod version;

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;
use std::sync::OnceLock;
use std::sync::RwLock;

use self::combined_json::CombinedJson;
use self::pipeline::Pipeline;
use self::standard_json::input::settings::optimizer::Optimizer as StandardJsonInputSettingsOptimizer;
use self::standard_json::input::Input as StandardJsonInput;
use self::standard_json::output::error::Error as StandardJsonOutputError;
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

    /// The first version of `solc`, where `--via-ir` codegen option is supported.
    pub const FIRST_VIA_IR_VERSION: semver::Version = semver::Version::new(0, 8, 13);

    /// The first version of `solc`, where EVM Cancun is supported.
    pub const FIRST_CANCUN_VERSION: semver::Version = semver::Version::new(0, 8, 24);

    /// The last supported version of `solc`.
    pub const LAST_SUPPORTED_VERSION: semver::Version = semver::Version::new(0, 8, 26);

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
            anyhow::bail!("The `{executable}` executable not found in ${{PATH}}: {error}");
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
        input: &mut StandardJsonInput,
        pipeline: Option<Pipeline>,
        messages: &mut Vec<StandardJsonOutputError>,
        base_path: Option<String>,
        include_paths: Vec<String>,
        allow_paths: Option<String>,
    ) -> anyhow::Result<StandardJsonOutput> {
        let mut command = std::process::Command::new(self.executable.as_str());
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
            anyhow::anyhow!("{} subprocess spawning: {:?}", self.executable, error)
        })?;
        let stdin = process
            .stdin
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("{} subprocess stdin getting error", self.executable))?;
        let stdin_input = serde_json::to_vec(&input).map_err(|error| {
            anyhow::anyhow!(
                "{} subprocess standard JSON input serialization: {error:?}",
                self.executable
            )
        })?;
        stdin.write_all(stdin_input.as_slice()).map_err(|error| {
            anyhow::anyhow!("{} subprocess stdin writing: {error:?}", self.executable)
        })?;

        let result = process.wait_with_output().map_err(|error| {
            anyhow::anyhow!("{} subprocess output reading: {error:?}", self.executable)
        })?;
        let stderr_message = String::from_utf8_lossy(result.stderr.as_slice());
        let mut solc_output = match era_compiler_common::deserialize_from_slice::<StandardJsonOutput>(
            result.stdout.as_slice(),
        ) {
            Ok(solc_output) => solc_output,
            Err(error) => {
                anyhow::bail!(
                    "{} subprocess stdout parsing: {error:?} (stderr: {stderr_message})",
                    self.executable
                );
            }
        };
        if !result.status.success() {
            anyhow::bail!("{} subprocess: {stderr_message}", self.executable);
        }

        let errors = solc_output.errors.get_or_insert_with(Vec::new);
        errors.retain(|error| match error.error_code.as_deref() {
            Some(code) => !StandardJsonOutputError::IGNORED_WARNING_CODES.contains(&code),
            None => true,
        });
        errors.append(messages);

        if let Some(pipeline) = pipeline {
            let mut suppressed_messages = input.suppressed_errors.to_owned().unwrap_or_default();
            suppressed_messages.extend(input.suppressed_warnings.to_owned().unwrap_or_default());

            input.resolve_sources();
            solc_output.preprocess_ast(
                &input.sources,
                &self.version,
                pipeline,
                suppressed_messages.as_slice(),
            )?;
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

        let process = command
            .spawn()
            .map_err(|error| anyhow::anyhow!("{} subprocess spawning: {:?}", executable, error))?;

        let result = process.wait_with_output().map_err(|error| {
            anyhow::anyhow!("{} subprocess output reading: {error:?}", self.executable)
        })?;
        let stderr_message = String::from_utf8_lossy(result.stderr.as_slice());
        let mut combined_json = match era_compiler_common::deserialize_from_slice::<CombinedJson>(
            result.stdout.as_slice(),
        ) {
            Ok(combined_json) => combined_json,
            Err(error) => {
                anyhow::bail!(
                    "{} subprocess stdout parsing: {error:?} (stderr: {stderr_message})",
                    self.executable
                );
            }
        };
        if !result.status.success() {
            anyhow::bail!("{} subprocess: {stderr_message}", self.executable);
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
        messages: &mut Vec<StandardJsonOutputError>,
    ) -> anyhow::Result<StandardJsonOutput> {
        if self.version.default != Self::LAST_SUPPORTED_VERSION {
            anyhow::bail!(
                "Yul validation is only supported with the latest supported version of the Solidity compiler: {}",
                Self::LAST_SUPPORTED_VERSION,
            );
        }

        let mut solc_input = StandardJsonInput::from_yul_paths(
            paths,
            libraries.clone(),
            StandardJsonInputSettingsOptimizer::new_yul_validation(),
            vec![],
        );
        self.validate_yul_standard_json(&mut solc_input, messages)
    }

    ///
    /// Validates the Yul project as standard JSON input.
    ///
    pub fn validate_yul_standard_json(
        &self,
        solc_input: &mut StandardJsonInput,
        messages: &mut Vec<StandardJsonOutputError>,
    ) -> anyhow::Result<StandardJsonOutput> {
        if self.version.default != Self::LAST_SUPPORTED_VERSION {
            anyhow::bail!(
                "Yul validation is only supported with the latest supported version of the Solidity compiler: {}",
                Self::LAST_SUPPORTED_VERSION,
            );
        }

        solc_input.normalize_yul_validation();
        let solc_output = self.standard_json(solc_input, None, messages, None, vec![], None)?;
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
            .map_err(|error| anyhow::anyhow!("{} subprocess: {:?}", executable, error))?;
        if !output.status.success() {
            anyhow::bail!(
                "{} version getting: {}",
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

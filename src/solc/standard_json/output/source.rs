//!
//! The `solc --standard-json` output source.
//!

use std::collections::BTreeMap;

use crate::solc::pipeline::Pipeline as SolcPipeline;
use crate::solc::standard_json::output::error::Error as SolcStandardJsonOutputError;
use crate::solc::version::Version as SolcVersion;
use crate::warning::Warning;

///
/// The `solc --standard-json` output source.
///
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    /// The source code ID.
    pub id: usize,
    /// The source code AST.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ast: Option<serde_json::Value>,
}

impl Source {
    ///
    /// Initializes a standard JSON source.
    ///
    /// Is used for projects compiled without `solc`.
    ///
    pub fn new(id: usize) -> Self {
        Self { id, ast: None }
    }

    ///
    /// Checks the AST node for the `ecrecover` function usage.
    ///
    pub fn check_ecrecover(
        ast: &serde_json::Value,
        id_paths: &BTreeMap<usize, &String>,
    ) -> Option<SolcStandardJsonOutputError> {
        let ast = ast.as_object()?;

        if ast.get("nodeType")?.as_str()? != "FunctionCall" {
            return None;
        }

        let expression = ast.get("expression")?.as_object()?;
        if expression.get("nodeType")?.as_str()? != "Identifier" {
            return None;
        }
        if expression.get("name")?.as_str()? != "ecrecover" {
            return None;
        }

        Some(SolcStandardJsonOutputError::warning_ecrecover(
            ast.get("src")?.as_str(),
            id_paths,
        ))
    }

    ///
    /// Checks the AST node for the `<address payable>`'s `send` and `transfer` methods usage.
    ///
    pub fn check_send_and_transfer(
        ast: &serde_json::Value,
        id_paths: &BTreeMap<usize, &String>,
    ) -> Option<SolcStandardJsonOutputError> {
        let ast = ast.as_object()?;

        if ast.get("nodeType")?.as_str()? != "FunctionCall" {
            return None;
        }

        let expression = ast.get("expression")?.as_object()?;
        if expression.get("nodeType")?.as_str()? != "MemberAccess" {
            return None;
        }
        let member_name = expression.get("memberName")?.as_str()?;
        if member_name != "send" && member_name != "transfer" {
            return None;
        }

        let expression = expression.get("expression")?.as_object()?;
        let type_descriptions = expression.get("typeDescriptions")?.as_object()?;
        let type_identifier = type_descriptions.get("typeIdentifier")?.as_str()?;
        if type_identifier != "t_address" && type_identifier != "t_address_payable" {
            return None;
        }

        Some(SolcStandardJsonOutputError::warning_send_and_transfer(
            ast.get("src")?.as_str(),
            id_paths,
        ))
    }

    ///
    /// Checks the AST node for the `extcodesize` assembly instruction usage.
    ///
    pub fn check_assembly_extcodesize(
        ast: &serde_json::Value,
        id_paths: &BTreeMap<usize, &String>,
    ) -> Option<SolcStandardJsonOutputError> {
        let ast = ast.as_object()?;

        if ast.get("nodeType")?.as_str()? != "YulFunctionCall" {
            return None;
        }
        if ast
            .get("functionName")?
            .as_object()?
            .get("name")?
            .as_str()?
            != "extcodesize"
        {
            return None;
        }

        Some(SolcStandardJsonOutputError::warning_extcodesize(
            ast.get("src")?.as_str(),
            id_paths,
        ))
    }

    ///
    /// Checks the AST node for the `origin` assembly instruction usage.
    ///
    pub fn check_assembly_origin(
        ast: &serde_json::Value,
        id_paths: &BTreeMap<usize, &String>,
    ) -> Option<SolcStandardJsonOutputError> {
        let ast = ast.as_object()?;

        if ast.get("nodeType")?.as_str()? != "YulFunctionCall" {
            return None;
        }
        if ast
            .get("functionName")?
            .as_object()?
            .get("name")?
            .as_str()?
            != "origin"
        {
            return None;
        }

        Some(SolcStandardJsonOutputError::warning_tx_origin(
            ast.get("src")?.as_str(),
            id_paths,
        ))
    }

    ///
    /// Checks the AST node for the `tx.origin` value usage.
    ///
    pub fn check_tx_origin(
        ast: &serde_json::Value,
        id_paths: &BTreeMap<usize, &String>,
    ) -> Option<SolcStandardJsonOutputError> {
        let ast = ast.as_object()?;

        if ast.get("nodeType")?.as_str()? != "MemberAccess" {
            return None;
        }
        if ast.get("memberName")?.as_str()? != "origin" {
            return None;
        }

        let expression = ast.get("expression")?.as_object()?;
        if expression.get("nodeType")?.as_str()? != "Identifier" {
            return None;
        }
        if expression.get("name")?.as_str()? != "tx" {
            return None;
        }

        Some(SolcStandardJsonOutputError::warning_tx_origin(
            ast.get("src")?.as_str(),
            id_paths,
        ))
    }

    ///
    /// Checks the AST node for the internal function pointers value usage.
    ///
    pub fn check_internal_function_pointer(
        ast: &serde_json::Value,
        id_paths: &BTreeMap<usize, &String>,
    ) -> Option<SolcStandardJsonOutputError> {
        let ast = ast.as_object()?;

        if ast.get("nodeType")?.as_str()? != "VariableDeclaration" {
            return None;
        }

        let type_descriptions = ast.get("typeDescriptions")?.as_object()?;
        if !type_descriptions
            .get("typeIdentifier")?
            .as_str()?
            .contains("function_internal")
        {
            return None;
        }

        Some(
            SolcStandardJsonOutputError::error_internal_function_pointer(
                ast.get("src")?.as_str(),
                id_paths,
            ),
        )
    }

    ///
    /// Returns the list of messages for some specific parts of the AST.
    ///
    pub fn get_messages(
        ast: &serde_json::Value,
        id_paths: &BTreeMap<usize, &String>,
        version: &SolcVersion,
        pipeline: SolcPipeline,
        suppressed_warnings: &[Warning],
    ) -> Vec<SolcStandardJsonOutputError> {
        let mut messages = Vec::new();
        if !suppressed_warnings.contains(&Warning::EcRecover) {
            if let Some(message) = Self::check_ecrecover(ast, id_paths) {
                messages.push(message);
            }
        }
        if !suppressed_warnings.contains(&Warning::SendTransfer) {
            if let Some(message) = Self::check_send_and_transfer(ast, id_paths) {
                messages.push(message);
            }
        }
        if !suppressed_warnings.contains(&Warning::ExtCodeSize) {
            if let Some(message) = Self::check_assembly_extcodesize(ast, id_paths) {
                messages.push(message);
            }
        }
        if !suppressed_warnings.contains(&Warning::TxOrigin) {
            if let Some(message) = Self::check_assembly_origin(ast, id_paths) {
                messages.push(message);
            }
            if let Some(message) = Self::check_tx_origin(ast, id_paths) {
                messages.push(message);
            }
        }
        if SolcPipeline::EVMLA == pipeline && version.l2_revision.is_none() {
            if let Some(message) = Self::check_internal_function_pointer(ast, id_paths) {
                messages.push(message);
            }
        }

        match ast {
            serde_json::Value::Array(array) => {
                for element in array.iter() {
                    messages.extend(Self::get_messages(
                        element,
                        id_paths,
                        version,
                        pipeline,
                        suppressed_warnings,
                    ));
                }
            }
            serde_json::Value::Object(object) => {
                for (_key, value) in object.iter() {
                    messages.extend(Self::get_messages(
                        value,
                        id_paths,
                        version,
                        pipeline,
                        suppressed_warnings,
                    ));
                }
            }
            _ => {}
        }

        messages
    }

    ///
    /// Returns the name of the last contract.
    ///
    pub fn last_contract_name(&self) -> anyhow::Result<String> {
        self.ast
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("The AST is empty"))?
            .get("nodes")
            .and_then(|value| value.as_array())
            .ok_or_else(|| {
                anyhow::anyhow!("The last contract cannot be found in an empty list of nodes")
            })?
            .iter()
            .filter_map(
                |node| match node.get("nodeType").and_then(|node| node.as_str()) {
                    Some("ContractDefinition") => Some(node.get("name")?.as_str()?.to_owned()),
                    _ => None,
                },
            )
            .last()
            .ok_or_else(|| anyhow::anyhow!("The last contract not found in the AST"))
    }
}

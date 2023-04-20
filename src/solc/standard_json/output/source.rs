//!
//! The `solc --standard-json` output source.
//!

use serde::Deserialize;
use serde::Serialize;

use crate::solc::pipeline::Pipeline as SolcPipeline;
use crate::solc::standard_json::output::error::Error as SolcStandardJsonOutputError;

///
/// The `solc --standard-json` output source.
///
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    /// The source code ID.
    pub id: usize,
    /// The source code AST.
    pub ast: Option<serde_json::Value>,
}

impl Source {
    ///
    /// Checks the AST node for the `ecrecover` function usage.
    ///
    pub fn check_ecrecover(ast: &serde_json::Value) -> Option<SolcStandardJsonOutputError> {
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

        Some(SolcStandardJsonOutputError::message_ecrecover(
            ast.get("src")?.as_str(),
        ))
    }

    ///
    /// Checks the AST node for the `<address payable>`'s `send` and `transfer` methods usage.
    ///
    pub fn check_send_and_transfer(ast: &serde_json::Value) -> Option<SolcStandardJsonOutputError> {
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

        Some(SolcStandardJsonOutputError::message_send_and_transfer(
            ast.get("src")?.as_str(),
        ))
    }

    ///
    /// Checks the AST node for the `extcodesize` assembly instruction usage.
    ///
    pub fn check_assembly_extcodesize(
        ast: &serde_json::Value,
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

        Some(SolcStandardJsonOutputError::message_extcodesize(
            ast.get("src")?.as_str(),
        ))
    }

    ///
    /// Checks the AST node for the `origin` assembly instruction usage.
    ///
    pub fn check_assembly_origin(ast: &serde_json::Value) -> Option<SolcStandardJsonOutputError> {
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

        Some(SolcStandardJsonOutputError::message_origin(
            ast.get("src")?.as_str(),
        ))
    }

    ///
    /// Checks the AST node for the `tx.origin` value usage.
    ///
    pub fn check_tx_origin(ast: &serde_json::Value) -> Option<SolcStandardJsonOutputError> {
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

        Some(SolcStandardJsonOutputError::message_origin(
            ast.get("src")?.as_str(),
        ))
    }

    ///
    /// Checks the AST node for the `timestamp` assembly instruction usage.
    ///
    pub fn check_assembly_timestamp(
        ast: &serde_json::Value,
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
            != "timestamp"
        {
            return None;
        }

        Some(SolcStandardJsonOutputError::message_timestamp(
            ast.get("src")?.as_str(),
        ))
    }

    ///
    /// Checks the AST node for the `block.timestamp` value usage.
    ///
    pub fn check_block_timestamp(ast: &serde_json::Value) -> Option<SolcStandardJsonOutputError> {
        let ast = ast.as_object()?;

        if ast.get("nodeType")?.as_str()? != "MemberAccess" {
            return None;
        }
        if ast.get("memberName")?.as_str()? != "timestamp" {
            return None;
        }

        let expression = ast.get("expression")?.as_object()?;
        if expression.get("nodeType")?.as_str()? != "Identifier" {
            return None;
        }
        if expression.get("name")?.as_str()? != "block" {
            return None;
        }

        Some(SolcStandardJsonOutputError::message_timestamp(
            ast.get("src")?.as_str(),
        ))
    }

    ///
    /// Checks the AST node for the `number` assembly instruction usage.
    ///
    pub fn check_assembly_number(ast: &serde_json::Value) -> Option<SolcStandardJsonOutputError> {
        let ast = ast.as_object()?;

        if ast.get("nodeType")?.as_str()? != "YulFunctionCall" {
            return None;
        }
        if ast
            .get("functionName")?
            .as_object()?
            .get("name")?
            .as_str()?
            != "number"
        {
            return None;
        }

        Some(SolcStandardJsonOutputError::message_number(
            ast.get("src")?.as_str(),
        ))
    }

    ///
    /// Checks the AST node for the `block.number` value usage.
    ///
    pub fn check_block_number(ast: &serde_json::Value) -> Option<SolcStandardJsonOutputError> {
        let ast = ast.as_object()?;

        if ast.get("nodeType")?.as_str()? != "MemberAccess" {
            return None;
        }
        if ast.get("memberName")?.as_str()? != "number" {
            return None;
        }

        let expression = ast.get("expression")?.as_object()?;
        if expression.get("nodeType")?.as_str()? != "Identifier" {
            return None;
        }
        if expression.get("name")?.as_str()? != "block" {
            return None;
        }

        Some(SolcStandardJsonOutputError::message_number(
            ast.get("src")?.as_str(),
        ))
    }

    ///
    /// Checks the AST node for the internal function pointers value usage.
    ///
    pub fn check_internal_function_pointer(
        ast: &serde_json::Value,
    ) -> Option<SolcStandardJsonOutputError> {
        let ast = ast.as_object()?;

        if ast.get("nodeType")?.as_str()? != "VariableDeclaration" {
            return None;
        }

        let type_descriptions = ast.get("typeDescriptions")?.as_object()?;
        if !type_descriptions
            .get("typeIdentifier")?
            .as_str()?
            .starts_with("t_function_internal")
        {
            return None;
        }

        Some(
            SolcStandardJsonOutputError::message_internal_function_pointer(
                ast.get("src")?.as_str(),
            ),
        )
    }

    ///
    /// Returns the list of messages for some specific parts of the AST.
    ///
    pub fn get_messages(
        ast: &serde_json::Value,
        pipeline: SolcPipeline,
    ) -> Vec<SolcStandardJsonOutputError> {
        let mut messages = Vec::new();
        if let Some(message) = Self::check_ecrecover(ast) {
            messages.push(message);
        }
        if let Some(message) = Self::check_send_and_transfer(ast) {
            messages.push(message);
        }
        if let Some(message) = Self::check_assembly_extcodesize(ast) {
            messages.push(message);
        }
        if let Some(message) = Self::check_assembly_origin(ast) {
            messages.push(message);
        }
        if let Some(message) = Self::check_tx_origin(ast) {
            messages.push(message);
        }
        if let Some(message) = Self::check_assembly_timestamp(ast) {
            messages.push(message);
        }
        if let Some(message) = Self::check_block_timestamp(ast) {
            messages.push(message);
        }
        if let Some(message) = Self::check_assembly_number(ast) {
            messages.push(message);
        }
        if let Some(message) = Self::check_block_number(ast) {
            messages.push(message);
        }
        if let SolcPipeline::EVMLA = pipeline {
            if let Some(message) = Self::check_internal_function_pointer(ast) {
                messages.push(message);
            }
        }

        match ast {
            serde_json::Value::Array(array) => {
                for element in array.iter() {
                    messages.extend(Self::get_messages(element, pipeline));
                }
            }
            serde_json::Value::Object(object) => {
                for (_key, value) in object.iter() {
                    messages.extend(Self::get_messages(value, pipeline));
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
                    Some(node_type) if node_type == "ContractDefinition" => {
                        Some(node.get("name")?.as_str()?.to_owned())
                    }
                    _ => None,
                },
            )
            .last()
            .ok_or_else(|| anyhow::anyhow!("The last contract not found in the AST"))
    }
}

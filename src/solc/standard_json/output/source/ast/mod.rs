//!
//! The `solc --standard-json` AST output.
//!

pub mod expression;

use serde::Deserialize;
use serde::Serialize;

use crate::solc::standard_json::output::error::Error as SolcStandardJsonOutputError;

use self::expression::Expression;

///
/// The `solc --standard-json` AST output.
///
#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(clippy::upper_case_acronyms)]
#[serde(rename_all = "camelCase")]
pub struct AST {
    /// The node type.
    pub node_type: Option<String>,
    /// The node name.
    pub name: Option<String>,
    /// The node location in the source code.
    pub src: Option<String>,
    /// The member name.
    pub member_name: Option<String>,
    /// The function name.
    pub function_name: Option<Box<Self>>,

    /// The inner AST.
    #[serde(rename = "AST")]
    pub ast: Option<Box<Self>>,
    /// The next level.
    pub nodes: Option<Vec<Self>>,
    /// The function statements.
    pub statements: Option<Vec<Self>>,

    /// The function argument expressions.
    pub arguments: Option<Vec<Expression>>,
    /// The declaration expressions.
    pub declarations: Option<Vec<Expression>>,
    /// The member expressions.
    pub members: Option<Vec<Expression>>,
    /// The tuple expressions.
    pub components: Option<Vec<Expression>>,
    /// The try-catch clause expressions.
    pub clauses: Option<Vec<Expression>>,
    /// The option expressions.
    pub options: Option<Vec<Expression>>,

    /// The function body.
    pub body: Option<Box<Expression>>,
    /// The conditional `true` block.
    pub true_body: Option<Box<Expression>>,
    /// The conditional `false` block.
    pub false_body: Option<Box<Expression>>,
    /// The function expression.
    pub expression: Option<Box<Expression>>,
    /// The conditional expression.
    pub condition: Option<Box<Expression>>,
    /// The initialization expression.
    pub initialization_expression: Option<Box<Expression>>,
    /// The left operand expression.
    pub left_expression: Option<Box<Expression>>,
    /// The right operand expression.
    pub right_expression: Option<Box<Expression>>,
    /// The unary operand expression.
    pub sub_expression: Option<Box<Expression>>,
    /// The `true` conditional expression.
    pub true_expression: Option<Box<Expression>>,
    /// The `false` conditional expression.
    pub false_expression: Option<Box<Expression>>,
    /// The loop expression.
    pub loop_expression: Option<Box<Expression>>,
    /// The index access base expression.
    pub base_expression: Option<Box<Expression>>,
    /// The index access index expression.
    pub index_expression: Option<Box<Expression>>,
    /// The loop range start expression.
    pub start_expression: Option<Box<Expression>>,
    /// The loop range end expression.
    pub end_expression: Option<Box<Expression>>,
    /// The ordinary expression.
    pub value: Option<Box<Expression>>,
    /// The initialization expression.
    pub initial_value: Option<Box<Expression>>,
    /// The external call expression.
    pub external_call: Option<Box<Expression>>,
    /// The event call expression.
    pub event_call: Option<Box<Expression>>,
    /// The error call expression.
    pub error_call: Option<Box<Expression>>,
    /// The assignment left-hand side.
    pub left_hand_side: Option<Box<Expression>>,
    /// The assignment right-hand side.
    pub right_hand_side: Option<Box<Expression>>,
    /// The length expression.
    pub length: Option<Box<Expression>>,
}

impl AST {
    ///
    /// Checks the AST node for the `ecrecover` function usage.
    ///
    pub fn check_ecrecover(&self) -> Option<SolcStandardJsonOutputError> {
        if let Some(node_type) = self.node_type.as_ref() {
            if node_type.as_str() != "FunctionCall" {
                return None;
            }
        }

        let expression = self.expression.as_ref()?.as_node()?;
        if let Some(node_type) = expression.node_type.as_ref() {
            if node_type.as_str() != "Identifier" {
                return None;
            }
        }
        if let Some(name) = expression.name.as_ref() {
            if name.as_str() != "ecrecover" {
                return None;
            }
        }

        Some(SolcStandardJsonOutputError::message_ecrecover(
            self.src.as_deref(),
        ))
    }

    ///
    /// Checks the AST node for the `<address payable>`'s `send` and `transfer` methods usage.
    ///
    pub fn check_send_and_transfer(&self) -> Option<SolcStandardJsonOutputError> {
        if let Some(node_type) = self.node_type.as_ref() {
            if node_type.as_str() != "FunctionCall" {
                return None;
            }
        }

        let expression = self.expression.as_ref()?.as_node()?;
        if let Some(node_type) = expression.node_type.as_ref() {
            if node_type.as_str() != "MemberAccess" {
                return None;
            }
        }
        if let Some(member_name) = expression.member_name.as_ref() {
            if member_name.as_str() != "send" && member_name.as_str() != "transfer" {
                return None;
            }
        }

        Some(SolcStandardJsonOutputError::message_send_and_transfer(
            self.src.as_deref(),
        ))
    }

    ///
    /// Checks the AST node for the `extcodesize` assembly instruction usage.
    ///
    pub fn check_assembly_extcodesize(&self) -> Option<SolcStandardJsonOutputError> {
        if let Some(node_type) = self.node_type.as_ref() {
            if node_type.as_str() != "YulFunctionCall" {
                return None;
            }
        }

        if let Some(function_name) = self
            .function_name
            .as_ref()
            .and_then(|inner| inner.name.as_ref())
        {
            if function_name.as_str() != "extcodesize" {
                return None;
            }
        }

        Some(SolcStandardJsonOutputError::message_extcodesize(
            self.src.as_deref(),
        ))
    }

    ///
    /// Checks the AST node for the `origin` assembly instruction usage.
    ///
    pub fn check_assembly_origin(&self) -> Option<SolcStandardJsonOutputError> {
        if let Some(node_type) = self.node_type.as_ref() {
            if node_type.as_str() != "YulFunctionCall" {
                return None;
            }
        }

        if let Some(function_name) = self
            .function_name
            .as_ref()
            .and_then(|inner| inner.name.as_ref())
        {
            if function_name.as_str() != "origin" {
                return None;
            }
        }

        Some(SolcStandardJsonOutputError::message_origin(
            self.src.as_deref(),
        ))
    }

    ///
    /// Checks the AST node for the `tx.origin` value usage.
    ///
    pub fn check_tx_origin(&self) -> Option<SolcStandardJsonOutputError> {
        if let Some(node_type) = self.node_type.as_ref() {
            if node_type.as_str() != "MemberAccess" {
                return None;
            }
        }
        if let Some(member_name) = self.member_name.as_ref() {
            if member_name.as_str() != "origin" {
                return None;
            }
        }

        let expression = self.expression.as_ref()?.as_node()?;
        if let Some(node_type) = expression.node_type.as_ref() {
            if node_type.as_str() != "Identifier" {
                return None;
            }
        }
        if let Some(name) = expression.name.as_ref() {
            if name.as_str() != "tx" {
                return None;
            }
        }

        Some(SolcStandardJsonOutputError::message_origin(
            self.src.as_deref(),
        ))
    }

    ///
    /// Checks the AST node for the `timestamp` assembly instruction usage.
    ///
    pub fn check_assembly_timestamp(&self) -> Option<SolcStandardJsonOutputError> {
        if let Some(node_type) = self.node_type.as_ref() {
            if node_type.as_str() != "YulFunctionCall" {
                return None;
            }
        }

        if let Some(function_name) = self
            .function_name
            .as_ref()
            .and_then(|inner| inner.name.as_ref())
        {
            if function_name.as_str() != "timestamp" {
                return None;
            }
        }

        Some(SolcStandardJsonOutputError::message_timestamp(
            self.src.as_deref(),
        ))
    }

    ///
    /// Checks the AST node for the `block.timestamp` value usage.
    ///
    pub fn check_block_timestamp(&self) -> Option<SolcStandardJsonOutputError> {
        if let Some(node_type) = self.node_type.as_ref() {
            if node_type.as_str() != "MemberAccess" {
                return None;
            }
        }
        if let Some(member_name) = self.member_name.as_ref() {
            if member_name.as_str() != "timestamp" {
                return None;
            }
        }

        let expression = self.expression.as_ref()?.as_node()?;
        if let Some(node_type) = expression.node_type.as_ref() {
            if node_type.as_str() != "Identifier" {
                return None;
            }
        }
        if let Some(name) = expression.name.as_ref() {
            if name.as_str() != "block" {
                return None;
            }
        }

        Some(SolcStandardJsonOutputError::message_timestamp(
            self.src.as_deref(),
        ))
    }

    ///
    /// Checks the AST node for the `number` assembly instruction usage.
    ///
    pub fn check_assembly_number(&self) -> Option<SolcStandardJsonOutputError> {
        if let Some(node_type) = self.node_type.as_ref() {
            if node_type.as_str() != "YulFunctionCall" {
                return None;
            }
        }

        if let Some(function_name) = self
            .function_name
            .as_ref()
            .and_then(|inner| inner.name.as_ref())
        {
            if function_name.as_str() != "number" {
                return None;
            }
        }

        Some(SolcStandardJsonOutputError::message_number(
            self.src.as_deref(),
        ))
    }

    ///
    /// Checks the AST node for the `block.number` value usage.
    ///
    pub fn check_block_number(&self) -> Option<SolcStandardJsonOutputError> {
        if let Some(node_type) = self.node_type.as_ref() {
            if node_type.as_str() != "MemberAccess" {
                return None;
            }
        }
        if let Some(member_name) = self.member_name.as_ref() {
            if member_name.as_str() != "number" {
                return None;
            }
        }

        let expression = self.expression.as_ref()?.as_node()?;
        if let Some(node_type) = expression.node_type.as_ref() {
            if node_type.as_str() != "Identifier" {
                return None;
            }
        }
        if let Some(name) = expression.name.as_ref() {
            if name.as_str() != "block" {
                return None;
            }
        }

        Some(SolcStandardJsonOutputError::message_number(
            self.src.as_deref(),
        ))
    }

    ///
    /// Returns the list of messages for some specific parts of the AST.
    ///
    pub fn get_messages(&self) -> anyhow::Result<Vec<SolcStandardJsonOutputError>> {
        let mut messages = Vec::new();
        if let Some(message) = self.check_ecrecover() {
            messages.push(message);
        }
        if let Some(message) = self.check_send_and_transfer() {
            messages.push(message);
        }
        if let Some(message) = self.check_assembly_extcodesize() {
            messages.push(message);
        }
        if let Some(message) = self.check_assembly_origin() {
            messages.push(message);
        }
        if let Some(message) = self.check_tx_origin() {
            messages.push(message);
        }
        if let Some(message) = self.check_assembly_timestamp() {
            messages.push(message);
        }
        if let Some(message) = self.check_block_timestamp() {
            messages.push(message);
        }
        if let Some(message) = self.check_assembly_number() {
            messages.push(message);
        }
        if let Some(message) = self.check_block_number() {
            messages.push(message);
        }

        if let Some(inner) = self.ast.as_ref() {
            messages.extend(inner.get_messages()?);
        }
        if let Some(inner) = self.nodes.as_ref() {
            for element in inner.iter() {
                messages.extend(element.get_messages()?);
            }
        }
        if let Some(inner) = self.statements.as_ref() {
            for element in inner.iter() {
                messages.extend(element.get_messages()?);
            }
        }

        if let Some(inner) = self.arguments.as_ref() {
            for element in inner.iter() {
                messages.extend(element.get_messages()?);
            }
        }
        if let Some(inner) = self.declarations.as_ref() {
            for element in inner.iter() {
                messages.extend(element.get_messages()?);
            }
        }
        if let Some(inner) = self.members.as_ref() {
            for element in inner.iter() {
                messages.extend(element.get_messages()?);
            }
        }
        if let Some(inner) = self.components.as_ref() {
            for element in inner.iter() {
                messages.extend(element.get_messages()?);
            }
        }
        if let Some(inner) = self.clauses.as_ref() {
            for element in inner.iter() {
                messages.extend(element.get_messages()?);
            }
        }
        if let Some(inner) = self.options.as_ref() {
            for element in inner.iter() {
                messages.extend(element.get_messages()?);
            }
        }

        if let Some(inner) = self.body.as_ref() {
            messages.extend(inner.get_messages()?);
        }
        if let Some(inner) = self.true_body.as_ref() {
            messages.extend(inner.get_messages()?);
        }
        if let Some(inner) = self.false_body.as_ref() {
            messages.extend(inner.get_messages()?);
        }
        if let Some(inner) = self.expression.as_ref() {
            messages.extend(inner.get_messages()?);
        }
        if let Some(inner) = self.condition.as_ref() {
            messages.extend(inner.get_messages()?);
        }
        if let Some(inner) = self.initialization_expression.as_ref() {
            messages.extend(inner.get_messages()?);
        }
        if let Some(inner) = self.left_expression.as_ref() {
            messages.extend(inner.get_messages()?);
        }
        if let Some(inner) = self.right_expression.as_ref() {
            messages.extend(inner.get_messages()?);
        }
        if let Some(inner) = self.sub_expression.as_ref() {
            messages.extend(inner.get_messages()?);
        }
        if let Some(inner) = self.true_expression.as_ref() {
            messages.extend(inner.get_messages()?);
        }
        if let Some(inner) = self.false_expression.as_ref() {
            messages.extend(inner.get_messages()?);
        }
        if let Some(inner) = self.loop_expression.as_ref() {
            messages.extend(inner.get_messages()?);
        }
        if let Some(inner) = self.base_expression.as_ref() {
            messages.extend(inner.get_messages()?);
        }
        if let Some(inner) = self.index_expression.as_ref() {
            messages.extend(inner.get_messages()?);
        }
        if let Some(inner) = self.start_expression.as_ref() {
            messages.extend(inner.get_messages()?);
        }
        if let Some(inner) = self.end_expression.as_ref() {
            messages.extend(inner.get_messages()?);
        }
        if let Some(inner) = self.value.as_ref() {
            messages.extend(inner.get_messages()?);
        }
        if let Some(inner) = self.initial_value.as_ref() {
            messages.extend(inner.get_messages()?);
        }
        if let Some(inner) = self.external_call.as_ref() {
            messages.extend(inner.get_messages()?);
        }
        if let Some(inner) = self.event_call.as_ref() {
            messages.extend(inner.get_messages()?);
        }
        if let Some(inner) = self.error_call.as_ref() {
            messages.extend(inner.get_messages()?);
        }
        if let Some(inner) = self.left_hand_side.as_ref() {
            messages.extend(inner.get_messages()?);
        }
        if let Some(inner) = self.right_hand_side.as_ref() {
            messages.extend(inner.get_messages()?);
        }
        if let Some(inner) = self.length.as_ref() {
            messages.extend(inner.get_messages()?);
        }

        Ok(messages)
    }

    ///
    /// Returns the name of the last contract.
    ///
    pub fn last_contract_name(&self) -> anyhow::Result<String> {
        self.nodes
            .as_ref()
            .ok_or_else(|| {
                anyhow::anyhow!("The last contract cannot be found in the empty list of nodes")
            })?
            .iter()
            .filter(|node| node.node_type.as_deref() == Some("ContractDefinition"))
            .last()
            .and_then(|node| node.name.as_ref())
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("The last contract not found in the AST"))
    }
}

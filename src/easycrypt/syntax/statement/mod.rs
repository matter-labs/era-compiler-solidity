use self::{block::Block, call::ProcCall, if_conditional::IfConditional};

use super::{definition::Definition, expression::Expression, reference::Reference};

pub mod block;
pub mod call;
pub mod if_conditional;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    VarDefinition(Definition, Expression),
    Expression(Expression),
    Block(Block),
    If(IfConditional),
    EAssignment(Vec<Reference>, Box<Expression>), // x <- expr
    PAssignment(Vec<Reference>, ProcCall),        // x <@ proc
    Return(Expression),
    While(Expression, Box<Self>),
    // SAssignment for // x <$ distr
    Pass,
}

impl Statement {
    /// Returns `true` if the statement is [`VarDefinition`].
    ///
    /// [`VarDefinition`]: Statement::VarDefinition
    #[must_use]
    pub fn is_var_definition(&self) -> bool {
        matches!(self, Self::VarDefinition(..))
    }

    /// Returns `true` if the statement is [`Expression`].
    ///
    /// [`Expression`]: Statement::Expression
    #[must_use]
    pub fn is_expression(&self) -> bool {
        matches!(self, Self::Expression(..))
    }

    /// Returns `true` if the statement is [`Block`].
    ///
    /// [`Block`]: Statement::Block
    #[must_use]
    pub fn is_block(&self) -> bool {
        matches!(self, Self::Block(..))
    }

    /// Returns `true` if the statement is [`If`].
    ///
    /// [`If`]: Statement::If
    #[must_use]
    pub fn is_if(&self) -> bool {
        matches!(self, Self::If(..))
    }

    /// Returns `true` if the statement is [`EAssignment`].
    ///
    /// [`EAssignment`]: Statement::EAssignment
    #[must_use]
    pub fn is_eassignment(&self) -> bool {
        matches!(self, Self::EAssignment(..))
    }

    /// Returns `true` if the statement is [`PAssignment`].
    ///
    /// [`PAssignment`]: Statement::PAssignment
    #[must_use]
    pub fn is_passignment(&self) -> bool {
        matches!(self, Self::PAssignment(..))
    }

    /// Returns `true` if the statement is [`Return`].
    ///
    /// [`Return`]: Statement::Return
    #[must_use]
    pub fn is_return(&self) -> bool {
        matches!(self, Self::Return(..))
    }

    /// Returns `true` if the statement is [`While`].
    ///
    /// [`While`]: Statement::While
    #[must_use]
    pub fn is_while(&self) -> bool {
        matches!(self, Self::While(..))
    }
}

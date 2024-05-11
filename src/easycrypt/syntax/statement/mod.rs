use self::{block::Block, call::ProcCall};

use super::{definition::Definition, expression::Expression, reference::Reference};

pub mod block;
pub mod call;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    VarDefinition(Definition, Expression),
    Expression(Expression),
    Block(Block),
    If(Expression, Box<Self>, Box<Self>),
    EAssignment(Vec<Reference>, Box<Expression>), // x <- expr
    PAssignment(Vec<Reference>, ProcCall),        // x <@ proc
    Return(Expression),
    While(Expression, Box<Self>),
    // SAssignment for // x <$ distr
    Pass,
}

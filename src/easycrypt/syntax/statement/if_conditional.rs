use crate::easycrypt::syntax::expression::Expression;
use crate::easycrypt::syntax::statement::Statement;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfConditional {
    pub condition: Expression,
    pub yes: Box<Statement>,
    pub no: Option<Box<Statement>>,
}

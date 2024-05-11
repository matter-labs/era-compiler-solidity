use crate::easycrypt::syntax::expression::Expression;
use crate::easycrypt::syntax::proc::name::ProcName;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcCall {
    pub target: ProcName,
    pub arguments: Vec<Expression>,
}

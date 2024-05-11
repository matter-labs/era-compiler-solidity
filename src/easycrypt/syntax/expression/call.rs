use super::Expression;
use crate::easycrypt::syntax::function::name::FunctionName;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionCall {
    pub target: FunctionName,
    pub arguments: Vec<Expression>,
}

//!
//! EasyCrypt AST
//!

use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
};

use super::from_yul::location::Location;

type Name = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal {
    String(String),
    Int(u64), // FIXME u256
    Bool(bool),
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Bool,
    Int,
    String,
    Arrow(Box<Type>, Box<Type>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryOpType {
    Add,
    Sub,
    Mul,
    Mod,
    And,
    Or,
    Xor,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnaryOpType {
    Neg,
    Not,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parameter {
    identifier: Name,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Definition {
    identifier: Name,
    location: Location,
}

impl Hash for Definition {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.identifier.hash(state);
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reference {
    identifier: Name,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Unary(UnaryOpType, Box<Self>),
    Binary(BinaryOpType, Box<Expression>, Box<Expression>),
    ECall(Box<Reference>, Vec<Box<Expression>>),
    Literal(Literal),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signature {
    pub formal_parameters: Vec<(Parameter, Type)>,
    pub return_type: Vec<Type>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    pub name: Definition,
    pub signature: Signature,
    pub body: Expression,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Proc {
    pub name: Definition,
    pub signature: Signature,
    pub body: Statement,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PCall {
    pub proc: Reference,
    pub arguments: Vec<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    VarDefinition(Definition, Expression),
    Expression(Expression),
    Block(Block),
    If(Expression, Box<Statement>, Box<Statement>),
    EAssignment(Vec<(Box<Reference>, Expression)>), // x <- expr
    PAssignment(Box<Reference>, PCall),             // x <@ proc
    Return(Expression),
    // SAssignment for // x <$ distr
    Pass,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleDefinition {
    ProcDef(Proc),
    FunDef(Function),
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    pub definitions: HashMap<Definition, ModuleDefinition>,
}

impl Module {
    pub fn new() -> Self {
        Self {
            definitions: HashMap::new(),
        }
    }
}

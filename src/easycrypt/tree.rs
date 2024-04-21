//!
//! EasyCrypt AST
//!
type Identifier = String;
pub enum Literal {
    String(String),
    Int(u16),
    Bool(bool),
}
pub enum Type {
    Bool,
    Int,
    String,
    Arrow(Box<Type>, Box<Type>),
}

pub enum BinaryOpType {
    Add,
    Sub,
    Mul,
    Mod,
    And,
    Or,
    Xor,
}

pub enum UnaryOpType {
    Neg,
    Not,
}

pub enum Expression {
    Unary(UnaryOpType, Box<Expression>),
    Binary(BinaryOpType, Box<Expression>, Box<Expression>),
    ECall(Identifier, Vec<Expression>),
    Literal(Literal),
}

pub struct Signature {
    pub formal_parameters: Vec<(Identifier, Type)>,
    pub return_type: Type,
}

pub struct Function {
    pub signature: Signature,
    pub body: Expression,
}

pub struct Proc {
    pub signature: Signature,
    pub body: Statement,
}

pub struct PCall {
    pub proc: Identifier,
    pub arguments: Vec<Expression>,
}

//#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Statement {
    VarDefinition(Identifier),
    Expression(Expression),
    Seq(Box<Statement>, Box<Statement>),
    If(Expression, Box<Statement>, Box<Statement>),
    EAssignment(Vec<(Identifier, Expression)>), // x <- expr
    PAssignment(Identifier, PCall),             // x <@ proc
    Return(Expression),
    // SAssignment for // x <$ distr
    Pass,
}

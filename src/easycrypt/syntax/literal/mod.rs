use self::integer::IntegerLiteral;

pub mod integer;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal {
    String(String),
    Int(IntegerLiteral),
    Bool(bool),
}

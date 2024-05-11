#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntegerLiteral {
    Decimal { inner: String },
    Hexadecimal { inner: String },
}

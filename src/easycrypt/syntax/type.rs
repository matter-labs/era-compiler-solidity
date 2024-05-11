use std::fmt::Display;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Type {
    Unknown,
    Unit,
    Bool,
    Int(usize),
    UInt(usize),
    Custom(String),
    Tuple(Vec<Type>),
    Arrow(Box<Type>, Box<Type>),
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Unit => f.write_str("unit"),
            Type::Bool => f.write_str("bool"),
            Type::Int(size) => f.write_fmt(format_args!("int{}", size)),
            Type::UInt(size) => f.write_fmt(format_args!("uint{}", size)),
            Type::Custom(name) => f.write_str(name),
            Type::Arrow(lhs, rhs) => f.write_fmt(format_args!("{}->{}", lhs, rhs)),
            Type::Tuple(inner) => {
                f.write_str("(")?;
                for (i, component) in inner.iter().enumerate() {
                    component.fmt(f)?;
                    if i > 0 {
                        f.write_str(" * ")?;
                    }
                }
                f.write_str(")")
            }
            Type::Unknown => f.write_str("Unknown"),
        }
    }
}

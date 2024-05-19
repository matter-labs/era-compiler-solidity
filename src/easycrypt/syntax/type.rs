use std::fmt::Display;

use super::definition::Definition;

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
                    if i > 0 {
                        f.write_str(" * ")?;
                    }
                    component.fmt(f)?;
                }
                f.write_str(")")
            }
            Type::Unknown => f.write_str("Unknown"),
        }
    }
}
impl Type {
    /// Default type: currently only `UInt(256)` is used for all definitions, as
    /// this is the limitation of the current YUL dialect.
    pub const DEFAULT: &'static Type = &Type::UInt(256);

    /// Returns either:
    /// - `Type::Unit`, if [`definitions`] is empty;
    /// - The type of the first definition, if there is only one definition;
    /// - A tuple with types of all definitions otherwise.
    pub fn type_of_definitions(definitions: &[Definition]) -> Type {
        let vec: Vec<Type> = definitions
            .iter()
            .map(|d| d.r#type.clone().unwrap_or(Type::DEFAULT.clone()))
            .collect();
        match vec.len() {
            0 => Type::Unit,
            1 => vec[0].clone(),
            _ => Type::Tuple(vec),
        }
    }
}

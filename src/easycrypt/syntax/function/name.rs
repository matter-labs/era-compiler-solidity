//!
//! Name of a function in EasyCrypt, which can be a user-defined custom name or
//! one of the pre-defined names such as `lt`.
//!

use std::fmt::Display;

use crate::easycrypt::syntax::Name;

/// Name of a function, which can be a user-defined custom name or one of the pre-defined names such as `lt`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionName {
    /// The user-defined function.
    UserDefined(Name),
    /// `1` if `x < y`, `0` otherwise.
    Lt,
    /// `1` if `x > y`, `0` otherwise.
    Gt,
    /// `1` if `x < y`, `0` otherwise, for signed numbers in two’s complement.
    Slt,
    /// `1` if `x > y`, `0` otherwise, for signed numbers in two’s complement.
    Sgt,
    /// `x / y`, for signed numbers in two’s complement, `0` if `y == 0`.
    Sdiv,
    /// `x % y`, for signed numbers in two’s complement, `0` if `y == 0`.
    Smod,
    /// Arithmetic left shift.
    Shl,
    /// Arithmetic right shift.
    Shr,
    /// `1` if `x == 0`, `0` otherwise
    IsZero,
    /// `n`th byte of `x`, where the most significant byte is the `0`th byte
    Byte,
    /// signed arithmetic shift right `y` by `x` bits.
    Sar,
    /// `(x + y) % m` with arbitrary precision arithmetic, `0` if `m == 0`.
    AddMod,
    /// `(x * y) % m` with arbitrary precision arithmetic, `0` if `m == 0`.
    MulMod,
    /// sign extend from `(i*8+7)`th bit counting from least significant.
    SignExtend,
}

impl Display for FunctionName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let FunctionName::UserDefined(s) = self {
            f.write_str(s)
        } else {
            let str = match self {
                FunctionName::UserDefined(_) => unreachable!(),
                FunctionName::Lt => "lt",
                FunctionName::Gt => "gt",
                FunctionName::Slt => "slt",
                FunctionName::Sgt => "sgt",
                FunctionName::Sdiv => "sdiv",
                FunctionName::Smod => "smod",
                FunctionName::IsZero => "iszero",
                FunctionName::Byte => "byte",
                FunctionName::Sar => "sar",
                FunctionName::AddMod => "addmod",
                FunctionName::MulMod => "mulmod",
                FunctionName::SignExtend => "signext",
                FunctionName::Shl => "shl",
                FunctionName::Shr => "shr",
            };
            f.write_str(str)
        }
    }
}

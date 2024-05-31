//!
//! Kinds of functions
//!

use anyhow::Error;

use crate::easycrypt::syntax::expression::binary::BinaryOpType;
use crate::easycrypt::syntax::expression::unary::UnaryOpType;
use crate::easycrypt::syntax::function::name::FunctionName;
use crate::easycrypt::syntax::proc::name::ProcName;
use crate::easycrypt::translator::definition_info::kind::Kind;
use crate::easycrypt::translator::definition_info::DefinitionInfo;
use crate::yul::parser::statement::expression::function_call::name::Name as YulName;
use crate::yul::path::full_name::FullName;
use crate::yul::path::symbol_table::SymbolTable;
use crate::yul::path::Path;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum YulSpecial {
    Return,
    Revert,
    Stop,
    Invalid,
}

/// Kind of a function is a type of EasyCrypt syntax tree node that will match
/// such function.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FunctionKind {
    Function(FunctionName),
    Proc(ProcName),
    BinOp(BinaryOpType),
    UnOp(UnaryOpType),
    Special(YulSpecial),
}

/// Kinds of standard YUL functions, such as `lt` or `mstore`.
pub fn standard_function_kind(name: &YulName) -> Result<FunctionKind, Error> {
    match name {
        YulName::UserDefined(_) => anyhow::bail!("Non-standard function"),
        YulName::Add => Ok(FunctionKind::BinOp(BinaryOpType::Add)),
        YulName::Sub => Ok(FunctionKind::BinOp(BinaryOpType::Sub)),
        YulName::Mul => Ok(FunctionKind::BinOp(BinaryOpType::Mul)),
        YulName::Div => Ok(FunctionKind::BinOp(BinaryOpType::Div)),
        YulName::Mod => Ok(FunctionKind::BinOp(BinaryOpType::Mod)),
        YulName::Exp => Ok(FunctionKind::BinOp(BinaryOpType::Exp)),
        YulName::And => Ok(FunctionKind::BinOp(BinaryOpType::And)),
        YulName::Shl => Ok(FunctionKind::BinOp(BinaryOpType::Shl)),
        YulName::Shr => Ok(FunctionKind::BinOp(BinaryOpType::Shr)),
        YulName::Sar => Ok(FunctionKind::Function(FunctionName::Sar)),
        YulName::Eq => Ok(FunctionKind::BinOp(BinaryOpType::Eq)),
        YulName::Or => Ok(FunctionKind::BinOp(BinaryOpType::Or)),
        YulName::Xor => Ok(FunctionKind::BinOp(BinaryOpType::Xor)),

        YulName::Smod => Ok(FunctionKind::Function(FunctionName::Smod)),
        YulName::Sdiv => Ok(FunctionKind::Function(FunctionName::Sdiv)),
        YulName::Lt => Ok(FunctionKind::Function(FunctionName::Lt)),
        YulName::Gt => Ok(FunctionKind::Function(FunctionName::Gt)),
        YulName::IsZero => Ok(FunctionKind::Function(FunctionName::IsZero)),

        YulName::Slt => Ok(FunctionKind::Function(FunctionName::Slt)),
        YulName::Sgt => Ok(FunctionKind::Function(FunctionName::Sgt)),

        YulName::Not => Ok(FunctionKind::UnOp(UnaryOpType::Not)),

        YulName::Byte => Ok(FunctionKind::Function(FunctionName::Byte)),
        YulName::Pop => Ok(FunctionKind::Proc(ProcName::Pop)),
        YulName::AddMod => Ok(FunctionKind::Function(FunctionName::AddMod)),
        YulName::MulMod => Ok(FunctionKind::Function(FunctionName::MulMod)),
        YulName::SignExtend => Ok(FunctionKind::Function(FunctionName::SignExtend)),
        YulName::Keccak256 => Ok(FunctionKind::Proc(ProcName::Keccak256)),
        YulName::MLoad => Ok(FunctionKind::Proc(ProcName::MLoad)),
        YulName::MStore => Ok(FunctionKind::Proc(ProcName::MStore)),
        YulName::MStore8 => Ok(FunctionKind::Proc(ProcName::MStore8)),
        YulName::MCopy => Ok(FunctionKind::Proc(ProcName::MCopy)),
        YulName::SLoad => Ok(FunctionKind::Proc(ProcName::SLoad)),
        YulName::SStore => Ok(FunctionKind::Proc(ProcName::SStore)),
        YulName::TLoad => Ok(FunctionKind::Proc(ProcName::TLoad)),
        YulName::TStore => Ok(FunctionKind::Proc(ProcName::TStore)),
        YulName::LoadImmutable => Ok(FunctionKind::Proc(ProcName::LoadImmutable)),
        YulName::SetImmutable => Ok(FunctionKind::Proc(ProcName::SetImmutable)),
        YulName::CallDataLoad => Ok(FunctionKind::Proc(ProcName::CallDataLoad)),
        YulName::CallDataSize => Ok(FunctionKind::Proc(ProcName::CallDataSize)),
        YulName::CallDataCopy => Ok(FunctionKind::Proc(ProcName::CallDataCopy)),
        YulName::CodeSize => Ok(FunctionKind::Proc(ProcName::CodeSize)),
        YulName::CodeCopy => Ok(FunctionKind::Proc(ProcName::CodeCopy)),
        YulName::ExtCodeSize => Ok(FunctionKind::Proc(ProcName::ExtCodeSize)),
        YulName::ExtCodeHash => Ok(FunctionKind::Proc(ProcName::ExtCodeHash)),
        YulName::ReturnDataSize => Ok(FunctionKind::Proc(ProcName::ReturnDataSize)),
        YulName::ReturnDataCopy => Ok(FunctionKind::Proc(ProcName::ReturnDataCopy)),
        YulName::Log0 => Ok(FunctionKind::Proc(ProcName::Log0)),
        YulName::Log1 => Ok(FunctionKind::Proc(ProcName::Log1)),
        YulName::Log2 => Ok(FunctionKind::Proc(ProcName::Log2)),
        YulName::Log3 => Ok(FunctionKind::Proc(ProcName::Log3)),
        YulName::Log4 => Ok(FunctionKind::Proc(ProcName::Log4)),
        YulName::Call => Ok(FunctionKind::Proc(ProcName::Call)),
        YulName::CallCode => Ok(FunctionKind::Proc(ProcName::CallCode)),
        YulName::DelegateCall => Ok(FunctionKind::Proc(ProcName::DelegateCall)),
        YulName::StaticCall => Ok(FunctionKind::Proc(ProcName::StaticCall)),
        YulName::Create => Ok(FunctionKind::Proc(ProcName::Create)),
        YulName::Create2 => Ok(FunctionKind::Proc(ProcName::Create2)),
        YulName::ZkCreate => Ok(FunctionKind::Proc(ProcName::ZkCreate)),
        YulName::ZkCreate2 => Ok(FunctionKind::Proc(ProcName::ZkCreate2)),
        YulName::DataSize => Ok(FunctionKind::Proc(ProcName::DataSize)),
        YulName::DataCopy => Ok(FunctionKind::Proc(ProcName::DataCopy)),
        YulName::DataOffset => Ok(FunctionKind::Proc(ProcName::DataOffset)),
        YulName::LinkerSymbol => Ok(FunctionKind::Proc(ProcName::LinkerSymbol)),
        YulName::MemoryGuard => Ok(FunctionKind::Proc(ProcName::MemoryGuard)),
        YulName::Address => Ok(FunctionKind::Proc(ProcName::Address)),
        YulName::Caller => Ok(FunctionKind::Proc(ProcName::Caller)),
        YulName::CallValue => Ok(FunctionKind::Proc(ProcName::CallValue)),
        YulName::Gas => Ok(FunctionKind::Proc(ProcName::Gas)),
        YulName::Balance => Ok(FunctionKind::Proc(ProcName::Balance)),
        YulName::SelfBalance => Ok(FunctionKind::Proc(ProcName::SelfBalance)),
        YulName::GasLimit => Ok(FunctionKind::Proc(ProcName::GasLimit)),
        YulName::GasPrice => Ok(FunctionKind::Proc(ProcName::GasPrice)),
        YulName::Origin => Ok(FunctionKind::Proc(ProcName::Origin)),
        YulName::ChainId => Ok(FunctionKind::Proc(ProcName::ChainId)),
        YulName::Number => Ok(FunctionKind::Proc(ProcName::Number)),
        YulName::Timestamp => Ok(FunctionKind::Proc(ProcName::Timestamp)),
        YulName::BlockHash => Ok(FunctionKind::Proc(ProcName::BlockHash)),
        YulName::BlobHash => Ok(FunctionKind::Proc(ProcName::BlobHash)),
        YulName::Difficulty => Ok(FunctionKind::Proc(ProcName::Difficulty)),
        YulName::Prevrandao => Ok(FunctionKind::Proc(ProcName::Prevrandao)),
        YulName::CoinBase => Ok(FunctionKind::Proc(ProcName::CoinBase)),
        YulName::MSize => Ok(FunctionKind::Proc(ProcName::MSize)),
        YulName::Verbatim {
            input_size,
            output_size,
        } => Ok(FunctionKind::Proc(ProcName::Verbatim {
            input_size: *input_size,
            output_size: *output_size,
        })),
        YulName::BaseFee => Ok(FunctionKind::Proc(ProcName::BaseFee)),
        YulName::BlobBaseFee => Ok(FunctionKind::Proc(ProcName::BlobBaseFee)),
        YulName::Pc => Ok(FunctionKind::Proc(ProcName::Pc)),
        YulName::ExtCodeCopy => Ok(FunctionKind::Proc(ProcName::ExtCodeCopy)),
        YulName::SelfDestruct => Ok(FunctionKind::Proc(ProcName::SelfDestruct)),
        YulName::ZkToL1 => Ok(FunctionKind::Proc(ProcName::ZkToL1)),
        YulName::ZkCodeSource => Ok(FunctionKind::Proc(ProcName::ZkCodeSource)),
        YulName::ZkPrecompile => Ok(FunctionKind::Proc(ProcName::ZkPrecompile)),
        YulName::ZkMeta => Ok(FunctionKind::Proc(ProcName::ZkMeta)),
        YulName::ZkSetContextU128 => Ok(FunctionKind::Proc(ProcName::ZkSetContextU128)),
        YulName::ZkSetPubdataPrice => Ok(FunctionKind::Proc(ProcName::ZkSetPubdataPrice)),
        YulName::ZkIncrementTxCounter => Ok(FunctionKind::Proc(ProcName::ZkIncrementTxCounter)),
        YulName::ZkEventInitialize => Ok(FunctionKind::Proc(ProcName::ZkEventInitialize)),
        YulName::ZkEventWrite => Ok(FunctionKind::Proc(ProcName::ZkEventWrite)),
        YulName::ZkMimicCall => Ok(FunctionKind::Proc(ProcName::ZkMimicCall)),
        YulName::ZkSystemMimicCall => Ok(FunctionKind::Proc(ProcName::ZkSystemMimicCall)),
        YulName::ZkMimicCallByRef => Ok(FunctionKind::Proc(ProcName::ZkMimicCallByRef)),
        YulName::ZkSystemMimicCallByRef => Ok(FunctionKind::Proc(ProcName::ZkSystemMimicCallByRef)),
        YulName::ZkRawCall => Ok(FunctionKind::Proc(ProcName::ZkRawCall)),
        YulName::ZkRawCallByRef => Ok(FunctionKind::Proc(ProcName::ZkRawCallByRef)),
        YulName::ZkSystemCall => Ok(FunctionKind::Proc(ProcName::ZkSystemCall)),
        YulName::ZkSystemCallByRef => Ok(FunctionKind::Proc(ProcName::ZkSystemCallByRef)),
        YulName::ZkStaticRawCall => Ok(FunctionKind::Proc(ProcName::ZkStaticRawCall)),
        YulName::ZkStaticRawCallByRef => Ok(FunctionKind::Proc(ProcName::ZkStaticRawCallByRef)),
        YulName::ZkStaticSystemCall => Ok(FunctionKind::Proc(ProcName::ZkStaticSystemCall)),
        YulName::ZkStaticSystemCallByRef => {
            Ok(FunctionKind::Proc(ProcName::ZkStaticSystemCallByRef))
        }
        YulName::ZkDelegateRawCall => Ok(FunctionKind::Proc(ProcName::ZkDelegateRawCall)),
        YulName::ZkDelegateRawCallByRef => Ok(FunctionKind::Proc(ProcName::ZkDelegateRawCallByRef)),
        YulName::ZkDelegateSystemCall => Ok(FunctionKind::Proc(ProcName::ZkDelegateSystemCall)),
        YulName::ZkDelegateSystemCallByRef => {
            Ok(FunctionKind::Proc(ProcName::ZkDelegateSystemCallByRef))
        }
        YulName::ZkLoadCalldataIntoActivePtr => {
            Ok(FunctionKind::Proc(ProcName::ZkLoadCalldataIntoActivePtr))
        }
        YulName::ZkLoadReturndataIntoActivePtr => {
            Ok(FunctionKind::Proc(ProcName::ZkLoadReturndataIntoActivePtr))
        }
        YulName::ZkPtrAddIntoActive => Ok(FunctionKind::Proc(ProcName::ZkPtrAddIntoActive)),
        YulName::ZkPtrShrinkIntoActive => Ok(FunctionKind::Proc(ProcName::ZkPtrShrinkIntoActive)),
        YulName::ZkPtrPackIntoActive => Ok(FunctionKind::Proc(ProcName::ZkPtrPackIntoActive)),
        YulName::ZkMultiplicationHigh => Ok(FunctionKind::Proc(ProcName::ZkMultiplicationHigh)),
        YulName::ZkGlobalLoad => Ok(FunctionKind::Proc(ProcName::ZkGlobalLoad)),
        YulName::ZkGlobalExtraAbiData => Ok(FunctionKind::Proc(ProcName::ZkGlobalExtraAbiData)),
        YulName::ZkGlobalStore => Ok(FunctionKind::Proc(ProcName::ZkGlobalStore)),
        YulName::Return => Ok(FunctionKind::Special(YulSpecial::Return)),
        YulName::Revert => Ok(FunctionKind::Special(YulSpecial::Revert)),
        YulName::Stop => Ok(FunctionKind::Special(YulSpecial::Stop)),
        YulName::Invalid => Ok(FunctionKind::Special(YulSpecial::Invalid)),
    }
}

/// Derive YUL function kind based on its body.
/// YUL function can be translated into a function if two conditions are
/// satisfied:
/// 1. Its body is a single assignment to its return variables.
/// 2. The initializer for these variables do not contain calls to such
///    functions that would be translated into EasyCrypt procedures.
pub fn derive_kind(
    environment: &SymbolTable<DefinitionInfo>,
    name: &YulName,
    path: &Path,
) -> Result<FunctionKind, Error> {
    match name {
        YulName::UserDefined(name_str) => {
            let full_name = FullName {
                name: name_str.to_string(),
                path: path.clone(),
            };
            let definition = environment.get(&full_name);
            match definition {
                Some(DefinitionInfo { kind, .. }) => match kind {
                    Kind::Function => Ok(FunctionKind::Function(FunctionName::UserDefined(
                        name_str.to_owned(),
                    ))),
                    Kind::Procedure => Ok(FunctionKind::Proc(ProcName::UserDefined(
                        name_str.to_owned(),
                    ))),
                    Kind::Variable => anyhow::bail!(
                        "Malformed YUL: variable {} shadows a function with the same name.",
                        name_str
                    ),
                },
                None => anyhow::bail!(
                    "Can not find user-defined function {} among the definitions",
                    name_str
                ),
            }
        }
        standard_function => standard_function_kind(standard_function),
    }
}

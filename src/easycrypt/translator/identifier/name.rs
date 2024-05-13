//!
//! Transpilation of the names of YUL identifiers.
//!


use crate::easycrypt::syntax::expression::binary::BinaryOpType;
use crate::easycrypt::syntax::expression::unary::UnaryOpType;
use crate::easycrypt::syntax::function::name::FunctionName;
use crate::easycrypt::syntax::module::definition::TopDefinition;
use crate::easycrypt::syntax::proc::name::ProcName;

use crate::easycrypt::translator::context::Context;
use crate::yul::parser::statement::expression::function_call::name::Name as YulName;

use crate::easycrypt::translator::identifier::special::YulSpecial;
use crate::easycrypt::translator::identifier::Translated;
use crate::Translator;

impl Translator {
    /// Transpile an arbitrary YUL identifier's name, which can be a
    /// user-defined custom name or a predefined name like `lt` of `call`.
    pub fn transpile_name(&mut self, ctx: &Context, name: &YulName) -> Translated {
        match name {
            YulName::UserDefined(name_str) => match self.get_module_definition(ctx, name_str) {
                Some(TopDefinition::Function(_)) => {
                    Translated::Function(FunctionName::UserDefined(name_str.clone()))
                }
                Some(TopDefinition::Proc(_)) => {
                    Translated::Proc(ProcName::UserDefined(name_str.clone()))
                }
                None => Translated::ProcOrFunction(name_str.clone()),
            },
            YulName::Add => Translated::BinOp(BinaryOpType::Add),
            YulName::Sub => Translated::BinOp(BinaryOpType::Sub),
            YulName::Mul => Translated::BinOp(BinaryOpType::Mul),
            YulName::Div => Translated::BinOp(BinaryOpType::Div),
            YulName::Mod => Translated::BinOp(BinaryOpType::Mod),
            YulName::Exp => Translated::BinOp(BinaryOpType::Exp),
            YulName::And => Translated::BinOp(BinaryOpType::And),
            YulName::Shl => Translated::BinOp(BinaryOpType::Shl),
            YulName::Shr => Translated::BinOp(BinaryOpType::Shr),
            YulName::Sar => Translated::Function(FunctionName::Sar),
            YulName::Eq => Translated::BinOp(BinaryOpType::Eq),
            YulName::Or => Translated::BinOp(BinaryOpType::Or),
            YulName::Xor => Translated::BinOp(BinaryOpType::Xor),

            YulName::Smod => Translated::Function(FunctionName::Smod),
            YulName::Sdiv => Translated::Function(FunctionName::Sdiv),
            YulName::Lt => Translated::Function(FunctionName::Lt),
            YulName::Gt => Translated::Function(FunctionName::Gt),
            YulName::IsZero => Translated::Function(FunctionName::IsZero),

            YulName::Slt => Translated::Function(FunctionName::Slt),
            YulName::Sgt => Translated::Function(FunctionName::Sgt),

            YulName::Not => Translated::UnOp(UnaryOpType::Not),

            YulName::Byte => Translated::Function(FunctionName::Byte),
            YulName::Pop => Translated::Proc(ProcName::Pop),
            YulName::AddMod => Translated::Function(FunctionName::AddMod),
            YulName::MulMod => Translated::Function(FunctionName::MulMod),
            YulName::SignExtend => Translated::Function(FunctionName::SignExtend),
            YulName::Keccak256 => Translated::Proc(ProcName::Keccak256),
            YulName::MLoad => Translated::Proc(ProcName::MLoad),
            YulName::MStore => Translated::Proc(ProcName::MStore),
            YulName::MStore8 => Translated::Proc(ProcName::MStore8),
            YulName::MCopy => Translated::Proc(ProcName::MCopy),
            YulName::SLoad => Translated::Proc(ProcName::SLoad),
            YulName::SStore => Translated::Proc(ProcName::SStore),
            YulName::TLoad => Translated::Proc(ProcName::TLoad),
            YulName::TStore => Translated::Proc(ProcName::TStore),
            YulName::LoadImmutable => Translated::Proc(ProcName::LoadImmutable),
            YulName::SetImmutable => Translated::Proc(ProcName::SetImmutable),
            YulName::CallDataLoad => Translated::Proc(ProcName::CallDataLoad),
            YulName::CallDataSize => Translated::Proc(ProcName::CallDataSize),
            YulName::CallDataCopy => Translated::Proc(ProcName::CallDataCopy),
            YulName::CodeSize => Translated::Proc(ProcName::CodeSize),
            YulName::CodeCopy => Translated::Proc(ProcName::CodeCopy),
            YulName::ExtCodeSize => Translated::Proc(ProcName::ExtCodeSize),
            YulName::ExtCodeHash => Translated::Proc(ProcName::ExtCodeHash),
            YulName::ReturnDataSize => Translated::Proc(ProcName::ReturnDataSize),
            YulName::ReturnDataCopy => Translated::Proc(ProcName::ReturnDataCopy),
            // YulName::Return => Translated::Proc(ProcName::Return),
            // YulName::Revert => Translated::Proc(ProcName::Revert),
            // YulName::Stop => Translated::Proc(ProcName::Stop),
            // YulName::Invalid => Translated::Proc(ProcName::Invalid),
            YulName::Log0 => Translated::Proc(ProcName::Log0),
            YulName::Log1 => Translated::Proc(ProcName::Log1),
            YulName::Log2 => Translated::Proc(ProcName::Log2),
            YulName::Log3 => Translated::Proc(ProcName::Log3),
            YulName::Log4 => Translated::Proc(ProcName::Log4),
            YulName::Call => Translated::Proc(ProcName::Call),
            YulName::CallCode => Translated::Proc(ProcName::CallCode),
            YulName::DelegateCall => Translated::Proc(ProcName::DelegateCall),
            YulName::StaticCall => Translated::Proc(ProcName::StaticCall),
            YulName::Create => Translated::Proc(ProcName::Create),
            YulName::Create2 => Translated::Proc(ProcName::Create2),
            YulName::ZkCreate => Translated::Proc(ProcName::ZkCreate),
            YulName::ZkCreate2 => Translated::Proc(ProcName::ZkCreate2),
            YulName::DataSize => Translated::Proc(ProcName::DataSize),
            YulName::DataCopy => Translated::Proc(ProcName::DataCopy),
            YulName::DataOffset => Translated::Proc(ProcName::DataOffset),
            YulName::LinkerSymbol => Translated::Proc(ProcName::LinkerSymbol),
            YulName::MemoryGuard => Translated::Proc(ProcName::MemoryGuard),
            YulName::Address => Translated::Proc(ProcName::Address),
            YulName::Caller => Translated::Proc(ProcName::Caller),
            YulName::CallValue => Translated::Proc(ProcName::CallValue),
            YulName::Gas => Translated::Proc(ProcName::Gas),
            YulName::Balance => Translated::Proc(ProcName::Balance),
            YulName::SelfBalance => Translated::Proc(ProcName::SelfBalance),
            YulName::GasLimit => Translated::Proc(ProcName::GasLimit),
            YulName::GasPrice => Translated::Proc(ProcName::GasPrice),
            YulName::Origin => Translated::Proc(ProcName::Origin),
            YulName::ChainId => Translated::Proc(ProcName::ChainId),
            YulName::Number => Translated::Proc(ProcName::Number),
            YulName::Timestamp => Translated::Proc(ProcName::Timestamp),
            YulName::BlockHash => Translated::Proc(ProcName::BlockHash),
            YulName::BlobHash => Translated::Proc(ProcName::BlobHash),
            YulName::Difficulty => Translated::Proc(ProcName::Difficulty),
            YulName::Prevrandao => Translated::Proc(ProcName::Prevrandao),
            YulName::CoinBase => Translated::Proc(ProcName::CoinBase),
            YulName::MSize => Translated::Proc(ProcName::MSize),
            YulName::Verbatim {
                input_size,
                output_size,
            } => Translated::Proc(ProcName::Verbatim {
                input_size: *input_size,
                output_size: *output_size,
            }),
            YulName::BaseFee => Translated::Proc(ProcName::BaseFee),
            YulName::BlobBaseFee => Translated::Proc(ProcName::BlobBaseFee),
            YulName::Pc => Translated::Proc(ProcName::Pc),
            YulName::ExtCodeCopy => Translated::Proc(ProcName::ExtCodeCopy),
            YulName::SelfDestruct => Translated::Proc(ProcName::SelfDestruct),
            YulName::ZkToL1 => Translated::Proc(ProcName::ZkToL1),
            YulName::ZkCodeSource => Translated::Proc(ProcName::ZkCodeSource),
            YulName::ZkPrecompile => Translated::Proc(ProcName::ZkPrecompile),
            YulName::ZkMeta => Translated::Proc(ProcName::ZkMeta),
            YulName::ZkSetContextU128 => Translated::Proc(ProcName::ZkSetContextU128),
            YulName::ZkSetPubdataPrice => Translated::Proc(ProcName::ZkSetPubdataPrice),
            YulName::ZkIncrementTxCounter => Translated::Proc(ProcName::ZkIncrementTxCounter),
            YulName::ZkEventInitialize => Translated::Proc(ProcName::ZkEventInitialize),
            YulName::ZkEventWrite => Translated::Proc(ProcName::ZkEventWrite),
            YulName::ZkMimicCall => Translated::Proc(ProcName::ZkMimicCall),
            YulName::ZkSystemMimicCall => Translated::Proc(ProcName::ZkSystemMimicCall),
            YulName::ZkMimicCallByRef => Translated::Proc(ProcName::ZkMimicCallByRef),
            YulName::ZkSystemMimicCallByRef => Translated::Proc(ProcName::ZkSystemMimicCallByRef),
            YulName::ZkRawCall => Translated::Proc(ProcName::ZkRawCall),
            YulName::ZkRawCallByRef => Translated::Proc(ProcName::ZkRawCallByRef),
            YulName::ZkSystemCall => Translated::Proc(ProcName::ZkSystemCall),
            YulName::ZkSystemCallByRef => Translated::Proc(ProcName::ZkSystemCallByRef),
            YulName::ZkStaticRawCall => Translated::Proc(ProcName::ZkStaticRawCall),
            YulName::ZkStaticRawCallByRef => Translated::Proc(ProcName::ZkStaticRawCallByRef),
            YulName::ZkStaticSystemCall => Translated::Proc(ProcName::ZkStaticSystemCall),
            YulName::ZkStaticSystemCallByRef => Translated::Proc(ProcName::ZkStaticSystemCallByRef),
            YulName::ZkDelegateRawCall => Translated::Proc(ProcName::ZkDelegateRawCall),
            YulName::ZkDelegateRawCallByRef => Translated::Proc(ProcName::ZkDelegateRawCallByRef),
            YulName::ZkDelegateSystemCall => Translated::Proc(ProcName::ZkDelegateSystemCall),
            YulName::ZkDelegateSystemCallByRef => {
                Translated::Proc(ProcName::ZkDelegateSystemCallByRef)
            }
            YulName::ZkLoadCalldataIntoActivePtr => {
                Translated::Proc(ProcName::ZkLoadCalldataIntoActivePtr)
            }
            YulName::ZkLoadReturndataIntoActivePtr => {
                Translated::Proc(ProcName::ZkLoadReturndataIntoActivePtr)
            }
            YulName::ZkPtrAddIntoActive => Translated::Proc(ProcName::ZkPtrAddIntoActive),
            YulName::ZkPtrShrinkIntoActive => Translated::Proc(ProcName::ZkPtrShrinkIntoActive),
            YulName::ZkPtrPackIntoActive => Translated::Proc(ProcName::ZkPtrPackIntoActive),
            YulName::ZkMultiplicationHigh => Translated::Proc(ProcName::ZkMultiplicationHigh),
            YulName::ZkGlobalLoad => Translated::Proc(ProcName::ZkGlobalLoad),
            YulName::ZkGlobalExtraAbiData => Translated::Proc(ProcName::ZkGlobalExtraAbiData),
            YulName::ZkGlobalStore => Translated::Proc(ProcName::ZkGlobalStore),
            YulName::Return => Translated::Special(YulSpecial::Return),
            YulName::Revert => Translated::Special(YulSpecial::Revert),
            YulName::Stop => Translated::Special(YulSpecial::Stop),
            YulName::Invalid => Translated::Special(YulSpecial::Invalid),
        }
    }
}

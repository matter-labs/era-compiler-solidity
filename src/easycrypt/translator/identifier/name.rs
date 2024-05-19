//!
//! Transpilation of the names of YUL identifiers.
//!

use anyhow::Error;

use crate::easycrypt::syntax::expression::binary::BinaryOpType;
use crate::easycrypt::syntax::expression::unary::UnaryOpType;
use crate::easycrypt::syntax::function::name::FunctionName;
use crate::easycrypt::syntax::proc::name::ProcName;

use crate::easycrypt::translator::context::Context;
use crate::easycrypt::translator::definition_info::kind::Kind;
use crate::easycrypt::translator::definition_info::DefinitionInfo;
use crate::yul::parser::statement::expression::function_call::name::Name as YulName;

use crate::easycrypt::translator::identifier::special::YulSpecial;
use crate::easycrypt::translator::identifier::Translated;
use crate::Translator;

impl Translator {
    /// Transpile an arbitrary YUL identifier's name, which can be a
    /// user-defined custom name or a predefined name like `lt` of `call`.
    pub fn transpile_name(&mut self, _ctx: &Context, name: &YulName) -> Result<Translated, Error> {
        match name {
            YulName::UserDefined(name_str) => match self.tracker.get(name_str) {
                Some(DefinitionInfo { kind, .. }) => {
                    match kind {
                        Kind::Procedure =>
                            Ok(Translated::Proc(ProcName::UserDefined(name_str.clone()))),
                        Kind::Function => Ok(Translated::Function(FunctionName::UserDefined(name_str.clone()))),
                        Kind::Variable => anyhow::bail!("Invalid YUL: a variable name clashes with the name of a previously defined function or a predefined function name."),
                    }
                },
                None => Ok(Translated::ProcOrFunction(name_str.clone())),
            },
            YulName::Add => Ok(Translated::BinOp(BinaryOpType::Add)),
            YulName::Sub => Ok(Translated::BinOp(BinaryOpType::Sub)),
            YulName::Mul => Ok(Translated::BinOp(BinaryOpType::Mul)),
            YulName::Div => Ok(Translated::BinOp(BinaryOpType::Div)),
            YulName::Mod => Ok(Translated::BinOp(BinaryOpType::Mod)),
            YulName::Exp => Ok(Translated::BinOp(BinaryOpType::Exp)),
            YulName::And => Ok(Translated::BinOp(BinaryOpType::And)),
            YulName::Shl => Ok(Translated::BinOp(BinaryOpType::Shl)),
            YulName::Shr => Ok(Translated::BinOp(BinaryOpType::Shr)),
            YulName::Sar => Ok(Translated::Function(FunctionName::Sar)),
            YulName::Eq => Ok(Translated::BinOp(BinaryOpType::Eq)),
            YulName::Or => Ok(Translated::BinOp(BinaryOpType::Or)),
            YulName::Xor => Ok(Translated::BinOp(BinaryOpType::Xor)),

            YulName::Smod => Ok(Translated::Function(FunctionName::Smod)),
            YulName::Sdiv => Ok(Translated::Function(FunctionName::Sdiv)),
            YulName::Lt => Ok(Translated::Function(FunctionName::Lt)),
            YulName::Gt => Ok(Translated::Function(FunctionName::Gt)),
            YulName::IsZero => Ok(Translated::Function(FunctionName::IsZero)),

            YulName::Slt => Ok(Translated::Function(FunctionName::Slt)),
            YulName::Sgt => Ok(Translated::Function(FunctionName::Sgt)),

            YulName::Not => Ok(Translated::UnOp(UnaryOpType::Not)),

            YulName::Byte => Ok(Translated::Function(FunctionName::Byte)),
            YulName::Pop => Ok(Translated::Proc(ProcName::Pop)),
            YulName::AddMod => Ok(Translated::Function(FunctionName::AddMod)),
            YulName::MulMod => Ok(Translated::Function(FunctionName::MulMod)),
            YulName::SignExtend => Ok(Translated::Function(FunctionName::SignExtend)),
            YulName::Keccak256 => Ok(Translated::Proc(ProcName::Keccak256)),
            YulName::MLoad => Ok(Translated::Proc(ProcName::MLoad)),
            YulName::MStore => Ok(Translated::Proc(ProcName::MStore)),
            YulName::MStore8 => Ok(Translated::Proc(ProcName::MStore8)),
            YulName::MCopy => Ok(Translated::Proc(ProcName::MCopy)),
            YulName::SLoad => Ok(Translated::Proc(ProcName::SLoad)),
            YulName::SStore => Ok(Translated::Proc(ProcName::SStore)),
            YulName::TLoad => Ok(Translated::Proc(ProcName::TLoad)),
            YulName::TStore => Ok(Translated::Proc(ProcName::TStore)),
            YulName::LoadImmutable => Ok(Translated::Proc(ProcName::LoadImmutable)),
            YulName::SetImmutable => Ok(Translated::Proc(ProcName::SetImmutable)),
            YulName::CallDataLoad => Ok(Translated::Proc(ProcName::CallDataLoad)),
            YulName::CallDataSize => Ok(Translated::Proc(ProcName::CallDataSize)),
            YulName::CallDataCopy => Ok(Translated::Proc(ProcName::CallDataCopy)),
            YulName::CodeSize => Ok(Translated::Proc(ProcName::CodeSize)),
            YulName::CodeCopy => Ok(Translated::Proc(ProcName::CodeCopy)),
            YulName::ExtCodeSize => Ok(Translated::Proc(ProcName::ExtCodeSize)),
            YulName::ExtCodeHash => Ok(Translated::Proc(ProcName::ExtCodeHash)),
            YulName::ReturnDataSize => Ok(Translated::Proc(ProcName::ReturnDataSize)),
            YulName::ReturnDataCopy => Ok(Translated::Proc(ProcName::ReturnDataCopy)),
            // YulName::Return => Ok(Translated::Proc(ProcName::Return)),
            // YulName::Revert => Ok(Translated::Proc(ProcName::Revert)),
            // YulName::Stop => Ok(Translated::Proc(ProcName::Stop)),
            // YulName::Invalid => Ok(Translated::Proc(ProcName::Invalid)),
            YulName::Log0 => Ok(Translated::Proc(ProcName::Log0)),
            YulName::Log1 => Ok(Translated::Proc(ProcName::Log1)),
            YulName::Log2 => Ok(Translated::Proc(ProcName::Log2)),
            YulName::Log3 => Ok(Translated::Proc(ProcName::Log3)),
            YulName::Log4 => Ok(Translated::Proc(ProcName::Log4)),
            YulName::Call => Ok(Translated::Proc(ProcName::Call)),
            YulName::CallCode => Ok(Translated::Proc(ProcName::CallCode)),
            YulName::DelegateCall => Ok(Translated::Proc(ProcName::DelegateCall)),
            YulName::StaticCall => Ok(Translated::Proc(ProcName::StaticCall)),
            YulName::Create => Ok(Translated::Proc(ProcName::Create)),
            YulName::Create2 => Ok(Translated::Proc(ProcName::Create2)),
            YulName::ZkCreate => Ok(Translated::Proc(ProcName::ZkCreate)),
            YulName::ZkCreate2 => Ok(Translated::Proc(ProcName::ZkCreate2)),
            YulName::DataSize => Ok(Translated::Proc(ProcName::DataSize)),
            YulName::DataCopy => Ok(Translated::Proc(ProcName::DataCopy)),
            YulName::DataOffset => Ok(Translated::Proc(ProcName::DataOffset)),
            YulName::LinkerSymbol => Ok(Translated::Proc(ProcName::LinkerSymbol)),
            YulName::MemoryGuard => Ok(Translated::Proc(ProcName::MemoryGuard)),
            YulName::Address => Ok(Translated::Proc(ProcName::Address)),
            YulName::Caller => Ok(Translated::Proc(ProcName::Caller)),
            YulName::CallValue => Ok(Translated::Proc(ProcName::CallValue)),
            YulName::Gas => Ok(Translated::Proc(ProcName::Gas)),
            YulName::Balance => Ok(Translated::Proc(ProcName::Balance)),
            YulName::SelfBalance => Ok(Translated::Proc(ProcName::SelfBalance)),
            YulName::GasLimit => Ok(Translated::Proc(ProcName::GasLimit)),
            YulName::GasPrice => Ok(Translated::Proc(ProcName::GasPrice)),
            YulName::Origin => Ok(Translated::Proc(ProcName::Origin)),
            YulName::ChainId => Ok(Translated::Proc(ProcName::ChainId)),
            YulName::Number => Ok(Translated::Proc(ProcName::Number)),
            YulName::Timestamp => Ok(Translated::Proc(ProcName::Timestamp)),
            YulName::BlockHash => Ok(Translated::Proc(ProcName::BlockHash)),
            YulName::BlobHash => Ok(Translated::Proc(ProcName::BlobHash)),
            YulName::Difficulty => Ok(Translated::Proc(ProcName::Difficulty)),
            YulName::Prevrandao => Ok(Translated::Proc(ProcName::Prevrandao)),
            YulName::CoinBase => Ok(Translated::Proc(ProcName::CoinBase)),
            YulName::MSize => Ok(Translated::Proc(ProcName::MSize)),
            YulName::Verbatim {
                input_size,
                output_size,
            } => Ok(Translated::Proc(ProcName::Verbatim {
                input_size: *input_size,
                output_size: *output_size,
            })),
            YulName::BaseFee => Ok(Translated::Proc(ProcName::BaseFee)),
            YulName::BlobBaseFee => Ok(Translated::Proc(ProcName::BlobBaseFee)),
            YulName::Pc => Ok(Translated::Proc(ProcName::Pc)),
            YulName::ExtCodeCopy => Ok(Translated::Proc(ProcName::ExtCodeCopy)),
            YulName::SelfDestruct => Ok(Translated::Proc(ProcName::SelfDestruct)),
            YulName::ZkToL1 => Ok(Translated::Proc(ProcName::ZkToL1)),
            YulName::ZkCodeSource => Ok(Translated::Proc(ProcName::ZkCodeSource)),
            YulName::ZkPrecompile => Ok(Translated::Proc(ProcName::ZkPrecompile)),
            YulName::ZkMeta => Ok(Translated::Proc(ProcName::ZkMeta)),
            YulName::ZkSetContextU128 => Ok(Translated::Proc(ProcName::ZkSetContextU128)),
            YulName::ZkSetPubdataPrice => Ok(Translated::Proc(ProcName::ZkSetPubdataPrice)),
            YulName::ZkIncrementTxCounter => Ok(Translated::Proc(ProcName::ZkIncrementTxCounter)),
            YulName::ZkEventInitialize => Ok(Translated::Proc(ProcName::ZkEventInitialize)),
            YulName::ZkEventWrite => Ok(Translated::Proc(ProcName::ZkEventWrite)),
            YulName::ZkMimicCall => Ok(Translated::Proc(ProcName::ZkMimicCall)),
            YulName::ZkSystemMimicCall => Ok(Translated::Proc(ProcName::ZkSystemMimicCall)),
            YulName::ZkMimicCallByRef => Ok(Translated::Proc(ProcName::ZkMimicCallByRef)),
            YulName::ZkSystemMimicCallByRef => Ok(Translated::Proc(ProcName::ZkSystemMimicCallByRef)),
            YulName::ZkRawCall => Ok(Translated::Proc(ProcName::ZkRawCall)),
            YulName::ZkRawCallByRef => Ok(Translated::Proc(ProcName::ZkRawCallByRef)),
            YulName::ZkSystemCall => Ok(Translated::Proc(ProcName::ZkSystemCall)),
            YulName::ZkSystemCallByRef => Ok(Translated::Proc(ProcName::ZkSystemCallByRef)),
            YulName::ZkStaticRawCall => Ok(Translated::Proc(ProcName::ZkStaticRawCall)),
            YulName::ZkStaticRawCallByRef => Ok(Translated::Proc(ProcName::ZkStaticRawCallByRef)),
            YulName::ZkStaticSystemCall => Ok(Translated::Proc(ProcName::ZkStaticSystemCall)),
            YulName::ZkStaticSystemCallByRef => Ok(Translated::Proc(ProcName::ZkStaticSystemCallByRef)),
            YulName::ZkDelegateRawCall => Ok(Translated::Proc(ProcName::ZkDelegateRawCall)),
            YulName::ZkDelegateRawCallByRef => Ok(Translated::Proc(ProcName::ZkDelegateRawCallByRef)),
            YulName::ZkDelegateSystemCall => Ok(Translated::Proc(ProcName::ZkDelegateSystemCall)),
            YulName::ZkDelegateSystemCallByRef => {
                Ok(Translated::Proc(ProcName::ZkDelegateSystemCallByRef))
            },
            YulName::ZkLoadCalldataIntoActivePtr => {
                Ok(Translated::Proc(ProcName::ZkLoadCalldataIntoActivePtr))
            },
            YulName::ZkLoadReturndataIntoActivePtr => {
                Ok(Translated::Proc(ProcName::ZkLoadReturndataIntoActivePtr))
            },
            YulName::ZkPtrAddIntoActive => Ok(Translated::Proc(ProcName::ZkPtrAddIntoActive)),
            YulName::ZkPtrShrinkIntoActive => Ok(Translated::Proc(ProcName::ZkPtrShrinkIntoActive)),
            YulName::ZkPtrPackIntoActive => Ok(Translated::Proc(ProcName::ZkPtrPackIntoActive)),
            YulName::ZkMultiplicationHigh => Ok(Translated::Proc(ProcName::ZkMultiplicationHigh)),
            YulName::ZkGlobalLoad => Ok(Translated::Proc(ProcName::ZkGlobalLoad)),
            YulName::ZkGlobalExtraAbiData => Ok(Translated::Proc(ProcName::ZkGlobalExtraAbiData)),
            YulName::ZkGlobalStore => Ok(Translated::Proc(ProcName::ZkGlobalStore)),
            YulName::Return => Ok(Translated::Special(YulSpecial::Return)),
            YulName::Revert => Ok(Translated::Special(YulSpecial::Revert)),
            YulName::Stop => Ok(Translated::Special(YulSpecial::Stop)),
            YulName::Invalid => Ok(Translated::Special(YulSpecial::Invalid)),
        }
    }
}

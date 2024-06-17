//!
//! [`DescriptionInfo`] instances for transpiled images of standard YUL definitions.
//!

use anyhow::Error;

use crate::easycrypt::syntax::expression::binary::BinaryOpType;
use crate::easycrypt::syntax::expression::unary::UnaryOpType;
use crate::easycrypt::syntax::function::name::FunctionName;
use crate::easycrypt::syntax::proc::name::ProcName;
use crate::easycrypt::syntax::r#type::Type;
use crate::easycrypt::translator::definition_info::attributes::Attributes;
use crate::easycrypt::translator::definition_info::kind::proc_kind::ProcKind;
use crate::easycrypt::translator::definition_info::kind::Kind;
use crate::easycrypt::translator::definition_info::kind::YulSpecial;
use crate::easycrypt::translator::definition_info::usage::Usage;
use crate::yul::parser::statement::expression::function_call::name::Name as YulName;
use crate::yul::path::full_name::FullName;
use crate::yul::path::Path;

use super::DefinitionInfo;

// FIXME: this could be more efficient

/// Get a definition for a standard YUL function.
pub fn standard_function_definition(yul_name: &YulName) -> Result<DefinitionInfo, Error> {
    fn def_type() -> Type {
        Type::DEFAULT.clone()
    }

    fn arrow_type(arity: usize) -> Type {
        let inputs = Type::type_of_vec(
            &std::iter::repeat(def_type())
                .take(arity)
                .collect::<Vec<_>>(),
        );
        Type::Arrow(Box::from(inputs), Box::from(def_type()))
    }
    fn arrow_type_proc(in_arity: usize, out_arity: usize) -> Type {
        let inputs = Type::type_of_vec(
            &std::iter::repeat(def_type())
                .take(in_arity)
                .collect::<Vec<_>>(),
        );
        let outputs = Type::type_of_vec(
            &std::iter::repeat(def_type())
                .take(out_arity)
                .collect::<Vec<_>>(),
        );
        Type::Arrow(Box::from(inputs), Box::from(outputs))
    }
    fn full_name(s: &str) -> FullName {
        FullName {
            name: s.to_string(),
            path: Path::empty(),
        }
    }
    fn unop(typ: UnaryOpType, name: &str) -> Result<DefinitionInfo, Error> {
        Ok(DefinitionInfo {
            kind: Kind::UnOp(typ),
            full_name: full_name(name),
            r#type: arrow_type(1),
        })
    }
    fn binop(typ: BinaryOpType, name: &str) -> Result<DefinitionInfo, Error> {
        Ok(DefinitionInfo {
            kind: Kind::BinOp(typ),
            full_name: full_name(name),
            r#type: arrow_type(2),
        })
    }

    fn fun(typ: FunctionName, name_str: &str, input_args: usize) -> Result<DefinitionInfo, Error> {
        Ok(DefinitionInfo {
            kind: Kind::Function(typ),
            full_name: full_name(name_str),
            r#type: arrow_type(input_args),
        })
    }

    fn proc(
        name: ProcName,
        name_str: &str,
        input_args: usize,
        output_args: usize,
        attributes: Attributes,
    ) -> Result<DefinitionInfo, Error> {
        Ok(DefinitionInfo {
            kind: Kind::Proc(ProcKind { name, attributes }),
            full_name: full_name(name_str),
            r#type: arrow_type_proc(input_args, output_args),
        })
    }
    fn proc_simple(
        name: ProcName,
        name_str: &str,
        input_args: usize,
        output_args: usize,
    ) -> Result<DefinitionInfo, Error> {
        let attributes = Attributes::default();
        proc(name, name_str, input_args, output_args, attributes)
    }
    fn proc_mem(
        name: ProcName,
        name_str: &str,
        usage: Usage,
        input_args: usize,
        output_args: usize,
    ) -> Result<DefinitionInfo, Error> {
        // Currently the memory parameter propagation is disabled.
        //let attributes = Attributes::heap(usage);
        let _ = usage;
        let attributes = Attributes::default();
        proc(name, name_str, input_args, output_args, attributes)
    }
    fn proc_storage(
        name: ProcName,
        name_str: &str,
        usage: Usage,
        input_args: usize,
        output_args: usize,
    ) -> Result<DefinitionInfo, Error> {
        let attributes = Attributes::storage(usage);
        proc(name, name_str, input_args, output_args, attributes)
    }

    fn proc_transient(
        name: ProcName,
        name_str: &str,
        usage: Usage,
        input_args: usize,
        output_args: usize,
    ) -> Result<DefinitionInfo, Error> {
        let attributes = Attributes::transient(usage);
        proc(name, name_str, input_args, output_args, attributes)
    }
    fn proc_other(
        name: ProcName,
        name_str: &str,
        usage: Usage,
        input_args: usize,
        output_args: usize,
    ) -> Result<DefinitionInfo, Error> {
        let attributes = Attributes::other(usage);
        proc(name, name_str, input_args, output_args, attributes)
    }

    match yul_name {
        YulName::Add => binop(BinaryOpType::Add, "add"),
        YulName::Sub => binop(BinaryOpType::Sub, "sub"),
        YulName::Mul => binop(BinaryOpType::Mul, "mul"),
        YulName::Div => binop(BinaryOpType::Div, "div"),
        YulName::Mod => binop(BinaryOpType::Mod, "mod"),
        YulName::Exp => binop(BinaryOpType::Exp, "exp"),
        YulName::And => binop(BinaryOpType::And, "and"),
        YulName::Shl => fun(FunctionName::Shl, "shl", 2),
        YulName::Shr => fun(FunctionName::Shr, "shr", 2),
        YulName::Sar => fun(FunctionName::Sar, "sar", 2),
        YulName::Eq => binop(BinaryOpType::Eq, "eq"),
        YulName::Or => binop(BinaryOpType::Or, "or"),
        YulName::Xor => binop(BinaryOpType::Xor, "xor"),

        YulName::Smod => fun(FunctionName::Smod, "smod", 2),
        YulName::Sdiv => fun(FunctionName::Sdiv, "sdiv", 2),
        YulName::Lt => fun(FunctionName::Lt, "lt", 2),
        YulName::Gt => fun(FunctionName::Gt, "gt", 2),
        YulName::IsZero => fun(FunctionName::IsZero, "iszero", 2),

        YulName::Slt => fun(FunctionName::Slt, "slt", 2),
        YulName::Sgt => fun(FunctionName::Sgt, "sgt", 2),

        YulName::Not => unop(UnaryOpType::Not, "not"),

        YulName::Byte => fun(FunctionName::Byte, "byte", 2),
        YulName::Pop => proc_simple(ProcName::Pop, "pop", 1, 0),
        YulName::AddMod => fun(FunctionName::AddMod, "addmod", 3),
        YulName::MulMod => fun(FunctionName::MulMod, "mulmod", 3),
        YulName::SignExtend => fun(FunctionName::SignExtend, "signextend", 2),
        YulName::Keccak256 => proc_simple(ProcName::Keccak256, "keccak256", 2, 1),
        YulName::MLoad => proc_mem(ProcName::MLoad, "mload", Usage::READ, 1, 1),
        YulName::MStore => proc_mem(ProcName::MStore, "mstore", Usage::WRITE, 2, 0),
        YulName::MStore8 => proc_mem(ProcName::MStore8, "mstore8", Usage::WRITE, 2, 0),
        YulName::MCopy => proc_mem(ProcName::MCopy, "mcopy", Usage::RW, 3, 1),
        YulName::SLoad => proc_storage(ProcName::SLoad, "sload", Usage::READ, 1, 1),
        YulName::SStore => proc_storage(ProcName::SStore, "sstore", Usage::WRITE, 2, 0),
        YulName::TLoad => proc_transient(ProcName::TLoad, "tload", Usage::READ, 1, 1),
        YulName::TStore => proc_transient(ProcName::TStore, "tstore", Usage::WRITE, 2, 0),
        YulName::LoadImmutable => proc_simple(ProcName::LoadImmutable, "loadimmutable", 1, 1),
        YulName::SetImmutable => proc_simple(ProcName::SetImmutable, "setimmutable", 3, 0),
        YulName::CallDataLoad => proc_simple(ProcName::CallDataLoad, "calldataload", 1, 1),
        YulName::CallDataSize => proc_simple(ProcName::CallDataSize, "calldatasize", 0, 1),
        YulName::CallDataCopy => proc_simple(ProcName::CallDataCopy, "calldatacopy", 3, 0),
        YulName::CodeSize => proc_simple(ProcName::CodeSize, "codesize", 0, 1),
        YulName::CodeCopy => proc_simple(ProcName::CodeCopy, "codecopy", 3, 0),
        YulName::ExtCodeSize => proc_simple(ProcName::ExtCodeSize, "extcodesize", 1, 1),
        YulName::ExtCodeHash => proc_simple(ProcName::ExtCodeHash, "extcodehash", 1, 1),
        YulName::ReturnDataSize => proc_simple(ProcName::ReturnDataSize, "returndatasize", 0, 1),
        YulName::ReturnDataCopy => proc_simple(ProcName::ReturnDataCopy, "returndatacopy", 3, 0),
        YulName::Log0 => proc_simple(ProcName::Log0, "log0", 2, 0),
        YulName::Log1 => proc_simple(ProcName::Log1, "log1", 3, 0),
        YulName::Log2 => proc_simple(ProcName::Log2, "log2", 4, 0),
        YulName::Log3 => proc_simple(ProcName::Log3, "log3", 5, 0),
        YulName::Log4 => proc_simple(ProcName::Log4, "log4", 6, 0),
        YulName::Call => proc_simple(ProcName::Call, "call", 7, 1),
        YulName::CallCode => proc_simple(ProcName::CallCode, "callcode", 7, 1),
        YulName::DelegateCall => proc_simple(ProcName::DelegateCall, "delegatecall", 6, 1),
        YulName::StaticCall => proc_mem(ProcName::StaticCall, "staticcall", Usage::WRITE, 6, 1),
        YulName::Create => proc_simple(ProcName::Create, "create", 3, 1),
        YulName::Create2 => proc_simple(ProcName::Create2, "create2", 4, 1),
        YulName::DataSize => proc_simple(ProcName::DataSize, "datasize", 1, 1),
        YulName::DataCopy => proc_simple(ProcName::DataCopy, "datacopy", 3, 0),
        YulName::DataOffset => proc_simple(ProcName::DataOffset, "dataoffset", 1, 1),
        YulName::LinkerSymbol => proc_simple(ProcName::LinkerSymbol, "linkersymbol", 1, 1),
        YulName::MemoryGuard => proc_simple(ProcName::MemoryGuard, "memoryguard", 1, 1),
        YulName::Address => proc_simple(ProcName::Address, "address", 0, 1),
        YulName::Caller => proc_simple(ProcName::Caller, "caller", 0, 1),
        YulName::CallValue => proc_simple(ProcName::CallValue, "callvalue", 0, 1),
        YulName::Gas => proc_simple(ProcName::Gas, "gas", 0, 1),
        YulName::Balance => proc_other(ProcName::Balance, "balance", Usage::READ, 1, 0),
        YulName::SelfBalance => proc_simple(ProcName::SelfBalance, "selfbalance", 0, 1),
        YulName::GasLimit => proc_simple(ProcName::GasLimit, "gaslimit", 0, 1),
        YulName::GasPrice => proc_simple(ProcName::GasPrice, "gasprice", 0, 1),
        YulName::Origin => proc_simple(ProcName::Origin, "origin", 0, 1),
        YulName::ChainId => proc_simple(ProcName::ChainId, "chainid", 0, 1),
        YulName::Number => proc_simple(ProcName::Number, "number", 0, 1),
        YulName::Timestamp => proc_simple(ProcName::Timestamp, "timestamp", 0, 1),
        YulName::BlockHash => proc_simple(ProcName::BlockHash, "blockhash", 1, 0),
        YulName::BlobHash => proc_simple(ProcName::BlobHash, "blobhash", 1, 0),
        YulName::Difficulty => proc_simple(ProcName::Difficulty, "difficulty", 0, 1),
        YulName::Prevrandao => proc_simple(ProcName::Prevrandao, "prevrandao", 0, 1),
        YulName::CoinBase => proc_simple(ProcName::CoinBase, "coinbase", 0, 1),
        YulName::MSize => proc_mem(ProcName::MSize, "msize", Usage::META, 0, 1),
        YulName::BaseFee => proc_other(ProcName::BaseFee, "basefee", Usage::READ, 0, 1),
        YulName::BlobBaseFee => proc_other(ProcName::BlobBaseFee, "blobbasefee", Usage::READ, 0, 1),
        YulName::Pc => proc_other(ProcName::Pc, "Pc", Usage::READ, 0, 1),
        YulName::ExtCodeCopy => proc_mem(ProcName::ExtCodeCopy, "extcodecopy", Usage::WRITE, 4, 0),
        YulName::SelfDestruct => {
            proc_other(ProcName::SelfDestruct, "selfdestruct", Usage::WRITE, 1, 0)
        }
        YulName::Verbatim {
            input_size,
            output_size,
        } => proc_other(
            ProcName::Verbatim {
                input_size: *input_size,
                output_size: *output_size,
            },
            format!("verbatim_i{}_o{}", input_size, output_size).as_str(),
            Usage::RW,
            *input_size,
            *output_size,
        ),
        YulName::ZkCreate
        | YulName::ZkCreate2
        | YulName::ZkToL1
        | YulName::ZkCodeSource
        | YulName::ZkPrecompile
        | YulName::ZkMeta
        | YulName::ZkSetContextU128
        | YulName::ZkSetPubdataPrice
        | YulName::ZkIncrementTxCounter
        | YulName::ZkEventInitialize
        | YulName::ZkEventWrite
        | YulName::ZkMimicCall
        | YulName::ZkSystemMimicCall
        | YulName::ZkMimicCallByRef
        | YulName::ZkSystemMimicCallByRef
        | YulName::ZkRawCall
        | YulName::ZkRawCallByRef
        | YulName::ZkSystemCall
        | YulName::ZkSystemCallByRef
        | YulName::ZkStaticRawCall
        | YulName::ZkStaticRawCallByRef
        | YulName::ZkStaticSystemCall
        | YulName::ZkStaticSystemCallByRef
        | YulName::ZkDelegateRawCall
        | YulName::ZkDelegateRawCallByRef
        | YulName::ZkDelegateSystemCall
        | YulName::ZkDelegateSystemCallByRef
        | YulName::ZkLoadCalldataIntoActivePtr
        | YulName::ZkLoadReturndataIntoActivePtr
        | YulName::ZkPtrAddIntoActive
        | YulName::ZkPtrShrinkIntoActive
        | YulName::ZkPtrPackIntoActive
        | YulName::ZkMultiplicationHigh
        | YulName::ZkGlobalLoad
        | YulName::ZkGlobalExtraAbiData
        | YulName::ZkGlobalStore => anyhow::bail!(
            "Transpilation of this YUL definition is not yet supported: {:?}",
            yul_name
        ),
        YulName::Return => Ok(DefinitionInfo {
            kind: Kind::Special(YulSpecial::Return),
            full_name: full_name("return"),
            r#type: arrow_type(2),
        }),
        YulName::Revert => Ok(DefinitionInfo {
            kind: Kind::Special(YulSpecial::Revert),
            full_name: full_name("revert"),
            r#type: arrow_type(2),
        }),
        YulName::Stop => Ok(DefinitionInfo {
            kind: Kind::Special(YulSpecial::Stop),
            full_name: full_name("stop"),
            r#type: arrow_type_proc(0, 0),
        }),
        YulName::Invalid => Ok(DefinitionInfo {
            kind: Kind::Special(YulSpecial::Invalid),
            full_name: full_name("invalid"),
            r#type: arrow_type_proc(0, 0),
        }),
        YulName::UserDefined(def) => {
            anyhow::bail!("Expecting standard definition, got {} instead.", def)
        }
    }
}

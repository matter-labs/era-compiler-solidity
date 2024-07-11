//!
//! [`DescriptionInfo`] instances for transpiled images of standard YUL definitions.
//!

use once_cell::sync::OnceCell;

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

static STANDARD_DEFINITIONS_COLLECTION: OnceCell<Vec<(YulName, DefinitionInfo)>> = OnceCell::new();
static TRANSPILER_DEFINITIONS_COLLECTION: OnceCell<Vec<DefinitionInfo>> = OnceCell::new();

pub fn transpiler_specific_definitions() -> &'static Vec<DefinitionInfo> {
    TRANSPILER_DEFINITIONS_COLLECTION.get_or_init(|| -> Vec<_> {
        vec![DefinitionInfo {
            kind: Kind::Function(FunctionName {
                name: String::from("of_int"),
                module: Some(String::from("W256")),
                yul_name: None,
            }),
            yul_name: full_name("of_int"),
            r#type: Type::Arrow(Box::from(Type::Integer), Box::from(Type::UInt(256))),
        }]
    })
}

/// Get a collection of definitions corresponding to the standard YUL functions.
pub fn standard_definitions() -> &'static Vec<(YulName, DefinitionInfo)> {
    STANDARD_DEFINITIONS_COLLECTION.get_or_init(|| -> Vec<_> {
        vec![
            (YulName::Add, binop(BinaryOpType::Add, "add")),
            (YulName::Sub, binop(BinaryOpType::Sub, "sub")),
            (YulName::Mul, binop(BinaryOpType::Mul, "mul")),
            (YulName::Div, binop(BinaryOpType::Div, "div")),
            (YulName::Mod, binop(BinaryOpType::Mod, "mod")),
            (YulName::Exp, binop(BinaryOpType::Exp, "exp")),
            (YulName::And, fun("bit_and", 2)),
            (YulName::Shl, fun("shl", 2)),
            (YulName::Shr, fun("shr", 2)),
            (YulName::Sar, fun("sar", 2)),
            (
                YulName::Eq,
                DefinitionInfo {
                    kind: Kind::Function(FunctionName {
                        name: "eq_uint256".to_string(),
                        module: Some(String::from("PurePrimops")),
                        yul_name: None,
                    }),
                    yul_name: FullName::new("eq_uint256".to_string(), Path::empty()),
                    r#type: Type::Arrow(
                        Box::from(Type::type_of_vec(&[Type::UInt(256), Type::UInt(256)])),
                        Box::from(Type::UInt(256)),
                    ),
                },
            ),
            (YulName::Or, fun("or", 2)),
            (YulName::Xor, fun("xor", 2)),
            (YulName::Smod, fun("smod", 2)),
            (YulName::Sdiv, fun("sdiv", 2)),
            (YulName::Lt, fun("lt_uint256", 2)),
            (YulName::Gt, fun("gt_uint256", 2)),
            (YulName::IsZero, fun("iszero", 2)),
            (YulName::Slt, fun("slt_uint256", 2)),
            (YulName::Sgt, fun("sgt_uint256", 2)),
            (YulName::Not, unop(UnaryOpType::Not, "not")),
            (YulName::Byte, fun("byte", 2)),
            (YulName::Pop, proc_simple("pop", 1, 0)),
            (YulName::AddMod, fun("addmod", 3)),
            (YulName::MulMod, fun("mulmod", 3)),
            (YulName::SignExtend, fun("signextend", 2)),
            (YulName::Keccak256, proc_mem("keccak256", Usage::READ, 2, 1)),
            (YulName::MLoad, proc_mem("mload", Usage::READ, 1, 1)),
            (YulName::MStore, proc_mem("mstore", Usage::WRITE, 2, 0)),
            (YulName::MStore8, proc_mem("mstore8", Usage::WRITE, 2, 0)),
            (YulName::MCopy, proc_mem("mcopy", Usage::RW, 3, 1)),
            (YulName::SLoad, proc_storage("sload", Usage::READ, 1, 1)),
            (YulName::SStore, proc_storage("sstore", Usage::WRITE, 2, 0)),
            (YulName::TLoad, proc_transient("tload", Usage::READ, 1, 1)),
            (
                YulName::TStore,
                proc_transient("tstore", Usage::WRITE, 2, 0),
            ),
            (YulName::LoadImmutable, proc_simple("loadimmutable", 1, 1)),
            (YulName::SetImmutable, proc_simple("setimmutable", 3, 0)),
            (YulName::CallDataLoad, proc_simple("calldataload", 1, 1)),
            (YulName::CallDataSize, proc_simple("calldatasize", 0, 1)),
            (YulName::CallDataCopy, proc_simple("calldatacopy", 3, 0)),
            (YulName::CodeSize, proc_simple("codesize", 0, 1)),
            (YulName::CodeCopy, proc_simple("codecopy", 3, 0)),
            (YulName::ExtCodeSize, proc_simple("extcodesize", 1, 1)),
            (YulName::ExtCodeHash, proc_simple("extcodehash", 1, 1)),
            (YulName::ReturnDataSize, proc_simple("returndatasize", 0, 1)),
            (YulName::ReturnDataCopy, proc_simple("returndatacopy", 3, 0)),
            (YulName::Log0, proc_simple("log0", 2, 0)),
            (YulName::Log1, proc_simple("log1", 3, 0)),
            (YulName::Log2, proc_simple("log2", 4, 0)),
            (YulName::Log3, proc_simple("log3", 5, 0)),
            (YulName::Log4, proc_simple("log4", 6, 0)),
            (YulName::Call, proc_simple("call", 7, 1)),
            (YulName::CallCode, proc_simple("callcode", 7, 1)),
            (YulName::DelegateCall, proc_simple("delegatecall", 6, 1)),
            (
                YulName::StaticCall,
                proc_mem("staticcall", Usage::WRITE, 6, 1),
            ),
            (YulName::Create, proc_simple("create", 3, 1)),
            (YulName::Create2, proc_simple("create2", 4, 1)),
            (YulName::DataSize, proc_simple("datasize", 1, 1)),
            (YulName::DataCopy, proc_simple("datacopy", 3, 0)),
            (YulName::DataOffset, proc_simple("dataoffset", 1, 1)),
            (YulName::LinkerSymbol, proc_simple("linkersymbol", 1, 1)),
            (YulName::MemoryGuard, proc_simple("memoryguard", 1, 1)),
            (YulName::Address, proc_simple("address", 0, 1)),
            (YulName::Caller, proc_simple("caller", 0, 1)),
            (YulName::CallValue, proc_simple("callvalue", 0, 1)),
            (YulName::Gas, proc_simple("gas", 0, 1)),
            (YulName::Balance, proc_other("balance", Usage::READ, 1, 0)),
            (YulName::SelfBalance, proc_simple("selfbalance", 0, 1)),
            (YulName::GasLimit, proc_simple("gaslimit", 0, 1)),
            (YulName::GasPrice, proc_simple("gasprice", 0, 1)),
            (YulName::Origin, proc_simple("origin", 0, 1)),
            (YulName::ChainId, proc_simple("chainid", 0, 1)),
            (YulName::Number, proc_simple("number", 0, 1)),
            (YulName::Timestamp, proc_simple("timestamp", 0, 1)),
            (YulName::BlockHash, proc_simple("blockhash", 1, 0)),
            (YulName::BlobHash, proc_simple("blobhash", 1, 0)),
            (YulName::Difficulty, proc_simple("difficulty", 0, 1)),
            (YulName::Prevrandao, proc_simple("prevrandao", 0, 1)),
            (YulName::CoinBase, proc_simple("coinbase", 0, 1)),
            (YulName::MSize, proc_mem("msize", Usage::META, 0, 1)),
            (YulName::BaseFee, proc_other("basefee", Usage::READ, 0, 1)),
            (
                YulName::BlobBaseFee,
                proc_other("blobbasefee", Usage::READ, 0, 1),
            ),
            (YulName::Pc, proc_other("Pc", Usage::READ, 0, 1)),
            (
                YulName::ExtCodeCopy,
                proc_mem("extcodecopy", Usage::WRITE, 4, 0),
            ),
            (
                YulName::SelfDestruct,
                proc_other("selfdestruct", Usage::WRITE, 1, 0),
            ),
            (
                YulName::Return,
                (DefinitionInfo {
                    kind: Kind::Special(YulSpecial::Return),
                    yul_name: full_name("return"),
                    r#type: arrow_type(2),
                }),
            ),
            (
                YulName::Revert,
                (DefinitionInfo {
                    kind: Kind::Special(YulSpecial::Revert),
                    yul_name: full_name("revert"),
                    r#type: arrow_type(2),
                }),
            ),
            (
                YulName::Stop,
                (DefinitionInfo {
                    kind: Kind::Special(YulSpecial::Stop),
                    yul_name: full_name("stop"),
                    r#type: arrow_type_proc(0, 0),
                }),
            ),
            (
                YulName::Invalid,
                (DefinitionInfo {
                    kind: Kind::Special(YulSpecial::Invalid),
                    yul_name: full_name("invalid"),
                    r#type: arrow_type_proc(0, 0),
                }),
            ),
        ]
        .iter()
        .chain(generate_verbatim_definitions().iter())
        .cloned()
        .collect()
    })
}

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
fn unop(typ: UnaryOpType, name: &str) -> DefinitionInfo {
    DefinitionInfo {
        kind: Kind::UnOp(typ),
        yul_name: full_name(name),
        r#type: arrow_type(1),
    }
}
fn binop(typ: BinaryOpType, name: &str) -> DefinitionInfo {
    DefinitionInfo {
        kind: Kind::BinOp(typ),
        yul_name: full_name(name),
        r#type: arrow_type(2),
    }
}

fn fun(name_str: &str, input_args: usize) -> DefinitionInfo {
    DefinitionInfo {
        kind: Kind::Function(primops_fun(name_str)),
        yul_name: full_name(name_str),
        r#type: arrow_type(input_args),
    }
}

fn proc(
    name_str: &str,
    input_args: usize,
    output_args: usize,
    attributes: Attributes,
) -> DefinitionInfo {
    DefinitionInfo {
        kind: Kind::Proc(ProcKind {
            name: primops_proc(name_str),
            attributes,
        }),
        yul_name: full_name(name_str),
        r#type: arrow_type_proc(input_args, output_args),
    }
}
fn proc_simple(name_str: &str, input_args: usize, output_args: usize) -> DefinitionInfo {
    let attributes = Attributes::default();
    proc(name_str, input_args, output_args, attributes)
}
fn proc_mem(name_str: &str, usage: Usage, input_args: usize, output_args: usize) -> DefinitionInfo {
    // Currently the memory parameter propagation is disabled.
    //let attributes = Attributes::heap(usage);
    let _ = usage;
    let attributes = Attributes::default();
    proc(name_str, input_args, output_args, attributes)
}
fn proc_storage(
    name_str: &str,
    usage: Usage,
    input_args: usize,
    output_args: usize,
) -> DefinitionInfo {
    let attributes = Attributes::storage(usage);
    proc(name_str, input_args, output_args, attributes)
}

fn proc_transient(
    name_str: &str,
    usage: Usage,
    input_args: usize,
    output_args: usize,
) -> DefinitionInfo {
    let attributes = Attributes::transient(usage);
    proc(name_str, input_args, output_args, attributes)
}
fn proc_other(
    name_str: &str,
    usage: Usage,
    input_args: usize,
    output_args: usize,
) -> DefinitionInfo {
    let attributes = Attributes::other(usage);
    proc(name_str, input_args, output_args, attributes)
}

fn top_proc(name: &str) -> ProcName {
    ProcName {
        name: name.to_string(),
        module: None,
        yul_name: None,
    }
}
fn top_fun(name: &str) -> FunctionName {
    FunctionName {
        name: name.to_string(),
        module: None,
        yul_name: None,
    }
}
fn primops_proc(name: &str) -> ProcName {
    ProcName {
        name: name.to_string(),
        module: Some("Primops".to_string()),
        yul_name: None,
    }
}
fn primops_fun(name: &str) -> FunctionName {
    FunctionName {
        name: name.to_string(),
        module: Some("PurePrimops".to_string()),
        yul_name: None,
    }
}

fn generate_verbatim_definitions() -> Vec<(YulName, DefinitionInfo)> {
    (0..3)
        .zip(0..3)
        .map(|(input_size, output_size)| {
            (
                YulName::Verbatim {
                    input_size,
                    output_size,
                },
                proc_other(
                    format!("verbatim_i{}_o{}", input_size, output_size).as_str(),
                    Usage::RW,
                    input_size,
                    output_size,
                ),
            )
        })
        .collect()
}

// Not implemented:
// - YulName::ZkCreate
// - YulName::ZkCreate2
// - YulName::ZkToL1
// - YulName::ZkCodeSource
// - YulName::ZkPrecompile
// - YulName::ZkMeta
// - YulName::ZkSetContextU128
// - YulName::ZkSetPubdataPrice
// - YulName::ZkIncrementTxCounter
// - YulName::ZkEventInitialize
// - YulName::ZkEventWrite
// - YulName::ZkMimicCall
// - YulName::ZkSystemMimicCall
// - YulName::ZkMimicCallByRef
// - YulName::ZkSystemMimicCallByRef
// - YulName::ZkRawCall
// - YulName::ZkRawCallByRef
// - YulName::ZkSystemCall
// - YulName::ZkSystemCallByRef
// - YulName::ZkStaticRawCall
// - YulName::ZkStaticRawCallByRef
// - YulName::ZkStaticSystemCall
// - YulName::ZkStaticSystemCallByRef
// - YulName::ZkDelegateRawCall
// - YulName::ZkDelegateRawCallByRef
// - YulName::ZkDelegateSystemCall
// - YulName::ZkDelegateSystemCallByRef
// - YulName::ZkLoadCalldataIntoActivePtr
// - YulName::ZkLoadReturndataIntoActivePtr
// - YulName::ZkPtrAddIntoActive
// - YulName::ZkPtrShrinkIntoActive
// - YulName::ZkPtrPackIntoActive
// - YulName::ZkMultiplicationHigh
// - YulName::ZkGlobalLoad
// - YulName::ZkGlobalExtraAbiData
// - YulName::ZkGlobalStore

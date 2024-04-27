//!
//! EasyCrypt AST
//!

use std::{
    collections::HashMap,
    fmt::Display,
    hash::{Hash, Hasher},
};

use super::from_yul::location::Location;

pub type Name = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcName {
    /// The user-defined procedure.
    UserDefined(Name),
    /// discard value x
    Pop,
    /// `keccak(mem[p…(p+n)))`
    Keccak256,

    /// `mem[p…(p+32))`.
    MLoad,
    /// `mem[p…(p+32)) := v`.
    MStore,
    /// `mem[p] := v & 0xff` (only modifies a single byte).
    MStore8,
    /// heap memory copy
    MCopy,

    /// `storage[p]`.
    SLoad,
    /// `storage[p] := v`.
    SStore,
    /// transient `storage[p]`.
    TLoad,
    /// transient `storage[p] := v`.
    TStore,
    /// `loadimmutable` storage read.
    LoadImmutable,
    /// `setimmutable` storage write.
    SetImmutable,

    /// call data starting from position `p` (32 bytes).
    CallDataLoad,
    /// size of call data in bytes.
    CallDataSize,
    /// copy `s` bytes from calldata at position `f` to memory at position `t`.
    CallDataCopy,
    /// size of the code of the current contract / execution context.
    CodeSize,
    /// copy `s` bytes from code at position `f` to mem at position `t`.
    CodeCopy,
    /// size of the code at address `a`.
    ExtCodeSize,
    /// code hash of address `a`.
    ExtCodeHash,
    /// size of the last returndata.
    ReturnDataSize,
    /// copy `s` bytes from returndata at position `f` to mem at position `t`.
    ReturnDataCopy,
    /// end execution, return data `mem[p…(p+s))`.
    //Return,
    /// end execution, revert state changes, return data `mem[p…(p+s))`.
    //Revert,
    /// stop execution, identical to `return(0, 0)`.
    //Stop,
    /// end execution with invalid instruction.
    //Invalid,

    /// log without topics and data `mem[p…(p+s))`
    Log0,
    /// log with topic t1 and data `mem[p…(p+s))`
    Log1,
    /// log with topics t1, t2 and data `mem[p…(p+s))`
    Log2,
    /// log with topics t1, t2, t3 and data `mem[p…(p+s))`
    Log3,
    /// log with topics t1, t2, t3, t4 and data `mem[p…(p+s))`
    Log4,

    /// call contract at address a with input `mem[in…(in+insize))` providing `g` gas and `v` wei
    /// and output area `mem[out…(out+outsize))` returning 0 on error (e.g. out of gas)
    /// and 1 on success
    /// [See more](https://docs.soliditylang.org/en/v0.8.2/yul.html#yul-call-return-area)
    Call,
    /// identical to call but only use the code from a and stay in the context of the current
    /// contract otherwise
    CallCode,
    /// identical to `callcode` but also keeps `caller` and `callvalue`
    DelegateCall,
    /// identical to `call(g, a, 0, in, insize, out, outsize)` but do not allows state modifications
    StaticCall,

    /// create new contract with code `mem[p…(p+n))` and send `v` wei and return the new address
    ///
    /// Passes bytecode to the system contracts.
    Create,
    /// create new contract with code `mem[p…(p+n))` at address
    /// `keccak256(0xff . this . s . keccak256(mem[p…(p+n)))` and send `v` wei and return the
    /// new address, where `0xff` is a 1-byte value, this is the current contract’s address as a
    /// 20-byte value and `s` is a big-endian 256-bit value
    ///
    /// Passes bytecode to the system contracts.
    Create2,
    /// create new contract with code `mem[p…(p+n))` and send `v` wei and return the new address
    ///
    /// Passes hash to the system contracts.
    ZkCreate,
    /// create new contract with code `mem[p…(p+n))` at address
    /// `keccak256(0xff . this . s . keccak256(mem[p…(p+n)))` and send `v` wei and return the
    /// new address, where `0xff` is a 1-byte value, this is the current contract’s address as a
    /// 20-byte value and `s` is a big-endian 256-bit value
    ///
    /// Passes hash to the system contracts.
    ZkCreate2,
    /// returns the size in the data area
    DataSize,
    /// is equivalent to `CodeCopy`
    DataCopy,
    /// returns the offset in the data area
    DataOffset,

    /// `linkersymbol` is a stub call
    LinkerSymbol,
    /// `memoryguard` is a stub call
    MemoryGuard,

    /// address of the current contract / execution context
    Address,
    /// call sender (excluding `delegatecall`)
    Caller,

    /// wei sent together with the current call
    CallValue,
    /// gas still available to execution
    Gas,
    /// wei balance at address `a`
    Balance,
    /// equivalent to `balance(address())`, but cheaper
    SelfBalance,

    /// block gas limit of the current block
    GasLimit,
    /// gas price of the transaction
    GasPrice,
    /// transaction sender
    Origin,
    /// ID of the executing chain (EIP 1344)
    ChainId,
    /// current block number
    Number,
    /// timestamp of the current block in seconds since the epoch
    Timestamp,
    /// hash of block nr b - only for last 256 blocks excluding current
    BlockHash,
    /// versioned hash of transaction’s i-th blob
    BlobHash,
    /// difficulty of the current block
    Difficulty,
    /// https://eips.ethereum.org/EIPS/eip-4399
    Prevrandao,
    /// current mining beneficiary
    CoinBase,
    /// size of memory, i.e. largest accessed memory index
    MSize,

    /// verbatim instruction with 0 inputs and 0 outputs
    /// only works in the Yul mode, so it is mostly used as a tool for extending Yul for zkSync
    Verbatim {
        /// the number of input arguments
        input_size: usize,
        /// the number of output arguments
        output_size: usize,
    },

    /// current block’s base fee (EIP-3198 and EIP-1559)
    BaseFee,
    /// current block’s blob base fee (EIP-7516 and EIP-4844)
    BlobBaseFee,
    /// current position in code
    Pc,
    /// like `codecopy(t, f, s)` but take code at address `a`
    ExtCodeCopy,
    /// end execution, destroy current contract and send funds to `a`
    SelfDestruct,

    /// The eponymous EraVM Yul extension instruction.
    ZkToL1,
    /// The eponymous EraVM Yul extension instruction.
    ZkCodeSource,
    /// The eponymous EraVM Yul extension instruction.
    ZkPrecompile,
    /// The eponymous EraVM Yul extension instruction.
    ZkMeta,
    /// The eponymous EraVM Yul extension instruction.
    ZkSetContextU128,
    /// The eponymous EraVM Yul extension instruction.
    ZkSetPubdataPrice,
    /// The eponymous EraVM Yul extension instruction.
    ZkIncrementTxCounter,
    /// The eponymous EraVM Yul extension instruction.
    ZkEventInitialize,
    /// The eponymous EraVM Yul extension instruction.
    ZkEventWrite,

    /// The eponymous EraVM Yul extension instruction.
    ZkMimicCall,
    /// The eponymous EraVM Yul extension instruction.
    ZkSystemMimicCall,
    /// The eponymous EraVM Yul extension instruction.
    ZkMimicCallByRef,
    /// The eponymous EraVM Yul extension instruction.
    ZkSystemMimicCallByRef,
    /// The eponymous EraVM Yul extension instruction.
    ZkRawCall,
    /// The eponymous EraVM Yul extension instruction.
    ZkRawCallByRef,
    /// The eponymous EraVM Yul extension instruction.
    ZkSystemCall,
    /// The eponymous EraVM Yul extension instruction.
    ZkSystemCallByRef,
    /// The eponymous EraVM Yul extension instruction.
    ZkStaticRawCall,
    /// The eponymous EraVM Yul extension instruction.
    ZkStaticRawCallByRef,
    /// The eponymous EraVM Yul extension instruction.
    ZkStaticSystemCall,
    /// The eponymous EraVM Yul extension instruction.
    ZkStaticSystemCallByRef,
    /// The eponymous EraVM Yul extension instruction.
    ZkDelegateRawCall,
    /// The eponymous EraVM Yul extension instruction.
    ZkDelegateRawCallByRef,
    /// The eponymous EraVM Yul extension instruction.
    ZkDelegateSystemCall,
    /// The eponymous EraVM Yul extension instruction.
    ZkDelegateSystemCallByRef,

    /// The eponymous EraVM Yul extension instruction.
    ZkLoadCalldataIntoActivePtr,
    /// The eponymous EraVM Yul extension instruction.
    ZkLoadReturndataIntoActivePtr,
    /// The eponymous EraVM Yul extension instruction.
    ZkPtrAddIntoActive,
    /// The eponymous EraVM Yul extension instruction.
    ZkPtrShrinkIntoActive,
    /// The eponymous EraVM Yul extension instruction.
    ZkPtrPackIntoActive,

    /// The eponymous EraVM Yul extension instruction.
    ZkMultiplicationHigh,

    /// The eponymous EraVM Yul extension instruction.
    ZkGlobalLoad,
    /// The eponymous EraVM Yul extension instruction.
    ZkGlobalExtraAbiData,
    /// The eponymous EraVM Yul extension instruction.
    ZkGlobalStore,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionName {
    /// The user-defined procedure or function.
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
            f.write_str(&s)
        } else {
            let str = match self {
                FunctionName::UserDefined(s) => unreachable!(),
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
            };
            f.write_str(str)
        }
    }
}
impl Display for ProcName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let ProcName::Verbatim {
            input_size,
            output_size,
        } = self
        {
            f.write_fmt(format_args!("verbatim_i{}_o{}", input_size, output_size))
        } else {
            let str: &str = match self {
                ProcName::UserDefined(s) => s,
                ProcName::Pop => "Pop",
                ProcName::Keccak256 => "Keccak256",
                ProcName::MLoad => "MLoad",
                ProcName::MStore => "MStore",
                ProcName::MStore8 => "MStore8",
                ProcName::MCopy => "MCopy",
                ProcName::SLoad => "SLoad",
                ProcName::SStore => "SStore",
                ProcName::TLoad => "TLoad",
                ProcName::TStore => "TStore",
                ProcName::LoadImmutable => "LoadImmutable",
                ProcName::SetImmutable => "SetImmutable",
                ProcName::CallDataLoad => "CallDataLoad",
                ProcName::CallDataSize => "CallDataSize",
                ProcName::CallDataCopy => "CallDataCopy",
                ProcName::CodeSize => "CodeSize",
                ProcName::CodeCopy => "CodeCopy",
                ProcName::ExtCodeSize => "ExtCodeSize",
                ProcName::ExtCodeHash => "ExtCodeHash",
                ProcName::ReturnDataSize => "ReturnDataSize",
                ProcName::ReturnDataCopy => "ReturnDataCopy",
                ProcName::Log0 => "Log0",
                ProcName::Log1 => "Log1",
                ProcName::Log2 => "Log2",
                ProcName::Log3 => "Log3",
                ProcName::Log4 => "Log4",
                ProcName::Call => "Call",
                ProcName::CallCode => "CallCode",
                ProcName::DelegateCall => "DelegateCall",
                ProcName::StaticCall => "StaticCall",
                ProcName::Create => "Create",
                ProcName::Create2 => "Create2",
                ProcName::ZkCreate => "ZkCreate",
                ProcName::ZkCreate2 => "ZkCreate2",
                ProcName::DataSize => "DataSize",
                ProcName::DataCopy => "DataCopy",
                ProcName::DataOffset => "DataOffset",
                ProcName::LinkerSymbol => "LinkerSymbol",
                ProcName::MemoryGuard => "MemoryGuard",
                ProcName::Address => "Address",
                ProcName::Caller => "Caller",
                ProcName::CallValue => "CallValue",
                ProcName::Gas => "Gas",
                ProcName::Balance => "Balance",
                ProcName::SelfBalance => "SelfBalance",
                ProcName::GasLimit => "GasLimit",
                ProcName::GasPrice => "GasPrice",
                ProcName::Origin => "Origin",
                ProcName::ChainId => "ChainId",
                ProcName::Number => "Number",
                ProcName::Timestamp => "Timestamp",
                ProcName::BlockHash => "BlockHash",
                ProcName::BlobHash => "BlobHash",
                ProcName::Difficulty => "Difficulty",
                ProcName::Prevrandao => "Prevrandao",
                ProcName::CoinBase => "CoinBase",
                ProcName::MSize => "MSize",
                ProcName::Verbatim {
                    input_size,
                    output_size,
                } => unreachable!(),
                ProcName::BaseFee => "BaseFee",
                ProcName::BlobBaseFee => "BlobBaseFee",
                ProcName::Pc => "Pc",
                ProcName::ExtCodeCopy => "ExtCodeCopy",
                ProcName::SelfDestruct => "SelfDestruct",
                ProcName::ZkToL1 => "ZkToL1",
                ProcName::ZkCodeSource => "ZkCodeSource",
                ProcName::ZkPrecompile => "ZkPrecompile",
                ProcName::ZkMeta => "ZkMeta",
                ProcName::ZkSetContextU128 => "ZkSetContextU128",
                ProcName::ZkSetPubdataPrice => "ZkSetPubdataPrice",
                ProcName::ZkIncrementTxCounter => "ZkIncrementTxCounter",
                ProcName::ZkEventInitialize => "ZkEventInitialize",
                ProcName::ZkEventWrite => "ZkEventWrite",
                ProcName::ZkMimicCall => "ZkMimicCall",
                ProcName::ZkSystemMimicCall => "ZkSystemMimicCall",
                ProcName::ZkMimicCallByRef => "ZkMimicCallByRef",
                ProcName::ZkSystemMimicCallByRef => "ZkSystemMimicCallByRef",
                ProcName::ZkRawCall => "ZkRawCall",
                ProcName::ZkRawCallByRef => "ZkRawCallByRef",
                ProcName::ZkSystemCall => "ZkSystemCall",
                ProcName::ZkSystemCallByRef => "ZkSystemCallByRef",
                ProcName::ZkStaticRawCall => "ZkStaticRawCall",
                ProcName::ZkStaticRawCallByRef => "ZkStaticRawCallByRef",
                ProcName::ZkStaticSystemCall => "ZkStaticSystemCall",
                ProcName::ZkStaticSystemCallByRef => "ZkStaticSystemCallByRef",
                ProcName::ZkDelegateRawCall => "ZkDelegateRawCall",
                ProcName::ZkDelegateRawCallByRef => "ZkDelegateRawCallByRef",
                ProcName::ZkDelegateSystemCall => "ZkDelegateSystemCall",
                ProcName::ZkDelegateSystemCallByRef => "ZkDelegateSystemCallByRef",
                ProcName::ZkLoadCalldataIntoActivePtr => "ZkLoadCalldataIntoActivePtr",
                ProcName::ZkLoadReturndataIntoActivePtr => "ZkLoadReturndataIntoActivePtr",
                ProcName::ZkPtrAddIntoActive => "ZkPtrAddIntoActive",
                ProcName::ZkPtrShrinkIntoActive => "ZkPtrShrinkIntoActive",
                ProcName::ZkPtrPackIntoActive => "ZkPtrPackIntoActive",
                ProcName::ZkMultiplicationHigh => "ZkMultiplicationHigh",
                ProcName::ZkGlobalLoad => "ZkGlobalLoad",
                ProcName::ZkGlobalExtraAbiData => "ZkGlobalExtraAbiData",
                ProcName::ZkGlobalStore => "ZkGlobalStore",
            };
            f.write_str(str)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntegerLiteral {
    Decimal { inner: String },
    Hexadecimal { inner: String },
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal {
    String(String),
    Int(IntegerLiteral),
    Bool(bool),
}
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
                f.write_str("(");
                for (i, component) in inner.iter().enumerate() {
                    component.fmt(f);
                    if i > 0 {
                        f.write_str(" * ");
                    }
                }
                f.write_str(")")
            }
            Type::Unknown => f.write_str("Unknown"),
        };
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryOpType {
    /// `x + y`.
    Add,
    /// `x - y`.
    Sub,
    /// `x * y`.
    Mul,
    /// `x / y` or `0` if `y == 0`.
    Div,
    /// `x % y` or `0` if `y == 0`.
    Mod,

    /// `1` if `x == y`, `0` otherwise.
    Eq,

    /// bitwise "or" of `x` and `y`.
    Or,
    /// bitwise "xor" of `x` and `y`.
    Xor,
    /// bitwise "and" of `x` and `y`.
    And,
    /// logical shift left `y` by `x` bits
    Shl,
    /// logical shift right `y` by `x` bits
    Shr,
    /// `x` to the power of `y`
    Exp,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnaryOpType {
    Neg,
    /// bitwise "not" of `x` (every bit of `x` is negated).
    Not,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Definition {
    pub identifier: Name,
    pub location: Location,
    pub r#type: Option<Type>,
}

impl Definition {
    pub fn reference(&self) -> Reference {
        Reference {
            identifier: self.identifier.clone(),
            location: self.location.clone(),
        }
    }
}

impl Hash for Reference {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.identifier.hash(state);
        self.location.hash(state);
    }
}

impl Hash for Definition {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.identifier.hash(state);
        self.location.hash(state);
        self.r#type.hash(state);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reference {
    pub identifier: Name,
    pub location: Location,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Unary(UnaryOpType, Box<Self>),
    Binary(BinaryOpType, Box<Expression>, Box<Expression>),
    ECall(FunctionCall),
    Literal(Literal),
    Reference(Reference),
    Tuple(Vec<Expression>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignatureKind {
    Function,
    Proc,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signature {
    pub formal_parameters: Vec<(Definition, Type)>,
    pub return_type: Type,
    pub kind: SignatureKind,
}

impl Signature {
    pub const UNIT_TO_UNIT: Signature = Signature {
        formal_parameters: vec![],
        return_type: Type::Unit,
        kind: SignatureKind::Proc,
    };
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    pub name: FunctionName,
    pub location: Location,
    pub signature: Signature,
    pub body: Expression,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Proc {
    pub name: ProcName,
    pub location: Location,
    pub signature: Signature,
    pub locals: Vec<Definition>,
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcCall {
    pub target: ProcName,
    pub arguments: Vec<Expression>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionCall {
    pub target: FunctionName,
    pub arguments: Vec<Expression>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pub statements: Vec<Statement>,
}

impl Block {
    pub fn empty() -> Self {
        Self { statements: vec![] }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    VarDefinition(Definition, Expression),
    Expression(Expression),
    Block(Block),
    If(Expression, Box<Statement>, Box<Statement>),
    EAssignment(Vec<Reference>, Box<Expression>), // x <- expr
    PAssignment(Vec<Reference>, ProcCall),        // x <@ proc
    Return(Expression),
    While(Expression, Box<Statement>),
    // SAssignment for // x <$ distr
    Pass,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleDefinition {
    ProcDef(Proc),
    FunDef(Function),
}

impl ModuleDefinition {
    pub fn reference(&self) -> Reference {
        match self {
            ModuleDefinition::ProcDef(proc) => Reference {
                identifier: proc.name.to_string(),
                location: proc.location.clone(),
            },
            ModuleDefinition::FunDef(fun) => Reference {
                identifier: fun.name.to_string(),
                location: fun.location.clone(),
            },
        }
    }
    pub fn is_proc(&self) -> bool {
        if let ModuleDefinition::ProcDef(_) = self {
            true
        } else {
            false
        }
    }
    pub fn is_fun(&self) -> bool {
        if let ModuleDefinition::FunDef(_) = self {
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    pub name: Option<Name>,
    pub definitions: HashMap<Reference, ModuleDefinition>,
}

impl Module {
    pub fn new(name: Option<Name>) -> Self {
        Self {
            definitions: HashMap::new(),
            name,
        }
    }
    pub fn merge(&mut self, other: &Self) {
        if other.name.is_none() {
            self.definitions.extend(other.definitions.clone())
        } else {
            panic!("Trying to merge named modules")
        }
    }
    pub fn add_def(&mut self, module_def: ModuleDefinition) {
        self.definitions.insert(module_def.reference(), module_def);
    }
}

impl Expression {
    pub fn pack_tuple(exprs: Vec<Expression>) -> Expression {
        match exprs.len() {
            0 => panic!("Attempt to pack in a tuple 0 expressions."),
            1 => exprs[0].clone(),
            _ => Expression::Tuple(exprs),
        }
    }
}

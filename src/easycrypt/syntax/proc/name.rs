use std::fmt::Display;

use crate::easycrypt::syntax::Name;

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
                    input_size: _,
                    output_size: _,
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

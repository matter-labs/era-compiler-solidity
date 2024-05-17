//!
//! Solidity to EraVM compiler constants.
//!

/// The default executable name.
pub static DEFAULT_EXECUTABLE_NAME: &str = "zksolc";

/// The rayon worker stack size.
pub const RAYON_WORKER_STACK_SIZE: usize = 16 * 1024 * 1024;

/// The `keccak256` scratch space offset.
pub const OFFSET_SCRATCH_SPACE: usize = 0;

/// The memory pointer offset.
pub const OFFSET_MEMORY_POINTER: usize = 2 * era_compiler_common::BYTE_LENGTH_FIELD;

/// The empty slot offset.
pub const OFFSET_EMPTY_SLOT: usize = 3 * era_compiler_common::BYTE_LENGTH_FIELD;

/// The non-reserved memory offset.
pub const OFFSET_NON_RESERVED: usize = 4 * era_compiler_common::BYTE_LENGTH_FIELD;

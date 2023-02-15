//!
//! The Ethereal IR block visited element.
//!

///
/// The Ethereal IR block visited element.
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VisitedElement {
    /// The block key.
    pub block_key: compiler_llvm_context::FunctionBlockKey,
    /// The initial stack state hash.
    pub stack_hash: md5::Digest,
}

impl VisitedElement {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        block_key: compiler_llvm_context::FunctionBlockKey,
        stack_hash: md5::Digest,
    ) -> Self {
        Self {
            block_key,
            stack_hash,
        }
    }
}

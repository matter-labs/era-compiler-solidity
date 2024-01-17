//!
//! The Ethereal IR block visited element.
//!

use std::cmp::Ordering;

///
/// The Ethereal IR block visited element.
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VisitedElement {
    /// The block key.
    pub block_key: compiler_llvm_context::EraVMFunctionBlockKey,
    /// The initial stack state hash.
    pub stack_hash: md5::Digest,
}

impl VisitedElement {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        block_key: compiler_llvm_context::EraVMFunctionBlockKey,
        stack_hash: md5::Digest,
    ) -> Self {
        Self {
            block_key,
            stack_hash,
        }
    }
}

impl PartialOrd for VisitedElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for VisitedElement {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.block_key.code_type, other.block_key.code_type) {
            (compiler_llvm_context::CodeType::Deploy, compiler_llvm_context::CodeType::Runtime) => {
                Ordering::Less
            }
            (compiler_llvm_context::CodeType::Runtime, compiler_llvm_context::CodeType::Deploy) => {
                Ordering::Greater
            }
            (compiler_llvm_context::CodeType::Deploy, compiler_llvm_context::CodeType::Deploy)
            | (
                compiler_llvm_context::CodeType::Runtime,
                compiler_llvm_context::CodeType::Runtime,
            ) => {
                let tag_comparison = self.block_key.tag.cmp(&other.block_key.tag);
                if tag_comparison == Ordering::Equal {
                    if self.stack_hash == other.stack_hash {
                        Ordering::Equal
                    } else {
                        Ordering::Less
                    }
                } else {
                    tag_comparison
                }
            }
        }
    }
}

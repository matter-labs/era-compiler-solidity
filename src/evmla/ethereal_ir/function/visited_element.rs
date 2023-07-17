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

impl PartialOrd for VisitedElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.block_key.code_type, other.block_key.code_type) {
            (compiler_llvm_context::CodeType::Deploy, compiler_llvm_context::CodeType::Runtime) => {
                Some(Ordering::Less)
            }
            (compiler_llvm_context::CodeType::Runtime, compiler_llvm_context::CodeType::Deploy) => {
                Some(Ordering::Greater)
            }
            (compiler_llvm_context::CodeType::Deploy, compiler_llvm_context::CodeType::Deploy)
            | (
                compiler_llvm_context::CodeType::Runtime,
                compiler_llvm_context::CodeType::Runtime,
            ) => {
                let tag_comparison = self.block_key.tag.cmp(&other.block_key.tag);
                if tag_comparison == Ordering::Equal {
                    if self.stack_hash == other.stack_hash {
                        Some(Ordering::Equal)
                    } else {
                        Some(Ordering::Less)
                    }
                } else {
                    Some(tag_comparison)
                }
            }
        }
    }
}

impl Ord for VisitedElement {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).expect("Always exists")
    }
}

use std::collections::BTreeSet;

use crate::yul::error::Error;
use crate::yul::parser::error::Error as ParserError;
use crate::yul::parser::identifier::Identifier;

/// The LLVM attribute section prefix.
pub const LLVM_ATTRIBUTE_PREFIX: &str = "$llvm_";

/// The LLVM attribute section suffix.
pub const LLVM_ATTRIBUTE_SUFFIX: &str = "_llvm$";

///
/// Gets the list of LLVM attributes provided in the function name.
///
pub(crate) fn get_llvm_attributes(
    identifier: &Identifier,
) -> Result<BTreeSet<era_compiler_llvm_context::Attribute>, Error> {
    let mut valid_attributes = BTreeSet::new();

    let llvm_begin = identifier.inner.find(LLVM_ATTRIBUTE_PREFIX);
    let llvm_end = identifier.inner.find(LLVM_ATTRIBUTE_SUFFIX);
    let attribute_string = if let (Some(llvm_begin), Some(llvm_end)) = (llvm_begin, llvm_end) {
        if llvm_begin < llvm_end {
            &identifier.inner[llvm_begin + LLVM_ATTRIBUTE_PREFIX.len()..llvm_end]
        } else {
            return Ok(valid_attributes);
        }
    } else {
        return Ok(valid_attributes);
    };

    let mut invalid_attributes = BTreeSet::new();
    for value in attribute_string.split('_') {
        match era_compiler_llvm_context::Attribute::try_from(value) {
            Ok(attribute) => valid_attributes.insert(attribute),
            Err(value) => invalid_attributes.insert(value),
        };
    }

    if !invalid_attributes.is_empty() {
        return Err(ParserError::InvalidAttributes {
            location: identifier.location,
            values: invalid_attributes,
        }
        .into());
    }

    Ok(valid_attributes)
}

//!
//! Parser of LLVM attributes encoded in the function identifier.
//!

use std::collections::BTreeSet;

use crate::yul::error::Error as YulError;
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
) -> Result<BTreeSet<era_compiler_llvm_context::Attribute>, YulError> {
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

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use crate::yul::error::Error as YulError;
    use crate::yul::lexer::token::location::Location;
    use crate::yul::parser::error::Error as ParserError;
    use crate::yul::parser::identifier::Identifier;

    use super::get_llvm_attributes;

    fn identifier_of(name: &str) -> Identifier {
        Identifier {
            location: Location { line: 0, column: 0 },
            inner: name.to_string(),
            r#type: None,
        }
    }

    fn attribute_helper(s: &&str) -> era_compiler_llvm_context::Attribute {
        era_compiler_llvm_context::Attribute::try_from(*s).expect(
            "Internal error in test: trying to create an instance of `era_compiler_llvm_context::Attribute` from an invalid string representation.",
        )
    }
    fn immediate_attributes(
        representations: &[&str],
    ) -> BTreeSet<era_compiler_llvm_context::Attribute> {
        representations.iter().map(attribute_helper).collect()
    }

    #[test]
    fn parse_single_attribute() {
        let input = r#"
$llvm_Hot_llvm$
"#;
        let expected = immediate_attributes(&["Hot"]);
        let result = get_llvm_attributes(&identifier_of(input)).unwrap_or_else(|_| {
            panic!(
                "LLVM attribute parser should be able to parse a valid input: \"{}\"",
                input
            )
        });
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_multiple_attributes() {
        let input = r#"
$llvm_Hot_Cold_MinSize_llvm$
"#;
        let expected = immediate_attributes(&["Cold", "Hot", "MinSize"]);
        let result = get_llvm_attributes(&identifier_of(input)).unwrap_or_else(|_| {
            panic!(
                "LLVM attribute parser should be able to parse a valid input: \"{}\"",
                input
            )
        });
        assert_eq!(result, expected)
    }
    #[test]
    fn parse_malformed_attributes() {
        let input = r#"
$llvm____*&@_llvm$
"#;
        get_llvm_attributes(&identifier_of(input)).expect_err(&format!(
            "LLVM attributes parser should not parse attributes from the malformed input \"{}\"",
            input
        ));
    }

    #[test]
    fn parse_invalid_attributes() {
        let input = r#"
$llvm_Hot_Cold_MinSize_BogusAttr_llvm$
"#;

        let values = BTreeSet::from(["BogusAttr".into()]);
        let location = Location { line: 0, column: 0 };
        let expected = YulError::Parser(ParserError::InvalidAttributes { location, values });
        let result = get_llvm_attributes(&identifier_of(input))
            .expect_err("LLVM attributes parser should not mask unknown attributes");

        assert_eq!(result, expected);
    }
}

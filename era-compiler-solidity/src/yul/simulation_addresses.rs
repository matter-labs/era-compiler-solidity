//!
//! Substitution of EraVM call simulation addresses held in variables.
//!
//! An EraVM extension is selected if the destination of `call`, `staticcall` or `delegatecall`
//! is a literal simulation address. The solc optimizer can replace the literal with a variable
//! that holds it, e.g. with a value moved into another variable in between:
//!
//! ```yul
//! let a := 0xFFFC
//! let _1 := mload(0x40)
//! let r := staticcall(gas(), a, _1, 4, _1, 32)
//! ```
//!
//! After `let a := <simulation address>`, the pass substitutes the literal for the destination
//! `a` in the following statements of the same block, up to the first statement that is not a
//! declaration, an assignment or an expression statement, or that assigns `a`. The condition
//! of an `if` and the expression of a `switch` that end the run are evaluated next, so they are
//! covered too. Yul expressions can't assign variables, and functions can't access the variables
//! of the caller, so `a` holds the literal at every substituted call.
//!
//! Only call destinations are substituted, and only with simulation addresses, so the code of
//! contracts that don't call a simulation this way doesn't change.
//!

use std::collections::HashMap;

use num::ToPrimitive;

use era_yul::yul::lexer::token::lexeme::literal::integer::Integer;
use era_yul::yul::lexer::token::lexeme::literal::Literal as LexicalLiteral;
use era_yul::yul::parser::statement::block::Block;
use era_yul::yul::parser::statement::expression::function_call::name::Name;
use era_yul::yul::parser::statement::expression::literal::Literal;
use era_yul::yul::parser::statement::expression::Expression;
use era_yul::yul::parser::statement::variable_declaration::VariableDeclaration;
use era_yul::yul::parser::statement::Statement;

use crate::yul::parser::dialect::era::EraDialect;

///
/// Substitutes the simulation addresses in `block` and the blocks nested in it.
///
pub fn run(block: &mut Block<EraDialect>) {
    // The variables of the current run declared with a simulation address.
    let mut literals: HashMap<String, Literal> = HashMap::new();
    for statement in block.statements.iter_mut() {
        match statement {
            Statement::Expression(expression) => replace_destinations(expression, &literals),
            Statement::VariableDeclaration(declaration) => {
                if let Some(expression) = declaration.expression.as_mut() {
                    replace_destinations(expression, &literals);
                }
                for binding in declaration.bindings.iter() {
                    literals.remove(binding.inner.as_str());
                }
                if let Some((name, literal)) = literal_declaration(declaration) {
                    literals.insert(name.to_owned(), literal.clone());
                }
            }
            Statement::Assignment(assignment) => {
                replace_destinations(&mut assignment.initializer, &literals);
                for binding in assignment.bindings.iter() {
                    literals.remove(binding.inner.as_str());
                }
            }
            Statement::IfConditional(conditional) => {
                replace_destinations(&mut conditional.condition, &literals);
                literals.clear();
            }
            Statement::Switch(switch) => {
                replace_destinations(&mut switch.expression, &literals);
                literals.clear();
            }
            Statement::Object(_)
            | Statement::Code(_)
            | Statement::Block(_)
            | Statement::ForLoop(_)
            | Statement::FunctionDefinition(_)
            | Statement::Continue(_)
            | Statement::Break(_)
            | Statement::Leave(_) => literals.clear(),
        }
        run_nested(statement);
    }
}

///
/// Returns the variable and the value of a `let a := <simulation address>` declaration.
///
fn literal_declaration(declaration: &VariableDeclaration) -> Option<(&str, &Literal)> {
    match (declaration.bindings.as_slice(), &declaration.expression) {
        (
            [binding],
            Some(Expression::Literal(
                literal @ Literal {
                    inner: LexicalLiteral::Integer(integer),
                    ..
                },
            )),
        ) if is_simulation_address(integer) => Some((binding.inner.as_str(), literal)),
        _ => None,
    }
}

///
/// Whether `integer` is the address of an EraVM call simulation.
///
fn is_simulation_address(integer: &Integer) -> bool {
    let value = match integer {
        Integer::Decimal { inner } => num::BigUint::parse_bytes(inner.as_bytes(), 10),
        Integer::Hexadecimal { inner } => inner
            .strip_prefix("0x")
            .and_then(|digits| num::BigUint::parse_bytes(digits.as_bytes(), 16)),
    };
    // The simulation addresses are contiguous.
    value.and_then(|value| value.to_u16()).is_some_and(|value| {
        (era_compiler_common::ERAVM_ADDRESS_ACTIVE_PTR_SWAP
            ..=era_compiler_common::ERAVM_ADDRESS_TO_L1)
            .contains(&value)
    })
}

///
/// Substitutes the literals for the call destinations in `expression` named in `literals`.
///
fn replace_destinations(expression: &mut Expression, literals: &HashMap<String, Literal>) {
    if literals.is_empty() {
        return;
    }
    let Expression::FunctionCall(call) = expression else {
        return;
    };
    let is_call = matches!(
        call.name,
        Name::Call | Name::StaticCall | Name::DelegateCall
    );
    for (index, argument) in call.arguments.iter_mut().enumerate() {
        if let Expression::Identifier(identifier) = argument {
            if let Some(literal) = literals
                .get(identifier.inner.as_str())
                .filter(|_| is_call && index == 1)
            {
                let mut literal = literal.clone();
                literal.location = identifier.location;
                *argument = Expression::Literal(literal);
            }
        } else {
            replace_destinations(argument, literals);
        }
    }
}

///
/// Substitutes the simulation addresses in the blocks nested in `statement`.
///
fn run_nested(statement: &mut Statement<EraDialect>) {
    match statement {
        Statement::Block(block) => run(block),
        Statement::IfConditional(conditional) => run(&mut conditional.block),
        Statement::Switch(switch) => {
            for case in switch.cases.iter_mut() {
                run(&mut case.block);
            }
            if let Some(default) = switch.default.as_mut() {
                run(default);
            }
        }
        Statement::ForLoop(for_loop) => {
            run(&mut for_loop.initializer);
            run(&mut for_loop.body);
            run(&mut for_loop.finalizer);
        }
        Statement::FunctionDefinition(function) => run(&mut function.body),
        Statement::Object(_)
        | Statement::Code(_)
        | Statement::Expression(_)
        | Statement::VariableDeclaration(_)
        | Statement::Assignment(_)
        | Statement::Continue(_)
        | Statement::Break(_)
        | Statement::Leave(_) => {}
    }
}

#[cfg(test)]
mod tests {
    use era_yul::yul::lexer::token::lexeme::literal::integer::Integer;
    use era_yul::yul::lexer::token::lexeme::literal::Literal as LexicalLiteral;
    use era_yul::yul::lexer::Lexer;
    use era_yul::yul::parser::statement::block::Block;
    use era_yul::yul::parser::statement::expression::function_call::name::Name;
    use era_yul::yul::parser::statement::expression::Expression;
    use era_yul::yul::parser::statement::object::Object;
    use era_yul::yul::parser::statement::Statement;

    use crate::yul::parser::dialect::era::EraDialect;

    ///
    /// Runs the pass on `code` and returns the destination of every `staticcall` in source order:
    /// the literal text if it is a literal after the pass, or `None` if it is still a variable.
    ///
    fn destinations(code: &str) -> Vec<Option<String>> {
        let input = format!("object \"Test\" {{ code {{ {code} }} }}");
        let mut object =
            Object::<EraDialect>::parse(&mut Lexer::new(input), None).expect("Invalid test input");
        super::run(&mut object.code.block);

        let mut destinations = Vec::new();
        collect_block(&object.code.block, &mut destinations);
        destinations
    }

    fn collect_block(block: &Block<EraDialect>, destinations: &mut Vec<Option<String>>) {
        for statement in block.statements.iter() {
            match statement {
                Statement::Block(block) => collect_block(block, destinations),
                Statement::Expression(expression) => collect_expression(expression, destinations),
                Statement::VariableDeclaration(declaration) => {
                    if let Some(expression) = declaration.expression.as_ref() {
                        collect_expression(expression, destinations);
                    }
                }
                Statement::Assignment(assignment) => {
                    collect_expression(&assignment.initializer, destinations)
                }
                Statement::IfConditional(conditional) => {
                    collect_expression(&conditional.condition, destinations);
                    collect_block(&conditional.block, destinations);
                }
                Statement::Switch(switch) => {
                    collect_expression(&switch.expression, destinations);
                    for case in switch.cases.iter() {
                        collect_block(&case.block, destinations);
                    }
                    if let Some(default) = switch.default.as_ref() {
                        collect_block(default, destinations);
                    }
                }
                Statement::ForLoop(for_loop) => {
                    collect_block(&for_loop.initializer, destinations);
                    collect_expression(&for_loop.condition, destinations);
                    collect_block(&for_loop.body, destinations);
                    collect_block(&for_loop.finalizer, destinations);
                }
                Statement::FunctionDefinition(function) => {
                    collect_block(&function.body, destinations)
                }
                _ => {}
            }
        }
    }

    fn collect_expression(expression: &Expression, destinations: &mut Vec<Option<String>>) {
        let Expression::FunctionCall(call) = expression else {
            return;
        };
        if call.name == Name::StaticCall {
            destinations.push(match &call.arguments[1] {
                Expression::Literal(literal) => match &literal.inner {
                    LexicalLiteral::Integer(
                        Integer::Decimal { inner } | Integer::Hexadecimal { inner },
                    ) => Some(inner.clone()),
                    _ => None,
                },
                _ => None,
            });
        }
        for argument in call.arguments.iter() {
            collect_expression(argument, destinations);
        }
    }

    fn known(value: &str) -> Option<String> {
        Some(value.to_owned())
    }

    #[test]
    fn literal_stays() {
        assert_eq!(
            destinations("pop(staticcall(0, 0xFFFC, 0, 0, 0, 0))"),
            vec![known("0xFFFC")]
        );
    }

    #[test]
    fn adjacent_declaration() {
        assert_eq!(
            destinations("let a := 0xFFFC let r := staticcall(0, a, 0, 0, 0, 0)"),
            vec![known("0xFFFC")]
        );
        assert_eq!(
            destinations("let r let a := 0xFFFC r := staticcall(0, a, 0, 0, 0, 0)"),
            vec![known("0xFFFC")]
        );
        assert_eq!(
            destinations("let a := 0xFFFC pop(staticcall(0, a, 0, 0, 0, 0))"),
            vec![known("0xFFFC")]
        );
    }

    #[test]
    fn decimal() {
        assert_eq!(
            destinations("let a := 65532 pop(staticcall(0, a, 0, 0, 0, 0))"),
            vec![known("65532")]
        );
    }

    #[test]
    fn call_nested_in_expression() {
        assert_eq!(
            destinations("let a := 0xFFFC mstore(0, add(1, staticcall(0, a, 0, 0, 0, 0)))"),
            vec![known("0xFFFC")]
        );
    }

    #[test]
    fn nested_blocks() {
        assert_eq!(
            destinations(
                r#"
                let n := calldataload(0)
                if n { let a := 0xFFFC pop(staticcall(0, a, 0, 0, 0, 0)) }
                switch n
                case 0 { let a := 0xFFFC pop(staticcall(0, a, 0, 0, 0, 0)) }
                default { let a := 0xFFFC pop(staticcall(0, a, 0, 0, 0, 0)) }
                for { } n { } { let a := 0xFFFC pop(staticcall(0, a, 0, 0, 0, 0)) n := 0 }
                { let a := 0xFFFC pop(staticcall(0, a, 0, 0, 0, 0)) }
                function g() { let a := 0xFFFC pop(staticcall(0, a, 0, 0, 0, 0)) }
                "#
            ),
            vec![known("0xFFFC"); 6]
        );
    }

    #[test]
    fn straight_line_run() {
        assert_eq!(
            destinations(
                r#"
                let a := 0xFFFC
                mstore(0, 1)
                let p := mload(0x40)
                let r := staticcall(0, a, p, 4, p, 32)
                r := add(r, staticcall(0, a, 0, 0, 0, 0))
                "#
            ),
            vec![known("0xFFFC"), known("0xFFFC")]
        );
    }

    #[test]
    fn if_condition_and_switch_expression() {
        assert_eq!(
            destinations(
                r#"
                let a := 0xFFFC
                if iszero(staticcall(0, a, 0, 0, 0, 0)) { revert(0, 0) }
                let b := 0xFFFC
                switch staticcall(0, b, 0, 0, 0, 0) case 0 { } default { }
                "#
            ),
            vec![known("0xFFFC"), known("0xFFFC")]
        );
    }

    #[test]
    fn assignment_ends_the_run() {
        assert_eq!(
            destinations(
                r#"
                let a := 0xFFFC
                a := staticcall(0, a, 0, 0, 0, 0)
                pop(staticcall(0, a, 0, 0, 0, 0))
                "#
            ),
            vec![known("0xFFFC"), None]
        );
        assert_eq!(
            destinations(
                r#"
                let a := 0xFFFC
                let b, c := two()
                a, b := two()
                pop(staticcall(0, a, 0, 0, 0, 0))
                function two() -> x, y { }
                "#
            ),
            vec![None]
        );
    }

    #[test]
    fn redeclaration_ends_the_run() {
        // Not valid Yul, but zksolc does not validate `--yul` input with EraVM extensions.
        assert_eq!(
            destinations(
                "let a := 0xFFFC let a := calldataload(0) pop(staticcall(0, a, 0, 0, 0, 0))"
            ),
            vec![None]
        );
    }

    #[test]
    fn control_flow_ends_the_run() {
        assert_eq!(
            destinations(
                r#"
                let a := 0xFFFC
                if calldataload(0) { a := 0x8002 }
                pop(staticcall(0, a, 0, 0, 0, 0))
                "#
            ),
            vec![None]
        );
        assert_eq!(
            destinations(
                r#"
                let a := 0xFFFC
                switch calldataload(0) case 0 { a := 0x8002 } default { }
                pop(staticcall(0, a, 0, 0, 0, 0))
                "#
            ),
            vec![None]
        );
        assert_eq!(
            destinations(
                r#"
                let a := 0xFFFC
                { a := 0x8002 }
                pop(staticcall(0, a, 0, 0, 0, 0))
                "#
            ),
            vec![None]
        );
    }

    #[test]
    fn loop_condition_is_not_in_the_run() {
        // The condition is evaluated again after the body.
        assert_eq!(
            destinations(
                r#"
                let a := 0xFFFC
                for { } staticcall(0, a, 0, 0, 0, 0) { } { a := 0x8002 }
                "#
            ),
            vec![None]
        );
    }

    #[test]
    fn not_in_the_same_block() {
        assert_eq!(
            destinations(
                r#"
                let a := 0xFFFC
                if calldataload(0) { pop(staticcall(0, a, 0, 0, 0, 0)) }
                { let b := 0xFFFC }
                pop(staticcall(0, b, 0, 0, 0, 0))
                "#
            ),
            vec![None, None]
        );
    }

    #[test]
    fn loop_body() {
        assert_eq!(
            destinations(
                r#"
                let a := 0xFFFC
                for { } lt(a, 0xFFFF) { } { pop(staticcall(0, a, 0, 0, 0, 0)) a := add(a, 1) }
                "#
            ),
            vec![None]
        );
    }

    #[test]
    fn assignment_is_not_a_declaration() {
        assert_eq!(
            destinations("let a := calldataload(0) a := 0xFFFC pop(staticcall(0, a, 0, 0, 0, 0))"),
            vec![None]
        );
    }

    #[test]
    fn non_literal_declaration() {
        assert_eq!(
            destinations("let a := calldataload(0) pop(staticcall(0, a, 0, 0, 0, 0))"),
            vec![None]
        );
        assert_eq!(
            destinations("let b := 0xFFFC let a := b pop(staticcall(0, a, 0, 0, 0, 0))"),
            vec![None]
        );
    }

    #[test]
    fn multiple_bindings() {
        assert_eq!(
            destinations(
                r#"
                function two() -> p, q { p := 0xFFFC q := 0xFFFC }
                let a, b := two()
                pop(staticcall(0, a, 0, 0, 0, 0))
                "#
            ),
            vec![None]
        );
    }

    #[test]
    fn other_variable() {
        assert_eq!(
            destinations(
                "let b := calldataload(0) let a := 0xFFFC pop(staticcall(0, b, 0, 0, 0, 0))"
            ),
            vec![None]
        );
    }

    #[test]
    fn only_simulation_addresses() {
        assert_eq!(
            destinations(
                r#"
                let a := 0x8002
                pop(staticcall(0, a, 0, 0, 0, 0))
                let b := 0xFFD8
                pop(staticcall(0, b, 0, 0, 0, 0))
                let c := 0x1FFFC
                pop(staticcall(0, c, 0, 0, 0, 0))
                let d := 0xFFD9
                pop(staticcall(0, d, 0, 0, 0, 0))
                let e := 0xFFFF
                pop(staticcall(0, e, 0, 0, 0, 0))
                "#
            ),
            vec![None, None, None, known("0xFFD9"), known("0xFFFF")]
        );
    }

    #[test]
    fn only_the_destination() {
        let input = r#"object "Test" { code {
            let a := 0xFFFC
            pop(staticcall(a, 0x8002, a, a, a, a))
        } }"#;
        let mut object = Object::<EraDialect>::parse(&mut Lexer::new(input.to_owned()), None)
            .expect("Invalid test input");
        let before = object.clone();
        super::run(&mut object.code.block);
        assert_eq!(object, before);
    }

    #[test]
    fn all_call_kinds() {
        let input = r#"object "Test" { code {
            let a := 0xFFFC
            pop(call(0, a, 0, 0, 0, 0, 0))
            pop(delegatecall(0, a, 0, 0, 0, 0))
        } }"#;
        let mut object = Object::<EraDialect>::parse(&mut Lexer::new(input.to_owned()), None)
            .expect("Invalid test input");
        super::run(&mut object.code.block);
        let destinations: Vec<bool> = object.code.block.statements[1..]
            .iter()
            .map(|statement| match statement {
                Statement::Expression(Expression::FunctionCall(pop)) => match &pop.arguments[0] {
                    Expression::FunctionCall(call) => {
                        matches!(call.arguments[1], Expression::Literal(_))
                    }
                    _ => false,
                },
                _ => false,
            })
            .collect();
        assert_eq!(destinations, vec![true, true]);
    }

    #[test]
    fn only_call_arguments() {
        let input = r#"object "Test" { code { let a := 0xFFFC mstore(a, a) } }"#;
        let mut object = Object::<EraDialect>::parse(&mut Lexer::new(input.to_owned()), None)
            .expect("Invalid test input");
        let before = object.clone();
        super::run(&mut object.code.block);
        assert_eq!(object, before);
    }
}

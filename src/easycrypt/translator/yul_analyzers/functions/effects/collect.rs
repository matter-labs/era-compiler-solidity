//!
//! Collect effects occurring in a procedure. Does not propagate them.
//!

use crate::easycrypt::translator::definition_info::attributes::Attributes;
use crate::easycrypt::translator::definition_info::get_definition;
use crate::easycrypt::translator::definition_info::kind::Kind;
use crate::easycrypt::translator::definition_info::kind::ProcKind;
use crate::easycrypt::translator::definition_info::DefinitionInfo;
use crate::easycrypt::translator::yul_analyzers::common::state::State;

use crate::yul::parser::statement::assignment::Assignment;
use crate::yul::parser::statement::expression::function_call::FunctionCall;
use crate::yul::parser::statement::expression::Expression;
use crate::yul::parser::statement::for_loop::ForLoop;
use crate::yul::parser::statement::if_conditional::IfConditional;
use crate::yul::parser::statement::switch::Switch;
use crate::yul::parser::statement::variable_declaration::VariableDeclaration;
use crate::yul::parser::statement::Statement;
use crate::yul::path::symbol_table::SymbolTable;
use crate::yul::path::Path;

use crate::yul::visitor::statements::StatementAction;

/// Collect effects occurring in a procedure. Does not propagate them.
pub struct CollectEffects<'a> {
    pub result: Attributes,
    pub state: State<'a>,
}

impl<'a> CollectEffects<'a> {
    pub fn new(symbol_table: &'a mut SymbolTable<DefinitionInfo>) -> Self {
        Self {
            result: Attributes::default(),
            state: State::new(symbol_table),
        }
    }
}
impl<'a> StatementAction for CollectEffects<'a> {
    fn action(&mut self, statement: &Statement, path: &Path) {
        match statement {
            Statement::Expression(expression)
            | Statement::VariableDeclaration(VariableDeclaration {
                expression: Some(expression),
                ..
            })
            | Statement::Assignment(Assignment {
                initializer: expression,
                ..
            })
            | Statement::IfConditional(IfConditional {
                condition: expression,
                ..
            })
            | Statement::Switch(Switch { expression, .. })
            | Statement::ForLoop(ForLoop {
                condition: expression,
                ..
            }) => {
                self.result =
                    self.result
                        .union(collect_effects(expression, path, self.state.symbol_table));
            }
            _ => (),
        }
    }
}

fn collect_effects(
    expr: &Expression,
    path: &Path,
    symbols: &SymbolTable<DefinitionInfo>,
) -> Attributes {
    let mut result: Attributes = Default::default();
    if let Expression::FunctionCall(FunctionCall {
        name, arguments, ..
    }) = expr
    {
        let DefinitionInfo { kind, .. } = &get_definition(symbols, name, path).unwrap();
        if let Kind::Proc(ProcKind { attributes, .. }) = kind {
            result = result.union(attributes.clone())
        }

        for argument in arguments.iter() {
            result = result.union(collect_effects(argument, path, symbols))
        }
    }
    result
}

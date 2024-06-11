//!
//! Decide which functions will be transpiled to EasyCrypt procedures, and which
//! will be transpiled to EasyCrypt functions.
//!

use anyhow::Error;

use crate::easycrypt::syntax::function::name::FunctionName;
use crate::easycrypt::syntax::proc::name::ProcName;
use crate::easycrypt::translator::definition_info::get_definition;
use crate::easycrypt::translator::definition_info::kind::proc_kind::ProcKind;
use crate::easycrypt::translator::definition_info::kind::Kind;
use crate::easycrypt::translator::definition_info::DefinitionInfo;
use crate::easycrypt::translator::yul_analyzers::common::round::Round;
use crate::easycrypt::translator::yul_analyzers::common::state::State;

use crate::yul::parser::statement::expression::function_call::FunctionCall;
use crate::yul::parser::statement::expression::Expression;
use crate::yul::parser::statement::function_definition::FunctionDefinition;
use crate::yul::parser::statement::object::Object;
use crate::yul::parser::statement::Statement;
use crate::yul::path::full_name::FullName;
use crate::yul::path::symbol_table::SymbolTable;
use crate::yul::path::Path;

use crate::yul::visitor::statements::StatementAction;
use crate::yul::visitor::statements::Statements;
use crate::YulVisitor;

struct FunctionKindRound<'a> {
    round: Round,
    state: State<'a>,
}

impl<'a> StatementAction for FunctionKindRound<'a> {
    fn action(&mut self, statement: &Statement, path: &Path) {
        if let Statement::FunctionDefinition(fd) = statement {
            let full_name = FullName {
                name: fd.identifier.to_string(),
                path: path.clone(),
            };

            match self.becomes_function(fd, path) {
                Ok(true) => {
                    let definition = self.state.symbol_table.get_mut(&full_name).unwrap();
                    promote_to_function(&mut self.round, definition);
                }
                Ok(false) => (),
                Err(msg) => panic!("{}", msg),
            }
        }
    }
}
/// Promote a procedure to a function. By default, all functions are translated
/// into procedures.
fn promote_to_function(round: &mut Round, definition: &mut DefinitionInfo) {
    match &definition.kind {
        Kind::Proc(ProcKind {
            name: ProcName::UserDefined(name),
            ..
        }) => {
            definition.kind = Kind::Function(FunctionName::UserDefined(name.clone()));
            round.register_effect()
        }
        Kind::Function(_) => (),
        Kind::Variable => panic!("Can not promote variable to function"),
        _ => panic!("Can not promote standard procedures to functions"),
    }
}

impl<'a> FunctionKindRound<'a> {
    /// Returns a new instance.
    pub fn new(all_definitions: &'a mut SymbolTable<DefinitionInfo>) -> Self {
        Self {
            round: Round::new(),
            state: State::new(all_definitions),
        }
    }

    /// Returns true if the round did any work.
    pub fn had_effect(&self) -> bool {
        self.round.had_effect()
    }

    /// Recursively analyze an expression to determine if it has any calls to
    /// procedures.
    fn prevents_promotion(&self, expr: &Expression, path: &Path) -> Result<bool, Error> {
        match expr {
            Expression::FunctionCall(FunctionCall {
                name, arguments, ..
            }) => {
                let kind = &get_definition(self.state.symbol_table, name, path)?.kind;
                if matches!(kind, Kind::Proc(_) | Kind::Special(_)) {
                    Ok(true)
                } else {
                    for argument in arguments.iter() {
                        if self.prevents_promotion(argument, path)? {
                            return Ok(true);
                        }
                    }
                    Ok(false)
                }
            }
            Expression::Identifier(_) | Expression::Literal(_) => Ok(false),
        }
    }

    /// True if the image of YUL function can be promoted from EasyCrypt procedure to EasyCrypt function.
    fn becomes_function(
        &self,
        function_definition: &FunctionDefinition,
        path: &Path,
    ) -> Result<bool, Error> {
        let FunctionDefinition { result, body, .. } = function_definition;
        if body.statements.len() != 1 {
            Ok(false)
        } else {
            match &body.statements[0] {
                Statement::Assignment(assignment) => Ok(assignment
                    .bindings
                    .iter()
                    .map(|i| &i.inner)
                    .zip(result.iter().map(|d| &d.inner))
                    .all(|(x, y)| x == y)
                    && !self.prevents_promotion(&assignment.initializer, path)?),
                _ => Ok(false),
            }
        }
    }
}

/// Infer types of all YUL functions.
pub fn infer_function_types(environment: &mut SymbolTable<DefinitionInfo>, root: &Object) {
    while {
        let mut stmts = Statements::new(FunctionKindRound::new(environment), Path::empty());
        stmts.visit_object(root);
        stmts.action.had_effect()
    } {}
}

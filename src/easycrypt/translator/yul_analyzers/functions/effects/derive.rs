//!
//! Derive the effects for the procedures, propagating them until fixpoint.
//!

use crate::easycrypt::translator::definition_info::kind::Kind;
use crate::easycrypt::translator::definition_info::kind::ProcKind;
use crate::easycrypt::translator::definition_info::DefinitionInfo;
use crate::easycrypt::translator::yul_analyzers::common::round::Round;
use crate::easycrypt::translator::yul_analyzers::common::state::State;

use crate::yul::parser::statement::object::Object;
use crate::yul::parser::statement::Statement;
use crate::yul::path::full_name::FullName;
use crate::yul::path::symbol_table::SymbolTable;
use crate::yul::path::Path;

use crate::yul::visitor::statements::StatementAction;
use crate::yul::visitor::statements::Statements;
use crate::yul::visitor::Visitor as _;

use super::collect::CollectEffects;

pub struct Effects<'a> {
    round: Round,
    state: State<'a>,
}

impl<'a> Effects<'a> {
    pub fn new(symbol_table: &'a mut SymbolTable<DefinitionInfo>) -> Self {
        Self {
            round: Round::default(),
            state: State::new(symbol_table),
        }
    }

    pub fn had_effect(&self) -> bool {
        self.round.had_effect()
    }
}

impl<'a> StatementAction for Effects<'a> {
    fn action(&mut self, statement: &Statement, path: &Path) {
        if let Statement::FunctionDefinition(fd) = statement {
            let full_name = FullName {
                name: fd.identifier.to_string(),
                path: path.clone(),
            };

            let effects = {
                let mut stmts =
                    Statements::new(CollectEffects::new(self.state.symbol_table), path.clone());
                stmts.visit_block(&fd.body);
                stmts.action.result
            };
            let definition = self.state.symbol_table.get_mut(&full_name).unwrap();

            if let Kind::Proc(ProcKind { attributes, .. }) = &mut definition.kind {
                let old_effects = attributes.clone();
                let new_effects = attributes.union(effects);
                if old_effects != new_effects {
                    self.round.register_effect();
                    *attributes = new_effects;
                }
            }
        }
    }
}

/// Infer types of all YUL functions.
pub fn infer_effects(environment: &mut SymbolTable<DefinitionInfo>, root: &Object) {
    while {
        let mut stmts = Statements::new(Effects::new(environment), Path::empty());
        stmts.visit_object(root);
        stmts.action.had_effect()
    } {}
}

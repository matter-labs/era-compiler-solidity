//!
//! Collect all the definitions in YUL program and derive their EasyCrypt types.
//!

use std::iter::repeat;

use crate::easycrypt::syntax::proc::name::ProcName;
use crate::easycrypt::syntax::r#type::Type as ECType;
use crate::easycrypt::translator::definition_info::kind::Kind;
use crate::easycrypt::translator::definition_info::kind::ProcKind;
use crate::easycrypt::translator::definition_info::DefinitionInfo;
use crate::yul::parser::identifier::Identifier;
use crate::yul::parser::statement::function_definition::FunctionDefinition;
use crate::yul::parser::statement::variable_declaration::VariableDeclaration;
use crate::yul::parser::statement::Statement;
use crate::yul::path::full_name::FullName;
use crate::yul::path::symbol_table::SymbolTable;
use crate::yul::path::tracker::symbol_tracker::SymbolTracker;
use crate::yul::path::Path;
use crate::yul::visitor::statements::StatementAction;

/// Collect all definitions in the YUL code.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CollectDefinitions {
    /// Tracks current location and allows binding the identifiers in the current scope.
    pub tracker: SymbolTracker<DefinitionInfo>,
    /// Collects all definitions in YUL code.
    pub all_symbols: SymbolTable<DefinitionInfo>,
}

impl StatementAction for CollectDefinitions {
    fn action(&mut self, statement: &Statement, path: &Path) {
        match statement {
            Statement::FunctionDefinition(fd) => self.add_function_definition(fd, path),
            Statement::VariableDeclaration(vd) => self.add_variable_declaration(vd, path),
            _ => (),
        }
    }
}

impl CollectDefinitions {
    pub fn new() -> Self {
        Self {
            tracker: SymbolTracker::new(),
            all_symbols: SymbolTable::new(),
        }
    }

    fn add_var(&mut self, binding: &Identifier, path: &Path) {
        let name = &binding.inner;
        let full_name = FullName {
            name: name.clone(),
            path: path.clone(),
        };
        let r#type = ECType::Unknown.clone();
        let definition = DefinitionInfo {
            kind: Kind::Variable,
            r#type,
            full_name: full_name.clone(),
        };
        self.tracker.add(name, &definition);
        self.all_symbols.insert(&full_name, &definition);
    }

    fn add_variable_declaration(
        &mut self,
        variable_declaration: &VariableDeclaration,
        path: &Path,
    ) {
        for binding in &variable_declaration.bindings {
            self.add_var(binding, path);
        }
    }

    fn add_function_definition(&mut self, function_definition: &FunctionDefinition, path: &Path) {
        let FunctionDefinition {
            identifier,
            arguments,
            result,
            ..
        } = function_definition;
        for argument in arguments {
            self.add_var(argument, path)
        }

        let name = &identifier;

        let full_name = FullName {
            name: name.to_string(),
            path: path.clone(),
        };
        let r#type = {
            let arg_type = ECType::type_of_vec(
                &repeat(ECType::Unknown)
                    .take(arguments.len())
                    .collect::<Vec<_>>(),
            );
            let ret_type = ECType::Unknown.clone();
            ECType::Arrow(Box::new(arg_type), Box::new(ret_type))
        };
        let definition = DefinitionInfo {
            kind: Kind::Proc(ProcKind {
                name: ProcName::UserDefined(name.to_string()),
                attributes: Default::default(),
            }),
            r#type,
            full_name: full_name.clone(),
        };
        self.tracker.add(name, &definition);
        self.all_symbols.insert(&full_name, &definition);

        for return_value in result {
            self.add_var(return_value, path)
        }
    }
}

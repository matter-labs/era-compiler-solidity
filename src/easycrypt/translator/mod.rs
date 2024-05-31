//!
//! Transpiler from YUL to EasyCrypt
//!

pub mod block;
pub mod code;
pub mod context;
pub mod definition_info;
pub mod expression;
pub mod function;
pub mod identifier;
pub mod object;
pub mod statement;
pub mod r#type;
pub mod yul_analyzers;

use anyhow::Error;


use crate::easycrypt::syntax::definition::Definition;
use crate::easycrypt::syntax::module::Module;
use crate::easycrypt::syntax::r#type::Type;
use crate::util::counter::Counter;
use crate::yul::parser::identifier::Identifier as YulIdentifier;
use crate::yul::parser::statement::object::Object as YulObject;
use crate::yul::path::Path;
use crate::yul::path::full_name::FullName;
use crate::yul::path::symbol_table::SymbolTable;
use crate::yul::path::tracker::PathTracker;
use crate::yul::path::tracker::symbol_tracker::SymbolTracker;
use crate::yul::visitor::statements::Statements;

use self::definition_info::DefinitionInfo;
use self::yul_analyzers::collect_definitions::CollectDefinitions;
use self::yul_analyzers::functions::inferrer::infer_function_types;

/// Global state of YUL to EasyCrypt translator
#[derive(Debug)]
pub struct Translator {
    root: YulObject,
    tracker: SymbolTracker<definition_info::DefinitionInfo>,
    tmp_counter: Counter,
    definitions: SymbolTable<DefinitionInfo>,
}

impl Translator {
    /// Transpile an object
    pub fn transpile(yul_object: &YulObject) -> Result<Module, Error> {
        let mut result = Self {
            root: yul_object.clone(),
            tracker: SymbolTracker::new(),
            tmp_counter: Counter::new(),
            definitions: SymbolTable::new(),
        };

        result.init();
        result.transpile_object(yul_object, true)
    }

    fn init(&mut self) {
        self.definitions = Statements::from(&self.root)
            .for_each(CollectDefinitions::new())
            .all_symbols;

        infer_function_types(&mut self.definitions, &self.root);
    }

    fn new_definition_here(&self, name: &str, typ: Option<Type>) -> Definition {
        Self::new_definition(self.here(), name, typ)
    }

    fn new_tmp_definition_here(&mut self) -> Definition {
        let name = format!("TMP{}", self.tmp_counter.get_value());
        self.tmp_counter.increment();
        Self::new_definition(self.here(), &name, None)
    }

    fn here(&self) -> Path {
        self.tracker.here().clone()
    }

    fn bindings_to_definitions(&self, idents: &[YulIdentifier]) -> Vec<Definition> {
        idents
            .iter()
            .map(|ident| Definition {
                identifier: ident.inner.clone(),
                location: Some(self.here()),
                r#type: ident
                    .r#type
                    .as_ref()
                    .and_then(|t| Self::transpile_type(t).ok()),
            })
            .collect()
    }

    fn new_definition(location: Path, name: &str, r#type: Option<Type>) -> Definition {
        Definition {
            identifier: String::from(name),
            location: Some(location),
            r#type,
        }
    }

    fn create_full_name(&self, identifier: &str) -> FullName {
        FullName::new(identifier.to_string(), self.here())
    }

    // fn get_definition_mut(&mut self, name: &str) -> Option<&mut DefinitionInfo> {
    //     let full_name = FullName {
    //         name: name.to_string(),
    //         path: self.here().clone(),
    //     };
    //     self.definitions.get_mut(&full_name)
    // }
}

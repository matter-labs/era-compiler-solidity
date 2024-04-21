pub mod location;

use std::collections::HashMap;

use anyhow::Error;

use crate::{
    util::counter::Counter,
    yul::parser::statement::{
        assignment::Assignment,
        block::Block,
        code::Code,
        expression::{
            function_call::{name::Name, FunctionCall},
            literal::Literal,
            Expression,
        },
        for_loop::ForLoop,
        function_definition::FunctionDefinition,
        if_conditional::IfConditional,
        object::Object,
        switch::Switch,
        variable_declaration::VariableDeclaration,
        Statement,
    },
};

use self::location::LocationBuilder;

use super::syntax::{self, Module};

pub struct Translator {
    pub module: syntax::Module,
    pub location_tracker: location::LocationBuilder,
}

impl Translator {
    pub fn new() -> Self {
        Self {
            module: Module::new(),
            location_tracker: LocationBuilder::new(),
        }
    }

    fn current_module(&mut self) -> Option<&mut Module> {
        todo!()
    }

    fn create_function(&mut self, name: String) {
        //let full_name = self.current_module().insert(value);
    }

    fn visit_switch(&mut self, s: &Switch) {
        todo!()
    }

    pub fn visit_object(&mut self, obj: &Object) -> Result<syntax::Module, Error> {
        // let name = self.context.location_prefix();

        //obj.code
        Ok(syntax::Module {
            definitions: HashMap::new(),
        })
    }

    fn visit_for_loop(&mut self, for_loop: &ForLoop) {
        todo!()
    }

    fn visit_variable_declaration(&mut self, vd: &VariableDeclaration) {
        todo!()
    }

    fn visit_function_definition(&mut self, fd: &FunctionDefinition) {
        todo!()
    }

    fn visit_name(&mut self, name: &Name) {
        todo!()
    }

    fn visit_functioncall(&mut self, call: &FunctionCall) {
        todo!()
    }

    fn visit_if_conditional(&mut self, if_conditional: &IfConditional) {
        todo!()
    }

    fn visit_literal(&mut self, lit: &Literal) {
        todo!()
    }

    fn visit_expression(&mut self, expr: &Expression) {
        todo!()
    }

    fn visit_assignment(&mut self, assignment: &Assignment) {
        todo!()
    }

    fn visit_statement(&mut self, stmt: &Statement) -> Result<TranslatedStatement, Error> {
        todo!()
    }

    fn visit_code_block(&mut self, block: &Block) -> Result<Vec<syntax::Definition>, Error> {
        todo!()
    }
    fn visit_block(&mut self, block: &Block) -> Result<Vec<syntax::Definition>, Error> {
        let mut result = Vec::with_capacity(block.statements.len());
        for yul_stmt in &block.statements {
            match self.visit_statement(&yul_stmt)? {
                TranslatedStatement::Statement(transformed) => result.push(transformed),
                TranslatedStatement::Function(fd) => todo!(),
                TranslatedStatement::Proc(proc) => todo!(),
            }
        }
        todo!()
        //Ok(result)
    }

    fn visit_code(&mut self, code: &Code) -> Result<Vec<syntax::Definition>, Error> {
        //create_function("default")
        return self.visit_block(&code.block);
    }
}
impl Module {
    fn add_function(&mut self, function: syntax::Function) {
        // self.definitions.insert(function.name, syntax::Definition::FunDef(function))
    }
    fn add_proc(&mut self, proc: syntax::Proc) {
        // self.definitions.insert(proc.name, syntax::Definition::ProcDef(proc))
    }
}
pub enum TranslatedStatement {
    Statement(syntax::Statement),
    Function(syntax::Function),
    Proc(syntax::Proc),
}

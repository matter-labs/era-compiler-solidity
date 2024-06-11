//!
//! Transpilation of the `code` block of an arbitrary YUL object.
//!

use std::collections::HashMap;

use anyhow::Error;

use crate::yul::visitor::IMPLICIT_CODE_FUNCTION_NAME;
use crate::Translator;

use crate::easycrypt::syntax::module::definition::TopDefinition;
use crate::easycrypt::syntax::module::Module;
use crate::easycrypt::syntax::proc::Proc;
use crate::easycrypt::syntax::reference::Reference;
use crate::easycrypt::syntax::signature::Signature;
use crate::easycrypt::syntax::statement::block::Block;
use crate::yul::parser::statement::code::Code as YulCode;

use crate::easycrypt::translator::block::Transformed as TransformedBlock;
use crate::easycrypt::translator::context::Context;
use crate::yul::path::tracker::PathTracker;

impl Translator {
    /// Transpile the `code` block of an arbitrary YUL object.
    pub fn transpile_code(&mut self, code: &YulCode) -> Result<Module, Error> {
        self.tracker.enter_code();
        self.call_stack
            .push(self.create_full_name(IMPLICIT_CODE_FUNCTION_NAME));

        let (Context { module, locals }, TransformedBlock { statements }) =
            self.transpile_block(&code.block, &Context::new())?;

        let default_code_proc = Proc {
            name: IMPLICIT_CODE_FUNCTION_NAME.to_string(),
            signature: Signature::UNIT_TO_UNIT,
            body: Block { statements },
            locals,
            location: Some(self.here()),
        };

        let mut new_module = module;

        if !default_code_proc.body.statements.is_empty() {
            new_module.merge(&Module {
                name: None,
                definitions: HashMap::from([(
                    Reference {
                        identifier: IMPLICIT_CODE_FUNCTION_NAME.to_string(),
                        location: Some(self.here()),
                    },
                    TopDefinition::Proc(default_code_proc),
                )]),
            });
        }

        self.tracker.leave();
        self.call_stack.pop();
        Ok(new_module)
    }
}

//!
//! Transpilation of YUL function calls.
//!

use anyhow::Error;

use crate::easycrypt::syntax::expression::call::FunctionCall;
use crate::easycrypt::syntax::expression::Expression;
use crate::easycrypt::syntax::proc::name::ProcName;
use crate::easycrypt::syntax::r#type::Type;
use crate::easycrypt::syntax::statement::call::ProcCall;
use crate::easycrypt::syntax::statement::Statement;
use crate::easycrypt::translator::definition_info::get_definition;
use crate::easycrypt::translator::definition_info::kind::proc_kind::state_formal_parameters;
use crate::easycrypt::translator::definition_info::kind::proc_kind::state_return_vars;
use crate::easycrypt::translator::definition_info::kind::Kind;
use crate::easycrypt::translator::definition_info::kind::YulSpecial;
use crate::easycrypt::translator::expression::Transformed;
use crate::easycrypt::translator::Translator;

use crate::yul::parser::statement::expression::function_call::name::Name as YulName;
use crate::yul::parser::statement::expression::Expression as YulExpression;

use crate::easycrypt::translator::context::Context;
use crate::easycrypt::translator::expression::context::Context as ExprContext;

impl Translator {
    /// Transpile a function call in YUL into either EasyCrypt procedure or function.
    pub fn transpile_function_call(
        &mut self,
        name: &YulName,
        yul_arguments: &[YulExpression],
        ctx: &Context,
        ectx: &ExprContext,
        is_root: bool,
    ) -> Result<Transformed, Error> {
        let caller_full_name = self
            .call_stack
            .last()
            .expect("Internal error: inside a function call but callstack is empty.");
        let caller_definition = self
            .definitions
            .get(caller_full_name)
            .expect("Internal error: caller not found in callstack.");
        let callee_definition = &get_definition(&self.definitions, name, &self.here())?;
        match &callee_definition.kind {
            Kind::Function(target) => {
                let (arguments, ectx) = self.transpile_expression_list(yul_arguments, ctx, ectx)?;
                Ok(Transformed::Expression(
                    Expression::ECall(FunctionCall {
                        target: target.clone(),
                        arguments,
                    }),
                    ectx,
                ))
            }
            Kind::Proc(proc_descr) => {
                let callee_extra_state_arguments = state_formal_parameters(callee_definition);
                let caller_extra_state_arguments = state_formal_parameters(caller_definition);
                let append_arguments =
                    caller_extra_state_arguments.iter().filter_map(|(def, t1)| {
                        if callee_extra_state_arguments.iter().any(|(_, t2)| t1 == t2) {
                            Some(Expression::Reference(def.reference()))
                        } else {
                            None
                        }
                    });

                let (mut arguments, mut ectx) =
                    self.transpile_expression_list(yul_arguments, ctx, ectx)?;
                arguments.extend(append_arguments);

                let returns_unit = matches!(&callee_definition.r#type, Type::Arrow(_, ret_type) if **ret_type == Type::Unit);

                let mut return_vars = {
                    if returns_unit {
                        vec![]
                    } else {
                        let tmp_def = self.new_tmp_definition_here();
                        ectx.locals.push(tmp_def.clone());
                        vec![tmp_def.reference()]
                    }
                };

                let callee_extra_return_vars = state_return_vars(callee_definition);

                let append_return_vars =
                    caller_extra_state_arguments.iter().filter_map(|(def, t1)| {
                        if callee_extra_return_vars.iter().any(|(_, t2)| t1 == t2) {
                            Some(def.reference())
                        } else {
                            None
                        }
                    });
                return_vars.extend(append_return_vars);

                let mut new_ectx = ectx;

                new_ectx.add_multiple_assignment(
                    &return_vars,
                    ProcCall {
                        target: proc_descr.name.clone(),
                        arguments,
                    },
                );

                if return_vars.is_empty() && is_root {
                    Ok(Transformed::Statements(vec![], new_ectx, ctx.clone()))
                } else {
                    Ok(Transformed::Expression(
                        Expression::Reference(return_vars[0].clone()),
                        new_ectx,
                    ))
                }
            }
            Kind::BinOp(optype) => {
                let (arguments, ectx) = self.transpile_expression_list(yul_arguments, ctx, ectx)?;
                Ok(Transformed::Expression(
                    Expression::Binary(
                        optype.clone(),
                        Box::new(arguments[0].clone()),
                        Box::new(arguments[1].clone()),
                    ),
                    ectx,
                ))
            }
            Kind::UnOp(optype) => {
                let (arguments, ectx) = self.transpile_expression_list(yul_arguments, ctx, ectx)?;
                Ok(Transformed::Expression(
                    Expression::Unary(optype.clone(), Box::new(arguments[0].clone())),
                    ectx,
                ))
            }

            Kind::Special(special) if is_root => match special {
                YulSpecial::Return => {
                    let (arguments, ectx) =
                        self.transpile_expression_list(yul_arguments, ctx, ectx)?;
                    assert!(ectx.locals.is_empty());
                    assert!(ectx.assignments.is_empty());
                    Ok(Transformed::Statements(
                        vec![Statement::Return(Expression::pack_tuple(&arguments))],
                        ectx,
                        ctx.clone(),
                    ))
                }
                YulSpecial::Revert => {
                    let (arguments, ectx) =
                        self.transpile_expression_list(yul_arguments, ctx, ectx)?;
                    let passignment = Statement::PAssignment(
                        vec![],
                        ProcCall {
                            target: ProcName::UserDefined {
                                name: String::from("revert"),
                                module: Some(String::from("Primops")),
                            },
                            arguments,
                        },
                    );
                    Ok(Transformed::Statements(
                        vec![passignment],
                        ectx,
                        ctx.clone(),
                    ))
                }
                YulSpecial::Stop | YulSpecial::Invalid => todo!(),
            },
            Kind::Special(_) => {
                anyhow::bail!("Unsupported type of YUL function call.")
            }

            Kind::Variable => anyhow::bail!(
                "Expected a name of function or procedure, got a name of variable instead. {:#?}",
                name
            ),
        }
    }
}

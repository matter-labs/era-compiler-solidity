use anyhow::Error;
use std::collections::HashMap;
use std::iter;

use crate::easycrypt::from_yul::location::LocationBuilder;
use crate::easycrypt::syntax::*;
use crate::util::counter::Counter;
use crate::yul::parser::identifier::Identifier as YulIdentifier;
use crate::yul::parser::r#type::Type as YulType;
use crate::yul::parser::statement::assignment::Assignment as YulAssignment;
use crate::yul::parser::statement::block::Block as YulBlock;
use crate::yul::parser::statement::code::Code as YulCode;
use crate::yul::parser::statement::expression::literal::Literal as YulLiteral;
use crate::yul::parser::statement::expression::{
    function_call::FunctionCall as YulFunctionCall, Expression as YulExpression,
};
use crate::yul::parser::statement::function_definition::FunctionDefinition;
use crate::yul::parser::statement::object::Object;
use crate::yul::parser::statement::variable_declaration::VariableDeclaration;
use crate::yul::parser::statement::Statement as YulStatement;

use super::location::Location;
use crate::yul::parser::statement::expression::function_call::name::Name as YulName;

#[derive(Debug)]
pub struct Translator {
    location_tracker: LocationBuilder,
    tmp_counter: Counter,
}

impl Translator {
    pub fn new() -> Self {
        Self {
            location_tracker: LocationBuilder::new(),
            tmp_counter: Counter::new(),
        }
    }

    fn here(&self) -> Location {
        self.location_tracker.elements.clone()
    }

    pub fn transpile_object(&mut self, obj: &Object, is_root: bool) -> Result<Module, Error> {
        let module_name = &obj.identifier;

        self.location_tracker.enter_object(module_name);
        let mut result = Module::new(if is_root {
            Some(module_name.to_owned())
        } else {
            None
        });

        result.merge(&self.transpile_code(&obj.code)?);

        if let Some(inner_object) = &obj.inner_object {
            let translated_inner_object = self.transpile_object(inner_object.as_ref(), false)?;
            result.merge(&translated_inner_object)
        }

        self.location_tracker.pop();

        Ok(result)
    }

    fn bindings_to_definitions(&self, idents: &Vec<YulIdentifier>) -> Vec<Definition> {
        idents
            .iter()
            .map(|ident| Definition {
                identifier: ident.inner.clone(),
                location: self.here(),
                r#type: ident
                    .r#type
                    .as_ref()
                    .and_then(|t| Self::transpile_type(&t).ok()),
            })
            .collect()
    }
    fn bindings_to_references(&self, idents: &Vec<YulIdentifier>) -> Vec<Reference> {
        idents
            .iter()
            .map(|ident| Reference {
                identifier: ident.inner.clone(),
                location: self.here(),
            })
            .collect()
    }

    fn transpile_literal(lit: &YulLiteral) -> Literal {
        match &lit.inner {
            crate::yul::lexer::token::lexeme::literal::Literal::Boolean(b) => {
                Literal::Bool(match b {
                    crate::yul::lexer::token::lexeme::literal::boolean::Boolean::False => false,
                    crate::yul::lexer::token::lexeme::literal::boolean::Boolean::True => true,
                })
            }
            crate::yul::lexer::token::lexeme::literal::Literal::Integer(i) => match i {
                crate::yul::lexer::token::lexeme::literal::integer::Integer::Decimal { inner } => {
                    Literal::Int(IntegerLiteral::Decimal {
                        inner: inner.to_string(),
                    })
                }
                crate::yul::lexer::token::lexeme::literal::integer::Integer::Hexadecimal {
                    inner,
                } => Literal::Int(IntegerLiteral::Hexadecimal {
                    inner: inner.to_string(),
                }),
            },

            crate::yul::lexer::token::lexeme::literal::Literal::String(s) => {
                Literal::String(s.to_string())
            }
        }
    }

    fn transpile_expression_list(
        &mut self,
        list: &Vec<YulExpression>,
        ctx: &Context,
        ectx: &ExpressionTranslationContext,
    ) -> Result<(Vec<Expression>, ExpressionTranslationContext), Error> {
        let mut ectx: ExpressionTranslationContext = ectx.clone();
        let mut result: Vec<Expression> = Vec::new();

        for expr in list {
            match self.transpile_expression(expr, &ctx, &ectx)? {
                (e, new_ectx) => {
                    ectx = new_ectx;
                    result.push(e);
                }
            }
        }
        Ok((result, ectx))
    }

    fn get_module_definition(&self, ctx: &Context, name: &str) -> Option<ModuleDefinition> {
        let reference = Reference {
            identifier: name.to_owned(),
            location: self.here(),
        };
        ctx.module.definitions.get(&reference).cloned()
    }

    fn transpile_name(&mut self, ctx: &Context, name: &YulName) -> NameTranslationResult {
        match name {
            YulName::UserDefined(name_str) => match self.get_module_definition(ctx, name_str) {
                Some(ModuleDefinition::FunDef(_)) => {
                    NameTranslationResult::Function(FunctionName::UserDefined(name_str.clone()))
                }
                Some(ModuleDefinition::ProcDef(_)) => {
                    NameTranslationResult::Proc(ProcName::UserDefined(name_str.clone()))
                }
                None => NameTranslationResult::ProcOrFunction(name_str.clone()),
            },
            YulName::Add => NameTranslationResult::BinOp(BinaryOpType::Add),
            YulName::Sub => NameTranslationResult::BinOp(BinaryOpType::Sub),
            YulName::Mul => NameTranslationResult::BinOp(BinaryOpType::Mul),
            YulName::Div => NameTranslationResult::BinOp(BinaryOpType::Div),
            YulName::Mod => NameTranslationResult::BinOp(BinaryOpType::Mod),
            YulName::Exp => NameTranslationResult::BinOp(BinaryOpType::Exp),
            YulName::And => NameTranslationResult::BinOp(BinaryOpType::And),
            YulName::Shl => NameTranslationResult::BinOp(BinaryOpType::Shl),
            YulName::Shr => NameTranslationResult::BinOp(BinaryOpType::Shr),
            YulName::Sar => NameTranslationResult::Function(FunctionName::Sar),
            YulName::Eq => NameTranslationResult::BinOp(BinaryOpType::Eq),
            YulName::Or => NameTranslationResult::BinOp(BinaryOpType::Or),
            YulName::Xor => NameTranslationResult::BinOp(BinaryOpType::Xor),

            YulName::Smod => NameTranslationResult::Function(FunctionName::Smod),
            YulName::Sdiv => NameTranslationResult::Function(FunctionName::Sdiv),
            YulName::Lt => NameTranslationResult::Function(FunctionName::Lt),
            YulName::Gt => NameTranslationResult::Function(FunctionName::Gt),
            YulName::IsZero => NameTranslationResult::Function(FunctionName::IsZero),

            YulName::Slt => NameTranslationResult::Function(FunctionName::Slt),
            YulName::Sgt => NameTranslationResult::Function(FunctionName::Sgt),

            YulName::Not => NameTranslationResult::UnOp(UnaryOpType::Not),

            YulName::Byte => NameTranslationResult::Function(FunctionName::Byte),
            YulName::Pop => NameTranslationResult::Proc(ProcName::Pop),
            YulName::AddMod => NameTranslationResult::Function(FunctionName::AddMod),
            YulName::MulMod => NameTranslationResult::Function(FunctionName::MulMod),
            YulName::SignExtend => NameTranslationResult::Function(FunctionName::SignExtend),
            YulName::Keccak256 => NameTranslationResult::Proc(ProcName::Keccak256),
            YulName::MLoad => NameTranslationResult::Proc(ProcName::MLoad),
            YulName::MStore => NameTranslationResult::Proc(ProcName::MStore),
            YulName::MStore8 => NameTranslationResult::Proc(ProcName::MStore8),
            YulName::MCopy => NameTranslationResult::Proc(ProcName::MCopy),
            YulName::SLoad => NameTranslationResult::Proc(ProcName::SLoad),
            YulName::SStore => NameTranslationResult::Proc(ProcName::SStore),
            YulName::TLoad => NameTranslationResult::Proc(ProcName::TLoad),
            YulName::TStore => NameTranslationResult::Proc(ProcName::TStore),
            YulName::LoadImmutable => NameTranslationResult::Proc(ProcName::LoadImmutable),
            YulName::SetImmutable => NameTranslationResult::Proc(ProcName::SetImmutable),
            YulName::CallDataLoad => NameTranslationResult::Proc(ProcName::CallDataLoad),
            YulName::CallDataSize => NameTranslationResult::Proc(ProcName::CallDataSize),
            YulName::CallDataCopy => NameTranslationResult::Proc(ProcName::CallDataCopy),
            YulName::CodeSize => NameTranslationResult::Proc(ProcName::CodeSize),
            YulName::CodeCopy => NameTranslationResult::Proc(ProcName::CodeCopy),
            YulName::ExtCodeSize => NameTranslationResult::Proc(ProcName::ExtCodeSize),
            YulName::ExtCodeHash => NameTranslationResult::Proc(ProcName::ExtCodeHash),
            YulName::ReturnDataSize => NameTranslationResult::Proc(ProcName::ReturnDataSize),
            YulName::ReturnDataCopy => NameTranslationResult::Proc(ProcName::ReturnDataCopy),
            // YulName::Return => NameTranslationResult::Proc(ProcName::Return),
            // YulName::Revert => NameTranslationResult::Proc(ProcName::Revert),
            // YulName::Stop => NameTranslationResult::Proc(ProcName::Stop),
            // YulName::Invalid => NameTranslationResult::Proc(ProcName::Invalid),
            YulName::Log0 => NameTranslationResult::Proc(ProcName::Log0),
            YulName::Log1 => NameTranslationResult::Proc(ProcName::Log1),
            YulName::Log2 => NameTranslationResult::Proc(ProcName::Log2),
            YulName::Log3 => NameTranslationResult::Proc(ProcName::Log3),
            YulName::Log4 => NameTranslationResult::Proc(ProcName::Log4),
            YulName::Call => NameTranslationResult::Proc(ProcName::Call),
            YulName::CallCode => NameTranslationResult::Proc(ProcName::CallCode),
            YulName::DelegateCall => NameTranslationResult::Proc(ProcName::DelegateCall),
            YulName::StaticCall => NameTranslationResult::Proc(ProcName::StaticCall),
            YulName::Create => NameTranslationResult::Proc(ProcName::Create),
            YulName::Create2 => NameTranslationResult::Proc(ProcName::Create2),
            YulName::ZkCreate => NameTranslationResult::Proc(ProcName::ZkCreate),
            YulName::ZkCreate2 => NameTranslationResult::Proc(ProcName::ZkCreate2),
            YulName::DataSize => NameTranslationResult::Proc(ProcName::DataSize),
            YulName::DataCopy => NameTranslationResult::Proc(ProcName::DataCopy),
            YulName::DataOffset => NameTranslationResult::Proc(ProcName::DataOffset),
            YulName::LinkerSymbol => NameTranslationResult::Proc(ProcName::LinkerSymbol),
            YulName::MemoryGuard => NameTranslationResult::Proc(ProcName::MemoryGuard),
            YulName::Address => NameTranslationResult::Proc(ProcName::Address),
            YulName::Caller => NameTranslationResult::Proc(ProcName::Caller),
            YulName::CallValue => NameTranslationResult::Proc(ProcName::CallValue),
            YulName::Gas => NameTranslationResult::Proc(ProcName::Gas),
            YulName::Balance => NameTranslationResult::Proc(ProcName::Balance),
            YulName::SelfBalance => NameTranslationResult::Proc(ProcName::SelfBalance),
            YulName::GasLimit => NameTranslationResult::Proc(ProcName::GasLimit),
            YulName::GasPrice => NameTranslationResult::Proc(ProcName::GasPrice),
            YulName::Origin => NameTranslationResult::Proc(ProcName::Origin),
            YulName::ChainId => NameTranslationResult::Proc(ProcName::ChainId),
            YulName::Number => NameTranslationResult::Proc(ProcName::Number),
            YulName::Timestamp => NameTranslationResult::Proc(ProcName::Timestamp),
            YulName::BlockHash => NameTranslationResult::Proc(ProcName::BlockHash),
            YulName::BlobHash => NameTranslationResult::Proc(ProcName::BlobHash),
            YulName::Difficulty => NameTranslationResult::Proc(ProcName::Difficulty),
            YulName::Prevrandao => NameTranslationResult::Proc(ProcName::Prevrandao),
            YulName::CoinBase => NameTranslationResult::Proc(ProcName::CoinBase),
            YulName::MSize => NameTranslationResult::Proc(ProcName::MSize),
            YulName::Verbatim {
                input_size,
                output_size,
            } => NameTranslationResult::Proc(ProcName::Verbatim {
                input_size: *input_size,
                output_size: *output_size,
            }),
            YulName::BaseFee => NameTranslationResult::Proc(ProcName::BaseFee),
            YulName::BlobBaseFee => NameTranslationResult::Proc(ProcName::BlobBaseFee),
            YulName::Pc => NameTranslationResult::Proc(ProcName::Pc),
            YulName::ExtCodeCopy => NameTranslationResult::Proc(ProcName::ExtCodeCopy),
            YulName::SelfDestruct => NameTranslationResult::Proc(ProcName::SelfDestruct),
            YulName::ZkToL1 => NameTranslationResult::Proc(ProcName::ZkToL1),
            YulName::ZkCodeSource => NameTranslationResult::Proc(ProcName::ZkCodeSource),
            YulName::ZkPrecompile => NameTranslationResult::Proc(ProcName::ZkPrecompile),
            YulName::ZkMeta => NameTranslationResult::Proc(ProcName::ZkMeta),
            YulName::ZkSetContextU128 => NameTranslationResult::Proc(ProcName::ZkSetContextU128),
            YulName::ZkSetPubdataPrice => NameTranslationResult::Proc(ProcName::ZkSetPubdataPrice),
            YulName::ZkIncrementTxCounter => {
                NameTranslationResult::Proc(ProcName::ZkIncrementTxCounter)
            }
            YulName::ZkEventInitialize => NameTranslationResult::Proc(ProcName::ZkEventInitialize),
            YulName::ZkEventWrite => NameTranslationResult::Proc(ProcName::ZkEventWrite),
            YulName::ZkMimicCall => NameTranslationResult::Proc(ProcName::ZkMimicCall),
            YulName::ZkSystemMimicCall => NameTranslationResult::Proc(ProcName::ZkSystemMimicCall),
            YulName::ZkMimicCallByRef => NameTranslationResult::Proc(ProcName::ZkMimicCallByRef),
            YulName::ZkSystemMimicCallByRef => {
                NameTranslationResult::Proc(ProcName::ZkSystemMimicCallByRef)
            }
            YulName::ZkRawCall => NameTranslationResult::Proc(ProcName::ZkRawCall),
            YulName::ZkRawCallByRef => NameTranslationResult::Proc(ProcName::ZkRawCallByRef),
            YulName::ZkSystemCall => NameTranslationResult::Proc(ProcName::ZkSystemCall),
            YulName::ZkSystemCallByRef => NameTranslationResult::Proc(ProcName::ZkSystemCallByRef),
            YulName::ZkStaticRawCall => NameTranslationResult::Proc(ProcName::ZkStaticRawCall),
            YulName::ZkStaticRawCallByRef => {
                NameTranslationResult::Proc(ProcName::ZkStaticRawCallByRef)
            }
            YulName::ZkStaticSystemCall => {
                NameTranslationResult::Proc(ProcName::ZkStaticSystemCall)
            }
            YulName::ZkStaticSystemCallByRef => {
                NameTranslationResult::Proc(ProcName::ZkStaticSystemCallByRef)
            }
            YulName::ZkDelegateRawCall => NameTranslationResult::Proc(ProcName::ZkDelegateRawCall),
            YulName::ZkDelegateRawCallByRef => {
                NameTranslationResult::Proc(ProcName::ZkDelegateRawCallByRef)
            }
            YulName::ZkDelegateSystemCall => {
                NameTranslationResult::Proc(ProcName::ZkDelegateSystemCall)
            }
            YulName::ZkDelegateSystemCallByRef => {
                NameTranslationResult::Proc(ProcName::ZkDelegateSystemCallByRef)
            }
            YulName::ZkLoadCalldataIntoActivePtr => {
                NameTranslationResult::Proc(ProcName::ZkLoadCalldataIntoActivePtr)
            }
            YulName::ZkLoadReturndataIntoActivePtr => {
                NameTranslationResult::Proc(ProcName::ZkLoadReturndataIntoActivePtr)
            }
            YulName::ZkPtrAddIntoActive => {
                NameTranslationResult::Proc(ProcName::ZkPtrAddIntoActive)
            }
            YulName::ZkPtrShrinkIntoActive => {
                NameTranslationResult::Proc(ProcName::ZkPtrShrinkIntoActive)
            }
            YulName::ZkPtrPackIntoActive => {
                NameTranslationResult::Proc(ProcName::ZkPtrPackIntoActive)
            }
            YulName::ZkMultiplicationHigh => {
                NameTranslationResult::Proc(ProcName::ZkMultiplicationHigh)
            }
            YulName::ZkGlobalLoad => NameTranslationResult::Proc(ProcName::ZkGlobalLoad),
            YulName::ZkGlobalExtraAbiData => {
                NameTranslationResult::Proc(ProcName::ZkGlobalExtraAbiData)
            }
            YulName::ZkGlobalStore => NameTranslationResult::Proc(ProcName::ZkGlobalStore),
            YulName::Return => NameTranslationResult::Special(YulSpecial::Return),
            YulName::Revert => NameTranslationResult::Special(YulSpecial::Revert),
            YulName::Stop => NameTranslationResult::Special(YulSpecial::Stop),
            YulName::Invalid => NameTranslationResult::Special(YulSpecial::Invalid),
        }
    }

    fn transpile_function_call(
        &mut self,
        name: &YulName,
        yul_arguments: &Vec<YulExpression>,
        ctx: &Context,
        ectx: &ExpressionTranslationContext,
    ) -> Result<(Expression, ExpressionTranslationContext), Error> {
        match self.transpile_name(ctx, name) {
            NameTranslationResult::Function(target) => {
                let (arguments, ectx) =
                    self.transpile_expression_list(&yul_arguments, ctx, ectx)?;
                Ok((Expression::ECall(FunctionCall { target, arguments }), ectx))
            }

            NameTranslationResult::Proc(target) => {
                let (arguments, ctx) = self.transpile_expression_list(&yul_arguments, ctx, ectx)?;
                let definition = self.new_tmp_definition_here();
                let mut new_ctx = ctx;

                new_ctx.add_assignment(&definition, ProcCall { target, arguments });
                Ok((Expression::Reference(definition.reference()), new_ctx))
            }
            NameTranslationResult::ProcOrFunction(name) => {
                let (arguments, ctx) = self.transpile_expression_list(&yul_arguments, ctx, ectx)?;
                let definition = self.new_tmp_definition_here();
                let mut new_ctx = ctx;

                new_ctx.add_assignment(
                    &definition,
                    ProcCall {
                        target: ProcName::UserDefined(name.clone()),
                        arguments,
                    },
                );
                Ok((Expression::Reference(definition.reference()), new_ctx))
            }
            NameTranslationResult::BinOp(optype) => {
                let (arguments, ectx) =
                    self.transpile_expression_list(&yul_arguments, ctx, ectx)?;
                Ok((
                    Expression::Binary(
                        optype,
                        Box::new(arguments[0].clone()),
                        Box::new(arguments[1].clone()),
                    ),
                    ectx,
                ))
            }
            NameTranslationResult::UnOp(optype) => {
                let (arguments, ectx) =
                    self.transpile_expression_list(&yul_arguments, ctx, ectx)?;
                Ok((
                    Expression::Unary(
                        optype,
                        Box::new(arguments[0].clone()),
                    ),
                    ectx,
                ))
            },
            NameTranslationResult::Special(_) => todo!(),
        }
    }

    fn transpile_expression_root(
        &mut self,
        expr: &YulExpression,
        ctx: &Context,
    ) -> Result<(Expression, ExpressionTranslationContext), Error> {
        self.transpile_expression(expr, &ctx, &ExpressionTranslationContext::new())
    }
    fn transpile_expression(
        &mut self,
        expr: &YulExpression,
        ctx: &Context,
        ectx: &ExpressionTranslationContext,
    ) -> Result<(Expression, ExpressionTranslationContext), Error> {
        match expr {
            YulExpression::FunctionCall(YulFunctionCall {
                location: _,
                name,
                arguments,
            }) => self.transpile_function_call(name, arguments, ctx, ectx),

            YulExpression::Identifier(ident) =>
            // FIXME: visit identifier
            {
                Ok((
                    Expression::Reference(Reference {
                        identifier: ident.inner.clone(),
                        location: self.here(),
                    }),
                    ectx.clone(),
                ))
            }
            YulExpression::Literal(lit) => Ok((
                Expression::Literal(Self::transpile_literal(lit)),
                ectx.clone(),
            )),
        }
    }

    fn transpile_type(r#type: &YulType) -> Result<Type, Error> {
        Ok(match r#type {
            YulType::Bool => Type::Bool,
            YulType::Int(size) => Type::Int(size.clone()),
            YulType::UInt(size) => Type::UInt(size.clone()),
            YulType::Custom(custom) => Type::Custom(custom.clone()),
        })
    }

    const DEFAULT_TYPE: Type = Type::Int(32);

    fn transpile_identifier(&mut self, ident: &YulIdentifier) -> (Name, Type) {
        let new_type = match &ident.r#type {
            Some(typ) => Self::transpile_type(typ).unwrap(),
            None => Self::DEFAULT_TYPE,
        };

        (ident.inner.to_string(), new_type)
    }

    fn transpile_formal_parameter(&mut self, ident: &YulIdentifier) -> (Definition, Type) {
        let (name, typ) = self.transpile_identifier(ident);
        (self.new_definition_here(&name, Some(typ.clone())), typ)
    }

    /// Transforms `var x,y,... = expr` or `var x,y`.
    /// 1. Transforms expression `expr`. This may produce additional statements and new temporary locals, all of them should be added to the context `ctx`.
    /// 2. Add `x,y,...` to the list of locals in context `ctx`
    /// 3. Return an assignment, if there was an expression on the right hand side.
    fn transpile_variable_declaration(
        &mut self,
        vd: &VariableDeclaration,
        ctx: &Context,
    ) -> Result<(Context, TranslatedStatement), Error> {
        let VariableDeclaration {
            location: _,
            bindings,
            expression,
        } = vd;
        let definitions = self.bindings_to_definitions(&bindings);

        if let Some(rhs) = expression {
            let references = self.bindings_to_references(&bindings);
            let (
                new_rhs,
                ExpressionTranslationContext {
                    assignments,
                    locals,
                },
            ) = self.transpile_expression_root(&rhs, ctx)?;
            let translated_assignment = Statement::EAssignment(references, Box::new(new_rhs));
            let translated_statements = assignments
                .iter()
                .chain(iter::once(&translated_assignment))
                .cloned()
                .collect();
            Ok((
                ctx.add_locals(definitions.iter().chain(locals.iter())),
                TranslatedStatement::Statements(translated_statements),
            ))
        } else {
            Ok((
                ctx.add_locals(definitions.iter()),
                TranslatedStatement::Statements(vec![]),
            ))
        }
    }

    fn is_ec_function<'a>(
        body: &'a Vec<Statement>,
        result_vars: &Vec<Definition>,
    ) -> Option<&'a Expression> {
        if body.len() != 1 {
            None
        } else {
            match &body[0] {
                Statement::Expression(e) => Some(&e),
                Statement::EAssignment(lhs, rhs)
                    if lhs
                        == &result_vars
                            .iter()
                            .map(|d| d.reference())
                            .collect::<Vec<_>>() =>
                {
                    Some(&rhs)
                }
                _ => None,
            }
        }
    }

    fn transpile_function_definition(
        &mut self,
        fd: &FunctionDefinition,
        ctx: &Context,
    ) -> Result<(Context, FunctionTranslationResult), Error> {
        let FunctionDefinition {
            location: _,
            identifier,
            arguments,
            result,
            body,
            attributes: _,
        } = fd;
        //self.location_tracker.enter_function(identifier.clone());
        let formal_parameters = arguments
            .iter()
            .map(|ident| self.transpile_formal_parameter(ident))
            .collect();
        let (ctx, ec_block) = self.transpile_block(&body, ctx)?;
        let result_vars = self.bindings_to_definitions(&result);
        let return_type: Type = {
            let vec: Vec<Type> = result_vars
                .iter()
                .map(|d| d.r#type.clone().unwrap_or(Type::Unknown))
                .collect();
            match vec.len() {
                0 => Type::Unit,
                1 => vec[0].clone(),
                _ => Type::Tuple(vec),
            }
        };

        if let Some(body_expr) = Self::is_ec_function(&ec_block, &result_vars) {
            let signature = Signature {
                formal_parameters,
                return_type,
                kind: SignatureKind::Function,
            };
            //self.location_tracker.pop();
            Ok((
                ctx.clone(),
                FunctionTranslationResult::Function(Function {
                    name: FunctionName::UserDefined(identifier.clone()),
                    signature,
                    body: body_expr.clone(),
                    location: self.here(),
                }),
            ))
        } else {
            let signature = Signature {
                formal_parameters,
                return_type,
                kind: SignatureKind::Proc,
            };
            let statements = if signature.return_type != Type::Unit {
                let return_statement = Statement::Return(Expression::pack_tuple(
                    result_vars
                        .iter()
                        .map(|d| Expression::Reference(d.reference()))
                        .collect(),
                ));
                ec_block
                    .iter()
                    .chain(iter::once(&return_statement))
                    .cloned()
                    .collect()
            } else {
                ec_block
            };
            let locals = result_vars
                .iter()
                .chain(ctx.locals.iter())
                .cloned()
                .collect();
            //self.location_tracker.pop();
            Ok((
                ctx.clone(),
                FunctionTranslationResult::Proc(Proc {
                    name: ProcName::UserDefined(identifier.clone()),
                    signature,
                    locals,
                    body: Block { statements },
                    location: self.here(),
                }),
            ))
        }
    }

    fn transpile_statement(
        &mut self,
        stmt: &YulStatement,
        ctx: &Context,
    ) -> Result<(Context, TranslatedStatement), Error> {
        match stmt {
            YulStatement::Object(_) => todo!(),
            YulStatement::Code(_code) => todo!(),
            YulStatement::Block(block) => {
                let (new_ctx, stmts) = self.transpile_block(block, ctx)?;
                Ok((new_ctx, TranslatedStatement::Statements(stmts)))
            }
            YulStatement::Expression(expr) => {
                let (result, ectx) = self.transpile_expression_root(expr, ctx)?;
                Ok((
                    ctx.add_locals(&ectx.locals),
                    TranslatedStatement::Statements(
                        ectx.assignments
                            .iter()
                            .chain(iter::once(&Statement::Expression(result)))
                            .cloned()
                            .collect(),
                    ),
                ))
            }
            YulStatement::FunctionDefinition(fd) => {
                let (ctx, translation_result) = self.transpile_function_definition(fd, ctx)?;
                let mut new_ctx = Context::new();
                match translation_result {
                    FunctionTranslationResult::Function(fd) => {
                        new_ctx.module.add_def(ModuleDefinition::FunDef(fd))
                    }
                    FunctionTranslationResult::Proc(pd) => {
                        new_ctx.module.add_def(ModuleDefinition::ProcDef(pd))
                    }
                };
                new_ctx.merge(&ctx);
                Ok((new_ctx, TranslatedStatement::Statements(vec![])))
            }
            YulStatement::VariableDeclaration(vd) => self.transpile_variable_declaration(&vd, &ctx),
            YulStatement::Assignment(assignment) => self.transpile_assignment(&assignment, &ctx),
            YulStatement::IfConditional(_) => todo!(),
            YulStatement::Switch(_) => todo!(),
            YulStatement::ForLoop(_) => todo!(),
            YulStatement::Continue(_) => todo!(),
            YulStatement::Break(_) => todo!(),
            YulStatement::Leave(_) => todo!(),
        }
    }

    fn transpile_block(
        &mut self,
        block: &YulBlock,
        ctx: &Context,
    ) -> Result<(Context, TranslatedBlock), Error> {
        let mut context = ctx.clone();
        let mut result: TranslatedBlock = Vec::new();

        self.location_tracker.enter_block();
        for stmt in block.statements.iter() {
            let (ctx, translated) = self.transpile_statement(stmt, &context)?;
            context.merge(&ctx);
            match translated {
                TranslatedStatement::Statements(stmts) => result.extend(stmts),
                TranslatedStatement::Function(fd) => {
                    context.module.add_def(ModuleDefinition::FunDef(fd));
                }
                TranslatedStatement::Proc(proc) => {
                    context.module.add_def(ModuleDefinition::ProcDef(proc));
                }
            }
        }
        self.location_tracker.pop();
        Ok((context, result))
    }

    fn transpile_code(&mut self, code: &YulCode) -> Result<Module, Error> {
        self.location_tracker.enter_code();

        let (Context { module, locals }, statements) =
            self.transpile_block(&code.block, &Context::new())?;
        let default_code_proc_name = "BODY".to_string();

        let default_code_proc = Proc {
            name: ProcName::UserDefined(default_code_proc_name.clone()),
            signature: Signature::UNIT_TO_UNIT,
            body: Block { statements },
            locals,
            location: self.here(),
        };

        let mut new_module = module;

        if default_code_proc.body.statements.len() != 0 {
            new_module.merge(&Module {
                name: None,
                definitions: HashMap::from([(
                    Reference {
                        identifier: default_code_proc_name.clone(),
                        location: self.here(),
                    },
                    ModuleDefinition::ProcDef(default_code_proc),
                )]),
            });
        }

        self.location_tracker.pop();

        Ok(new_module)
    }

    fn new_definition_here(&self, name: &str, typ: Option<Type>) -> Definition {
        new_definition(self.here(), name, typ)
    }
    fn new_tmp_definition_here(&mut self) -> Definition {
        let name = format!("TMP{}", self.tmp_counter.get_value());
        self.tmp_counter.increment();
        new_definition(self.here(), &name, None)
    }

    fn transpile_assignment(
        &mut self,
        assignment: &YulAssignment,
        ctx: &Context,
    ) -> Result<(Context, TranslatedStatement), Error> {
        let YulAssignment {
            location: _,
            bindings,
            initializer,
        } = assignment;
        let references = self.bindings_to_references(&bindings);
        let (
            new_rhs,
            ExpressionTranslationContext {
                assignments,
                locals,
            },
        ) = self.transpile_expression_root(&initializer, ctx)?;
        let ec_assignment = Statement::EAssignment(references, Box::new(new_rhs));
        let ec_statements = assignments
            .iter()
            .chain(iter::once(&ec_assignment))
            .cloned()
            .collect();
        Ok((
            ctx.add_locals(locals.iter()),
            TranslatedStatement::Statements(ec_statements),
        ))
    }
}

#[derive(Clone)]
struct Context {
    // Completed definitions so far.
    module: Module,
    /// Used in inner functions.
    locals: Vec<Definition>,
}

impl Context {
    fn new() -> Context {
        Context {
            module: Module::new(None),
            locals: vec![],
        }
    }

    fn merge(&mut self, other: &Context) {
        self.module.merge(&other.module);
        self.locals.extend(other.locals.clone());
    }

    fn add_locals<'a, I>(&self, definitions: I) -> Self
    where
        I: IntoIterator<Item = &'a Definition>,
    {
        Self {
            module: self.module.clone(),
            locals: self
                .locals
                .iter()
                .cloned()
                .chain(definitions.into_iter().cloned())
                .collect(),
        }
    }
    fn add_local(&self, definition: Definition) -> Self {
        self.add_locals(iter::once(&definition))
    }
}
impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

enum TranslatedStatement {
    Statements(Vec<Statement>),
    Function(Function),
    Proc(Proc),
}

#[derive(Clone)]
struct ExpressionTranslationContext {
    assignments: Vec<Statement>,
    locals: Vec<Definition>,
}

impl ExpressionTranslationContext {
    fn new() -> Self {
        Self {
            assignments: vec![],
            locals: vec![],
        }
    }

    fn add_assignment(&mut self, new_definition: &Definition, rhs: ProcCall) {
        self.assignments.push(Statement::PAssignment(
            vec![new_definition.reference()],
            rhs,
        ));
        self.locals.push(new_definition.clone())
    }
}

impl Default for ExpressionTranslationContext {
    fn default() -> Self {
        Self::new()
    }
}

type TranslatedBlock = Vec<Statement>;

enum FunctionTranslationResult {
    Function(Function),
    Proc(Proc),
}

fn new_definition(location: Location, name: &str, r#type: Option<Type>) -> Definition {
    Definition {
        identifier: String::from(name),
        location,
        r#type: r#type,
    }
}
// In blocks we may create new definitions but they should be lifted to the parent function scope.
// Functions should be lifted to the top scope

enum YulSpecial {
    Return,
    Revert,
    Stop,
    Invalid,
}

enum NameTranslationResult {
    Function(FunctionName),
    Proc(ProcName),
    ProcOrFunction(Name),
    BinOp(BinaryOpType),
    UnOp(UnaryOpType),
    Special(YulSpecial),
}

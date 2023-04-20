//!
//! The function call subexpression.
//!

pub mod name;
pub mod verbatim;

use inkwell::values::BasicValue;
use num::ToPrimitive;

use crate::yul::error::Error;
use crate::yul::lexer::token::lexeme::symbol::Symbol;
use crate::yul::lexer::token::lexeme::Lexeme;
use crate::yul::lexer::token::location::Location;
use crate::yul::lexer::token::Token;
use crate::yul::lexer::Lexer;
use crate::yul::parser::error::Error as ParserError;
use crate::yul::parser::statement::expression::Expression;

use self::name::Name;

///
/// The Yul function call subexpression.
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionCall {
    /// The location.
    pub location: Location,
    /// The function name.
    pub name: Name,
    /// The function arguments expression list.
    pub arguments: Vec<Expression>,
}

impl FunctionCall {
    ///
    /// The element parser.
    ///
    pub fn parse(lexer: &mut Lexer, initial: Option<Token>) -> Result<Self, Error> {
        let token = crate::yul::parser::take_or_next(initial, lexer)?;

        let (location, name) = match token {
            Token {
                lexeme: Lexeme::Identifier(identifier),
                location,
                ..
            } => (location, Name::from(identifier.inner.as_str())),
            token => {
                return Err(ParserError::InvalidToken {
                    location: token.location,
                    expected: vec!["{identifier}"],
                    found: token.lexeme.to_string(),
                }
                .into());
            }
        };

        let mut arguments = Vec::new();
        loop {
            let argument = match lexer.next()? {
                Token {
                    lexeme: Lexeme::Symbol(Symbol::ParenthesisRight),
                    ..
                } => break,
                token => Expression::parse(lexer, Some(token))?,
            };

            arguments.push(argument);

            match lexer.peek()? {
                Token {
                    lexeme: Lexeme::Symbol(Symbol::Comma),
                    ..
                } => {
                    lexer.next()?;
                    continue;
                }
                Token {
                    lexeme: Lexeme::Symbol(Symbol::ParenthesisRight),
                    ..
                } => {
                    lexer.next()?;
                    break;
                }
                _ => break,
            }
        }

        Ok(Self {
            location,
            name,
            arguments,
        })
    }

    ///
    /// Converts the function call into an LLVM value.
    ///
    pub fn into_llvm<'ctx, D>(
        mut self,
        context: &mut compiler_llvm_context::Context<'ctx, D>,
    ) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
    where
        D: compiler_llvm_context::Dependency,
    {
        let location = self.location;

        match self.name {
            Name::UserDefined(name)
                if name
                    .starts_with(compiler_llvm_context::Function::ZKSYNC_NEAR_CALL_ABI_PREFIX)
                    && context.is_system_mode() =>
            {
                let mut values = Vec::with_capacity(self.arguments.len());
                for argument in self.arguments.into_iter() {
                    let value = argument.into_llvm(context)?.expect("Always exists").value;
                    values.push(value);
                }
                let function = context.get_function(name.as_str()).ok_or_else(|| {
                    anyhow::anyhow!("{} Undeclared function `{}`", location, name)
                })?;
                let r#return = function.borrow().r#return();

                if let compiler_llvm_context::FunctionReturn::Compound { pointer, .. } = r#return {
                    let pointer = context.build_alloca(
                        pointer.r#type,
                        format!("{name}_near_call_return_pointer_argument").as_str(),
                    );
                    context.build_store(pointer, pointer.r#type.const_zero());
                    values.insert(1, pointer.value.as_basic_value_enum());
                }

                let function_pointer = function
                    .borrow()
                    .declaration()
                    .value
                    .as_global_value()
                    .as_pointer_value();
                values.insert(0, function_pointer.as_basic_value_enum());

                let expected_arguments_count =
                    function.borrow().declaration().value.count_params() as usize;
                if expected_arguments_count != (values.len() - 2) {
                    anyhow::bail!(
                        "{} Function `{}` expected {} arguments, found {}",
                        location,
                        name,
                        expected_arguments_count,
                        values.len()
                    );
                }

                let return_value = context.build_invoke_near_call_abi(
                    function.borrow().declaration(),
                    values,
                    format!("{name}_near_call").as_str(),
                );

                if let compiler_llvm_context::FunctionReturn::Compound { pointer, .. } = r#return {
                    let pointer = compiler_llvm_context::Pointer::new(
                        pointer.r#type,
                        compiler_llvm_context::AddressSpace::Stack,
                        return_value.expect("Always exists").into_pointer_value(),
                    );
                    let return_value = context
                        .build_load(pointer, format!("{name}_near_call_return_value").as_str());
                    Ok(Some(return_value))
                } else {
                    Ok(return_value)
                }
            }
            Name::UserDefined(name) => {
                let mut values = Vec::with_capacity(self.arguments.len());
                for argument in self.arguments.into_iter() {
                    let value = argument.into_llvm(context)?.expect("Always exists").value;
                    values.push(value);
                }
                let function = context.get_function(name.as_str()).ok_or_else(|| {
                    anyhow::anyhow!("{} Undeclared function `{}`", location, name)
                })?;

                let expected_arguments_count =
                    function.borrow().declaration().value.count_params() as usize;
                if expected_arguments_count != values.len() {
                    anyhow::bail!(
                        "{} Function `{}` expected {} arguments, found {}",
                        location,
                        name,
                        expected_arguments_count,
                        values.len()
                    );
                }

                let return_value = context.build_invoke(
                    function.borrow().declaration(),
                    values.as_slice(),
                    format!("{name}_call").as_str(),
                );

                Ok(return_value)
            }

            Name::Add => {
                let arguments = self.pop_arguments_llvm::<D, 2>(context)?;
                compiler_llvm_context::arithmetic::addition(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            Name::Sub => {
                let arguments = self.pop_arguments_llvm::<D, 2>(context)?;
                compiler_llvm_context::arithmetic::subtraction(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            Name::Mul => {
                let arguments = self.pop_arguments_llvm::<D, 2>(context)?;
                compiler_llvm_context::arithmetic::multiplication(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            Name::Div => {
                let arguments = self.pop_arguments_llvm::<D, 2>(context)?;
                compiler_llvm_context::arithmetic::division(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            Name::Mod => {
                let arguments = self.pop_arguments_llvm::<D, 2>(context)?;
                compiler_llvm_context::arithmetic::remainder(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            Name::Sdiv => {
                let arguments = self.pop_arguments_llvm::<D, 2>(context)?;
                compiler_llvm_context::arithmetic::division_signed(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            Name::Smod => {
                let arguments = self.pop_arguments_llvm::<D, 2>(context)?;
                compiler_llvm_context::arithmetic::remainder_signed(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }

            Name::Lt => {
                let arguments = self.pop_arguments_llvm::<D, 2>(context)?;
                compiler_llvm_context::comparison::compare(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    inkwell::IntPredicate::ULT,
                )
                .map(Some)
            }
            Name::Gt => {
                let arguments = self.pop_arguments_llvm::<D, 2>(context)?;
                compiler_llvm_context::comparison::compare(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    inkwell::IntPredicate::UGT,
                )
                .map(Some)
            }
            Name::Eq => {
                let arguments = self.pop_arguments_llvm::<D, 2>(context)?;
                compiler_llvm_context::comparison::compare(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    inkwell::IntPredicate::EQ,
                )
                .map(Some)
            }
            Name::IsZero => {
                let arguments = self.pop_arguments_llvm::<D, 1>(context)?;
                compiler_llvm_context::comparison::compare(
                    context,
                    arguments[0].into_int_value(),
                    context.field_const(0),
                    inkwell::IntPredicate::EQ,
                )
                .map(Some)
            }
            Name::Slt => {
                let arguments = self.pop_arguments_llvm::<D, 2>(context)?;
                compiler_llvm_context::comparison::compare(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    inkwell::IntPredicate::SLT,
                )
                .map(Some)
            }
            Name::Sgt => {
                let arguments = self.pop_arguments_llvm::<D, 2>(context)?;
                compiler_llvm_context::comparison::compare(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    inkwell::IntPredicate::SGT,
                )
                .map(Some)
            }

            Name::Or => {
                let arguments = self.pop_arguments_llvm::<D, 2>(context)?;
                compiler_llvm_context::bitwise::or(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            Name::Xor => {
                let arguments = self.pop_arguments_llvm::<D, 2>(context)?;
                compiler_llvm_context::bitwise::xor(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            Name::Not => {
                let arguments = self.pop_arguments_llvm::<D, 1>(context)?;
                compiler_llvm_context::bitwise::xor(
                    context,
                    arguments[0].into_int_value(),
                    context.field_type().const_all_ones(),
                )
                .map(Some)
            }
            Name::And => {
                let arguments = self.pop_arguments_llvm::<D, 2>(context)?;
                compiler_llvm_context::bitwise::and(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            Name::Shl => {
                let arguments = self.pop_arguments_llvm::<D, 2>(context)?;
                compiler_llvm_context::bitwise::shift_left(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            Name::Shr => {
                let arguments = self.pop_arguments_llvm::<D, 2>(context)?;
                compiler_llvm_context::bitwise::shift_right(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            Name::Sar => {
                let arguments = self.pop_arguments_llvm::<D, 2>(context)?;
                compiler_llvm_context::bitwise::shift_right_arithmetic(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            Name::Byte => {
                let arguments = self.pop_arguments_llvm::<D, 2>(context)?;
                compiler_llvm_context::bitwise::byte(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            Name::Pop => {
                let _arguments = self.pop_arguments_llvm::<D, 1>(context)?;
                Ok(None)
            }

            Name::AddMod => {
                let arguments = self.pop_arguments_llvm::<D, 3>(context)?;
                compiler_llvm_context::math::add_mod(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    arguments[2].into_int_value(),
                )
                .map(Some)
            }
            Name::MulMod => {
                let arguments = self.pop_arguments_llvm::<D, 3>(context)?;
                compiler_llvm_context::math::mul_mod(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    arguments[2].into_int_value(),
                )
                .map(Some)
            }
            Name::Exp => {
                let arguments = self.pop_arguments_llvm::<D, 2>(context)?;
                compiler_llvm_context::math::exponent(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            Name::SignExtend => {
                let arguments = self.pop_arguments_llvm::<D, 2>(context)?;
                compiler_llvm_context::math::sign_extend(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }

            Name::Keccak256 => {
                let arguments = self.pop_arguments_llvm::<D, 2>(context)?;
                let input_offset = arguments[0].into_int_value();
                let input_length = arguments[1].into_int_value();

                let function = compiler_llvm_context::Runtime::keccak256(context);
                Ok(context.build_invoke(
                    function,
                    &[
                        input_offset.as_basic_value_enum(),
                        input_length.as_basic_value_enum(),
                    ],
                    "keccak256_call",
                ))
            }

            Name::MLoad => {
                let arguments = self.pop_arguments_llvm::<D, 1>(context)?;
                compiler_llvm_context::memory::load(context, arguments[0].into_int_value())
                    .map(Some)
            }
            Name::MStore => {
                let arguments = self.pop_arguments_llvm::<D, 2>(context)?;
                compiler_llvm_context::memory::store(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(|_| None)
            }
            Name::MStore8 => {
                let arguments = self.pop_arguments_llvm::<D, 2>(context)?;
                compiler_llvm_context::memory::store_byte(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(|_| None)
            }

            Name::SLoad => {
                let arguments = self.pop_arguments_llvm::<D, 1>(context)?;
                compiler_llvm_context::storage::load(context, arguments[0].into_int_value())
                    .map(Some)
            }
            Name::SStore => {
                let arguments = self.pop_arguments_llvm::<D, 2>(context)?;
                compiler_llvm_context::storage::store(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(|_| None)
            }
            Name::LoadImmutable => {
                let mut arguments = self.pop_arguments::<D, 1>(context)?;
                let key = arguments[0].original.take().ok_or_else(|| {
                    anyhow::anyhow!("{} `load_immutable` literal is missing", location)
                })?;

                if key.as_str() == "library_deploy_address" {
                    return Ok(context.build_call(
                        context.intrinsics().code_source,
                        &[],
                        "library_deploy_address",
                    ));
                }

                let offset = context
                    .solidity_mut()
                    .get_or_allocate_immutable(key.as_str());

                let index = context.field_const(offset as u64);

                compiler_llvm_context::immutable::load(context, index).map(Some)
            }
            Name::SetImmutable => {
                let mut arguments = self.pop_arguments::<D, 3>(context)?;
                let key = arguments[1].original.take().ok_or_else(|| {
                    anyhow::anyhow!("{} `load_immutable` literal is missing", location)
                })?;

                if key.as_str() == "library_deploy_address" {
                    return Ok(None);
                }

                let offset = context.solidity_mut().allocate_immutable(key.as_str());

                let index = context.field_const(offset as u64);
                let value = arguments[2].value.into_int_value();
                compiler_llvm_context::immutable::store(context, index, value).map(|_| None)
            }

            Name::CallDataLoad => {
                let arguments = self.pop_arguments_llvm::<D, 1>(context)?;

                match context
                    .code_type()
                    .ok_or_else(|| anyhow::anyhow!("The contract code part type is undefined"))?
                {
                    compiler_llvm_context::CodeType::Deploy => {
                        Ok(Some(context.field_const(0).as_basic_value_enum()))
                    }
                    compiler_llvm_context::CodeType::Runtime => {
                        compiler_llvm_context::calldata::load(
                            context,
                            arguments[0].into_int_value(),
                        )
                        .map(Some)
                    }
                }
            }
            Name::CallDataSize => {
                match context
                    .code_type()
                    .ok_or_else(|| anyhow::anyhow!("The contract code part type is undefined"))?
                {
                    compiler_llvm_context::CodeType::Deploy => {
                        Ok(Some(context.field_const(0).as_basic_value_enum()))
                    }
                    compiler_llvm_context::CodeType::Runtime => {
                        compiler_llvm_context::calldata::size(context).map(Some)
                    }
                }
            }
            Name::CallDataCopy => {
                let arguments = self.pop_arguments_llvm::<D, 3>(context)?;

                match context
                    .code_type()
                    .ok_or_else(|| anyhow::anyhow!("The contract code part type is undefined"))?
                {
                    compiler_llvm_context::CodeType::Deploy => {
                        let calldata_size = compiler_llvm_context::calldata::size(context)?;

                        compiler_llvm_context::calldata::copy(
                            context,
                            arguments[0].into_int_value(),
                            calldata_size.into_int_value(),
                            arguments[2].into_int_value(),
                        )
                        .map(|_| None)
                    }
                    compiler_llvm_context::CodeType::Runtime => {
                        compiler_llvm_context::calldata::copy(
                            context,
                            arguments[0].into_int_value(),
                            arguments[1].into_int_value(),
                            arguments[2].into_int_value(),
                        )
                        .map(|_| None)
                    }
                }
            }
            Name::CodeSize => {
                match context
                    .code_type()
                    .ok_or_else(|| anyhow::anyhow!("The contract code part type is undefined"))?
                {
                    compiler_llvm_context::CodeType::Deploy => {
                        compiler_llvm_context::calldata::size(context).map(Some)
                    }
                    compiler_llvm_context::CodeType::Runtime => {
                        let code_source =
                            compiler_llvm_context::zkevm_general::code_source(context)?;
                        compiler_llvm_context::ext_code::size(context, code_source.into_int_value())
                            .map(Some)
                    }
                }
            }
            Name::CodeCopy => {
                if let compiler_llvm_context::CodeType::Runtime = context
                    .code_type()
                    .ok_or_else(|| anyhow::anyhow!("The contract code part type is undefined"))?
                {
                    anyhow::bail!(
                        "{} The `CODECOPY` instruction is not supported in the runtime code",
                        location,
                    );
                }

                let arguments = self.pop_arguments_llvm::<D, 3>(context)?;
                compiler_llvm_context::calldata::copy(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    arguments[2].into_int_value(),
                )
                .map(|_| None)
            }
            Name::ReturnDataSize => compiler_llvm_context::return_data::size(context).map(Some),
            Name::ReturnDataCopy => {
                let arguments = self.pop_arguments_llvm::<D, 3>(context)?;
                compiler_llvm_context::return_data::copy(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    arguments[2].into_int_value(),
                )
                .map(|_| None)
            }
            Name::ExtCodeSize => {
                let arguments = self.pop_arguments_llvm::<D, 1>(context)?;
                compiler_llvm_context::ext_code::size(context, arguments[0].into_int_value())
                    .map(Some)
            }
            Name::ExtCodeHash => {
                let arguments = self.pop_arguments_llvm::<D, 1>(context)?;
                compiler_llvm_context::ext_code::hash(context, arguments[0].into_int_value())
                    .map(Some)
            }

            Name::Return => {
                let arguments = self.pop_arguments_llvm::<D, 2>(context)?;
                compiler_llvm_context::r#return::r#return(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(|_| None)
            }
            Name::Revert => {
                let arguments = self.pop_arguments_llvm::<D, 2>(context)?;
                compiler_llvm_context::r#return::revert(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(|_| None)
            }
            Name::Stop => compiler_llvm_context::r#return::stop(context).map(|_| None),
            Name::Invalid => compiler_llvm_context::r#return::invalid(context).map(|_| None),

            Name::Log0 => {
                let arguments = self.pop_arguments_llvm_log::<D, 2>(context)?;
                compiler_llvm_context::event::log(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    vec![],
                )
                .map(|_| None)
            }
            Name::Log1 => {
                let arguments = self.pop_arguments_llvm_log::<D, 3>(context)?;
                compiler_llvm_context::event::log(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    arguments[2..]
                        .iter()
                        .map(|argument| argument.into_int_value())
                        .collect(),
                )
                .map(|_| None)
            }
            Name::Log2 => {
                let arguments = self.pop_arguments_llvm_log::<D, 4>(context)?;
                compiler_llvm_context::event::log(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    arguments[2..]
                        .iter()
                        .map(|argument| argument.into_int_value())
                        .collect(),
                )
                .map(|_| None)
            }
            Name::Log3 => {
                let arguments = self.pop_arguments_llvm_log::<D, 5>(context)?;
                compiler_llvm_context::event::log(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    arguments[2..]
                        .iter()
                        .map(|argument| argument.into_int_value())
                        .collect(),
                )
                .map(|_| None)
            }
            Name::Log4 => {
                let arguments = self.pop_arguments_llvm_log::<D, 6>(context)?;
                compiler_llvm_context::event::log(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    arguments[2..]
                        .iter()
                        .map(|argument| argument.into_int_value())
                        .collect(),
                )
                .map(|_| None)
            }

            Name::Call => {
                let mut arguments = self.pop_arguments::<D, 7>(context)?;

                let gas = arguments[0].value.into_int_value();
                let address = arguments[1].value.into_int_value();
                let value = arguments[2].value.into_int_value();
                let input_offset = arguments[3].value.into_int_value();
                let input_size = arguments[4].value.into_int_value();
                let output_offset = arguments[5].value.into_int_value();
                let output_size = arguments[6].value.into_int_value();

                let simulation_address = arguments[1]
                    .constant
                    .take()
                    .and_then(|value| value.to_u16());

                compiler_llvm_context::call::default(
                    context,
                    context.llvm_runtime().far_call,
                    gas,
                    address,
                    Some(value),
                    input_offset,
                    input_size,
                    output_offset,
                    output_size,
                    simulation_address,
                )
                .map(Some)
            }
            Name::StaticCall => {
                let mut arguments = self.pop_arguments::<D, 6>(context)?;

                let gas = arguments[0].value.into_int_value();
                let address = arguments[1].value.into_int_value();
                let input_offset = arguments[2].value.into_int_value();
                let input_size = arguments[3].value.into_int_value();
                let output_offset = arguments[4].value.into_int_value();
                let output_size = arguments[5].value.into_int_value();

                let simulation_address = arguments[1]
                    .constant
                    .take()
                    .and_then(|value| value.to_u16());

                compiler_llvm_context::call::default(
                    context,
                    context.llvm_runtime().static_call,
                    gas,
                    address,
                    None,
                    input_offset,
                    input_size,
                    output_offset,
                    output_size,
                    simulation_address,
                )
                .map(Some)
            }
            Name::DelegateCall => {
                let mut arguments = self.pop_arguments::<D, 6>(context)?;

                let gas = arguments[0].value.into_int_value();
                let address = arguments[1].value.into_int_value();
                let input_offset = arguments[2].value.into_int_value();
                let input_size = arguments[3].value.into_int_value();
                let output_offset = arguments[4].value.into_int_value();
                let output_size = arguments[5].value.into_int_value();

                let simulation_address = arguments[1]
                    .constant
                    .take()
                    .and_then(|value| value.to_u16());

                compiler_llvm_context::call::default(
                    context,
                    context.llvm_runtime().delegate_call,
                    gas,
                    address,
                    None,
                    input_offset,
                    input_size,
                    output_offset,
                    output_size,
                    simulation_address,
                )
                .map(Some)
            }

            Name::Create | Name::ZkCreate => {
                let arguments = self.pop_arguments_llvm::<D, 3>(context)?;

                let value = arguments[0].into_int_value();
                let input_offset = arguments[1].into_int_value();
                let input_length = arguments[2].into_int_value();

                compiler_llvm_context::create::create(context, value, input_offset, input_length)
                    .map(Some)
            }
            Name::Create2 | Name::ZkCreate2 => {
                let arguments = self.pop_arguments_llvm::<D, 4>(context)?;

                let value = arguments[0].into_int_value();
                let input_offset = arguments[1].into_int_value();
                let input_length = arguments[2].into_int_value();
                let salt = arguments[3].into_int_value();

                compiler_llvm_context::create::create2(
                    context,
                    value,
                    input_offset,
                    input_length,
                    Some(salt),
                )
                .map(Some)
            }
            Name::DataOffset => {
                let mut arguments = self.pop_arguments::<D, 1>(context)?;

                let identifier = arguments[0].original.take().ok_or_else(|| {
                    anyhow::anyhow!("{} `dataoffset` object identifier is missing", location)
                })?;

                compiler_llvm_context::create::contract_hash(context, identifier).map(Some)
            }
            Name::DataSize => {
                let mut arguments = self.pop_arguments::<D, 1>(context)?;

                let identifier = arguments[0].original.take().ok_or_else(|| {
                    anyhow::anyhow!("{} `dataoffset` object identifier is missing", location)
                })?;

                compiler_llvm_context::create::header_size(context, identifier).map(Some)
            }
            Name::DataCopy => {
                let arguments = self.pop_arguments_llvm::<D, 3>(context)?;
                let offset = context.builder().build_int_add(
                    arguments[0].into_int_value(),
                    context.field_const(
                        (compiler_common::BYTE_LENGTH_X32 + compiler_common::BYTE_LENGTH_FIELD)
                            as u64,
                    ),
                    "datacopy_contract_hash_offset",
                );
                compiler_llvm_context::memory::store(context, offset, arguments[1].into_int_value())
                    .map(|_| None)
            }

            Name::LinkerSymbol => {
                let mut arguments = self.pop_arguments::<D, 1>(context)?;
                let path = arguments[0].original.take().ok_or_else(|| {
                    anyhow::anyhow!("{} Linker symbol literal is missing", location)
                })?;

                Ok(Some(
                    context
                        .resolve_library(path.as_str())?
                        .as_basic_value_enum(),
                ))
            }
            Name::MemoryGuard => {
                let arguments = self.pop_arguments_llvm::<D, 1>(context)?;
                Ok(Some(arguments[0]))
            }

            Name::Address => Ok(context.build_call(context.intrinsics().address, &[], "address")),
            Name::Caller => Ok(context.build_call(context.intrinsics().caller, &[], "caller")),

            Name::CallValue => compiler_llvm_context::ether_gas::value(context).map(Some),
            Name::Gas => compiler_llvm_context::ether_gas::gas(context).map(Some),
            Name::Balance => {
                let arguments = self.pop_arguments_llvm::<D, 1>(context)?;

                let address = arguments[0].into_int_value();
                compiler_llvm_context::ether_gas::balance(context, address).map(Some)
            }
            Name::SelfBalance => {
                let address = context
                    .build_call(context.intrinsics().address, &[], "self_balance_address")
                    .expect("Always exists")
                    .into_int_value();

                compiler_llvm_context::ether_gas::balance(context, address).map(Some)
            }

            Name::GasLimit => compiler_llvm_context::contract_context::gas_limit(context).map(Some),
            Name::GasPrice => compiler_llvm_context::contract_context::gas_price(context).map(Some),
            Name::Origin => compiler_llvm_context::contract_context::origin(context).map(Some),
            Name::ChainId => compiler_llvm_context::contract_context::chain_id(context).map(Some),
            Name::Timestamp => {
                compiler_llvm_context::contract_context::block_timestamp(context).map(Some)
            }
            Name::Number => {
                compiler_llvm_context::contract_context::block_number(context).map(Some)
            }
            Name::BlockHash => {
                let arguments = self.pop_arguments_llvm::<D, 1>(context)?;
                let index = arguments[0].into_int_value();

                compiler_llvm_context::contract_context::block_hash(context, index).map(Some)
            }
            Name::Difficulty | Name::Prevrandao => {
                compiler_llvm_context::contract_context::difficulty(context).map(Some)
            }
            Name::CoinBase => compiler_llvm_context::contract_context::coinbase(context).map(Some),
            Name::BaseFee => compiler_llvm_context::contract_context::basefee(context).map(Some),
            Name::MSize => compiler_llvm_context::contract_context::msize(context).map(Some),

            Name::Verbatim {
                input_size,
                output_size,
            } => verbatim::verbatim(context, &mut self, input_size, output_size),

            Name::CallCode => {
                let _arguments = self.pop_arguments_llvm::<D, 7>(context)?;
                anyhow::bail!("{} The `CALLCODE` instruction is not supported", location)
            }
            Name::Pc => anyhow::bail!("{} The `PC` instruction is not supported", location),
            Name::ExtCodeCopy => {
                let _arguments = self.pop_arguments_llvm::<D, 4>(context)?;
                anyhow::bail!(
                    "{} The `EXTCODECOPY` instruction is not supported",
                    location
                )
            }
            Name::SelfDestruct => {
                let _arguments = self.pop_arguments_llvm::<D, 1>(context)?;
                anyhow::bail!(
                    "{} The `SELFDESTRUCT` instruction is not supported",
                    location
                )
            }

            Name::ZkToL1 => {
                let [is_first, in_0, in_1] = self.pop_arguments_llvm::<D, 3>(context)?;

                compiler_llvm_context::zkevm_general::to_l1(
                    context,
                    is_first.into_int_value(),
                    in_0.into_int_value(),
                    in_1.into_int_value(),
                )
                .map(Some)
            }
            Name::ZkCodeSource => {
                compiler_llvm_context::zkevm_general::code_source(context).map(Some)
            }
            Name::ZkPrecompile => {
                let [in_0, in_1] = self.pop_arguments_llvm::<D, 2>(context)?;

                compiler_llvm_context::zkevm_general::precompile(
                    context,
                    in_0.into_int_value(),
                    in_1.into_int_value(),
                )
                .map(Some)
            }
            Name::ZkMeta => compiler_llvm_context::zkevm_general::meta(context).map(Some),
            Name::ZkSetContextU128 => {
                let [value] = self.pop_arguments_llvm::<D, 1>(context)?;

                compiler_llvm_context::zkevm_general::set_context_value(
                    context,
                    value.into_int_value(),
                )
                .map(|_| None)
            }
            Name::ZkSetPubdataPrice => {
                let [value] = self.pop_arguments_llvm::<D, 1>(context)?;

                compiler_llvm_context::zkevm_general::set_pubdata_price(
                    context,
                    value.into_int_value(),
                )
                .map(|_| None)
            }
            Name::ZkIncrementTxCounter => {
                compiler_llvm_context::zkevm_general::increment_tx_counter(context).map(|_| None)
            }
            Name::ZkEventInitialize => {
                let [operand_1, operand_2] = self.pop_arguments_llvm::<D, 2>(context)?;

                compiler_llvm_context::zkevm_general::event(
                    context,
                    operand_1.into_int_value(),
                    operand_2.into_int_value(),
                    true,
                )
                .map(|_| None)
            }
            Name::ZkEventWrite => {
                let [operand_1, operand_2] = self.pop_arguments_llvm::<D, 2>(context)?;

                compiler_llvm_context::zkevm_general::event(
                    context,
                    operand_1.into_int_value(),
                    operand_2.into_int_value(),
                    false,
                )
                .map(|_| None)
            }

            Name::ZkMimicCall => {
                let [address, abi_data, mimic] = self.pop_arguments_llvm::<D, 3>(context)?;

                compiler_llvm_context::zkevm_call::mimic(
                    context,
                    context.llvm_runtime().mimic_call,
                    address.into_int_value(),
                    mimic.into_int_value(),
                    abi_data.as_basic_value_enum(),
                    vec![],
                )
                .map(Some)
            }
            Name::ZkSystemMimicCall => {
                let [address, abi_data, mimic, extra_value_1, extra_value_2] =
                    self.pop_arguments_llvm::<D, 5>(context)?;

                compiler_llvm_context::zkevm_call::mimic(
                    context,
                    context.llvm_runtime().mimic_call,
                    address.into_int_value(),
                    mimic.into_int_value(),
                    abi_data.as_basic_value_enum(),
                    vec![
                        extra_value_1.into_int_value(),
                        extra_value_2.into_int_value(),
                    ],
                )
                .map(Some)
            }
            Name::ZkMimicCallByRef => {
                let [address, mimic] = self.pop_arguments_llvm::<D, 2>(context)?;
                let abi_data = context.get_global(compiler_llvm_context::GLOBAL_ACTIVE_POINTER)?;

                compiler_llvm_context::zkevm_call::mimic(
                    context,
                    context.llvm_runtime().mimic_call_byref,
                    address.into_int_value(),
                    mimic.into_int_value(),
                    abi_data,
                    vec![],
                )
                .map(Some)
            }
            Name::ZkSystemMimicCallByRef => {
                let [address, mimic, extra_value_1, extra_value_2] =
                    self.pop_arguments_llvm::<D, 4>(context)?;
                let abi_data = context.get_global(compiler_llvm_context::GLOBAL_ACTIVE_POINTER)?;

                compiler_llvm_context::zkevm_call::mimic(
                    context,
                    context.llvm_runtime().mimic_call_byref,
                    address.into_int_value(),
                    mimic.into_int_value(),
                    abi_data,
                    vec![
                        extra_value_1.into_int_value(),
                        extra_value_2.into_int_value(),
                    ],
                )
                .map(Some)
            }
            Name::ZkRawCall => {
                let [address, abi_data, output_offset, output_length] =
                    self.pop_arguments_llvm::<D, 4>(context)?;

                compiler_llvm_context::zkevm_call::raw_far(
                    context,
                    context.llvm_runtime().far_call,
                    address.into_int_value(),
                    abi_data.as_basic_value_enum(),
                    output_offset.into_int_value(),
                    output_length.into_int_value(),
                )
                .map(Some)
            }
            Name::ZkRawCallByRef => {
                let [address, output_offset, output_length] =
                    self.pop_arguments_llvm::<D, 3>(context)?;
                let abi_data = context.get_global(compiler_llvm_context::GLOBAL_ACTIVE_POINTER)?;

                compiler_llvm_context::zkevm_call::raw_far(
                    context,
                    context.llvm_runtime().far_call_byref,
                    address.into_int_value(),
                    abi_data,
                    output_offset.into_int_value(),
                    output_length.into_int_value(),
                )
                .map(Some)
            }
            Name::ZkSystemCall => {
                let [address, abi_data, extra_value_1, extra_value_2, extra_value_3, extra_value_4] =
                    self.pop_arguments_llvm::<D, 6>(context)?;

                compiler_llvm_context::zkevm_call::system(
                    context,
                    context.llvm_runtime().far_call,
                    address.into_int_value(),
                    abi_data,
                    context.field_const(0),
                    context.field_const(0),
                    vec![
                        extra_value_1.into_int_value(),
                        extra_value_2.into_int_value(),
                        extra_value_3.into_int_value(),
                        extra_value_4.into_int_value(),
                    ],
                )
                .map(Some)
            }
            Name::ZkSystemCallByRef => {
                let [address, extra_value_1, extra_value_2, extra_value_3, extra_value_4] =
                    self.pop_arguments_llvm::<D, 5>(context)?;
                let abi_data = context.get_global(compiler_llvm_context::GLOBAL_ACTIVE_POINTER)?;

                compiler_llvm_context::zkevm_call::system(
                    context,
                    context.llvm_runtime().far_call_byref,
                    address.into_int_value(),
                    abi_data,
                    context.field_const(0),
                    context.field_const(0),
                    vec![
                        extra_value_1.into_int_value(),
                        extra_value_2.into_int_value(),
                        extra_value_3.into_int_value(),
                        extra_value_4.into_int_value(),
                    ],
                )
                .map(Some)
            }
            Name::ZkStaticRawCall => {
                let [address, abi_data, output_offset, output_length] =
                    self.pop_arguments_llvm::<D, 4>(context)?;

                compiler_llvm_context::zkevm_call::raw_far(
                    context,
                    context.llvm_runtime().static_call,
                    address.into_int_value(),
                    abi_data.as_basic_value_enum(),
                    output_offset.into_int_value(),
                    output_length.into_int_value(),
                )
                .map(Some)
            }
            Name::ZkStaticRawCallByRef => {
                let [address, output_offset, output_length] =
                    self.pop_arguments_llvm::<D, 3>(context)?;
                let abi_data = context.get_global(compiler_llvm_context::GLOBAL_ACTIVE_POINTER)?;

                compiler_llvm_context::zkevm_call::raw_far(
                    context,
                    context.llvm_runtime().static_call_byref,
                    address.into_int_value(),
                    abi_data,
                    output_offset.into_int_value(),
                    output_length.into_int_value(),
                )
                .map(Some)
            }
            Name::ZkStaticSystemCall => {
                let [address, abi_data, extra_value_1, extra_value_2, extra_value_3, extra_value_4] =
                    self.pop_arguments_llvm::<D, 6>(context)?;

                compiler_llvm_context::zkevm_call::system(
                    context,
                    context.llvm_runtime().static_call,
                    address.into_int_value(),
                    abi_data,
                    context.field_const(0),
                    context.field_const(0),
                    vec![
                        extra_value_1.into_int_value(),
                        extra_value_2.into_int_value(),
                        extra_value_3.into_int_value(),
                        extra_value_4.into_int_value(),
                    ],
                )
                .map(Some)
            }
            Name::ZkStaticSystemCallByRef => {
                let [address, extra_value_1, extra_value_2, extra_value_3, extra_value_4] =
                    self.pop_arguments_llvm::<D, 5>(context)?;
                let abi_data = context.get_global(compiler_llvm_context::GLOBAL_ACTIVE_POINTER)?;

                compiler_llvm_context::zkevm_call::system(
                    context,
                    context.llvm_runtime().static_call_byref,
                    address.into_int_value(),
                    abi_data,
                    context.field_const(0),
                    context.field_const(0),
                    vec![
                        extra_value_1.into_int_value(),
                        extra_value_2.into_int_value(),
                        extra_value_3.into_int_value(),
                        extra_value_4.into_int_value(),
                    ],
                )
                .map(Some)
            }
            Name::ZkDelegateRawCall => {
                let [address, abi_data, output_offset, output_length] =
                    self.pop_arguments_llvm::<D, 4>(context)?;

                compiler_llvm_context::zkevm_call::raw_far(
                    context,
                    context.llvm_runtime().delegate_call,
                    address.into_int_value(),
                    abi_data.as_basic_value_enum(),
                    output_offset.into_int_value(),
                    output_length.into_int_value(),
                )
                .map(Some)
            }
            Name::ZkDelegateRawCallByRef => {
                let [address, output_offset, output_length] =
                    self.pop_arguments_llvm::<D, 3>(context)?;
                let abi_data = context.get_global(compiler_llvm_context::GLOBAL_ACTIVE_POINTER)?;

                compiler_llvm_context::zkevm_call::raw_far(
                    context,
                    context.llvm_runtime().delegate_call_byref,
                    address.into_int_value(),
                    abi_data,
                    output_offset.into_int_value(),
                    output_length.into_int_value(),
                )
                .map(Some)
            }
            Name::ZkDelegateSystemCall => {
                let [address, abi_data, extra_value_1, extra_value_2, extra_value_3, extra_value_4] =
                    self.pop_arguments_llvm::<D, 6>(context)?;

                compiler_llvm_context::zkevm_call::system(
                    context,
                    context.llvm_runtime().delegate_call,
                    address.into_int_value(),
                    abi_data,
                    context.field_const(0),
                    context.field_const(0),
                    vec![
                        extra_value_1.into_int_value(),
                        extra_value_2.into_int_value(),
                        extra_value_3.into_int_value(),
                        extra_value_4.into_int_value(),
                    ],
                )
                .map(Some)
            }
            Name::ZkDelegateSystemCallByRef => {
                let [address, extra_value_1, extra_value_2, extra_value_3, extra_value_4] =
                    self.pop_arguments_llvm::<D, 5>(context)?;
                let abi_data = context.get_global(compiler_llvm_context::GLOBAL_ACTIVE_POINTER)?;

                compiler_llvm_context::zkevm_call::system(
                    context,
                    context.llvm_runtime().delegate_call_byref,
                    address.into_int_value(),
                    abi_data,
                    context.field_const(0),
                    context.field_const(0),
                    vec![
                        extra_value_1.into_int_value(),
                        extra_value_2.into_int_value(),
                        extra_value_3.into_int_value(),
                        extra_value_4.into_int_value(),
                    ],
                )
                .map(Some)
            }

            Name::ZkLoadCalldataIntoActivePtr => {
                compiler_llvm_context::zkevm_abi::calldata_ptr_to_active(context).map(|_| None)
            }
            Name::ZkLoadReturndataIntoActivePtr => {
                compiler_llvm_context::zkevm_abi::return_data_ptr_to_active(context).map(|_| None)
            }
            Name::ZkPtrAddIntoActive => {
                let [offset] = self.pop_arguments_llvm::<D, 1>(context)?;

                compiler_llvm_context::zkevm_abi::active_ptr_add_assign(
                    context,
                    offset.into_int_value(),
                )
                .map(|_| None)
            }
            Name::ZkPtrShrinkIntoActive => {
                let [offset] = self.pop_arguments_llvm::<D, 1>(context)?;

                compiler_llvm_context::zkevm_abi::active_ptr_shrink_assign(
                    context,
                    offset.into_int_value(),
                )
                .map(|_| None)
            }
            Name::ZkPtrPackIntoActive => {
                let [data] = self.pop_arguments_llvm::<D, 1>(context)?;

                compiler_llvm_context::zkevm_abi::active_ptr_pack_assign(
                    context,
                    data.into_int_value(),
                )
                .map(|_| None)
            }

            Name::ZkMultiplicationHigh => {
                let [operand_1, operand_2] = self.pop_arguments_llvm::<D, 2>(context)?;

                compiler_llvm_context::zkevm_math::multiplication_512(
                    context,
                    operand_1.into_int_value(),
                    operand_2.into_int_value(),
                )
                .map(Some)
            }

            Name::ZkGlobalLoad => {
                let [mut key] = self.pop_arguments::<D, 1>(context)?;
                let key = key.original.take().ok_or_else(|| {
                    anyhow::anyhow!("{} `$zk_global_load` literal is missing", location)
                })?;

                context.get_global(key.as_str()).map(Some)
            }
            Name::ZkGlobalExtraAbiData => {
                let [index] = self.pop_arguments_llvm::<D, 1>(context)?;

                compiler_llvm_context::zkevm_abi::get_extra_abi_data(
                    context,
                    index.into_int_value(),
                )
                .map(Some)
            }
            Name::ZkGlobalStore => {
                let [mut key, value] = self.pop_arguments::<D, 2>(context)?;
                let key = key.original.take().ok_or_else(|| {
                    anyhow::anyhow!("{} `$zk_global_store` literal is missing", location)
                })?;
                let value = value.value.into_int_value();

                context.set_global(key.as_str(), context.field_type(), value);
                Ok(None)
            }
        }
    }

    ///
    /// Pops the specified number of arguments, converted into their LLVM values.
    ///
    fn pop_arguments_llvm<'ctx, D, const N: usize>(
        &mut self,
        context: &mut compiler_llvm_context::Context<'ctx, D>,
    ) -> anyhow::Result<[inkwell::values::BasicValueEnum<'ctx>; N]>
    where
        D: compiler_llvm_context::Dependency,
    {
        let mut arguments = Vec::with_capacity(N);
        for expression in self.arguments.drain(0..N) {
            arguments.push(expression.into_llvm(context)?.expect("Always exists").value);
        }

        Ok(arguments.try_into().expect("Always successful"))
    }

    ///
    /// Pops the specified number of arguments.
    ///
    fn pop_arguments<'ctx, D, const N: usize>(
        &mut self,
        context: &mut compiler_llvm_context::Context<'ctx, D>,
    ) -> anyhow::Result<[compiler_llvm_context::Argument<'ctx>; N]>
    where
        D: compiler_llvm_context::Dependency,
    {
        let mut arguments = Vec::with_capacity(N);
        for expression in self.arguments.drain(0..N) {
            arguments.push(expression.into_llvm(context)?.expect("Always exists"));
        }

        Ok(arguments.try_into().expect("Always successful"))
    }

    ///
    /// Pops the specified number of arguments, converted into their LLVM values.
    ///
    /// This function inverts the order of event topics, taking into account its behavior in EVM.
    ///
    fn pop_arguments_llvm_log<'ctx, D, const N: usize>(
        &mut self,
        context: &mut compiler_llvm_context::Context<'ctx, D>,
    ) -> anyhow::Result<[inkwell::values::BasicValueEnum<'ctx>; N]>
    where
        D: compiler_llvm_context::Dependency,
    {
        self.arguments[2..].reverse();
        let mut arguments = Vec::with_capacity(N);
        for expression in self.arguments.drain(0..N) {
            arguments.push(expression.into_llvm(context)?.expect("Always exists").value);
        }
        arguments[2..].reverse();

        Ok(arguments.try_into().expect("Always successful"))
    }
}

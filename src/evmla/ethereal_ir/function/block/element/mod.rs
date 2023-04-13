//!
//! The Ethereal IR block element.
//!

pub mod stack;

use inkwell::values::BasicValue;

use crate::evmla::assembly::instruction::codecopy;
use crate::evmla::assembly::instruction::name::Name as InstructionName;
use crate::evmla::assembly::instruction::Instruction;

use self::stack::Stack;

///
/// The Ethereal IR block element.
///
#[derive(Debug, Clone)]
pub struct Element {
    /// The Solidity compiler version.
    pub solc_version: semver::Version,
    /// The instruction.
    pub instruction: Instruction,
    /// The stack data.
    pub stack: Stack,
}

impl Element {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(solc_version: semver::Version, instruction: Instruction) -> Self {
        Self {
            solc_version,
            instruction,
            stack: Stack::new(),
        }
    }

    ///
    /// Pops the specified number of arguments, converted into their LLVM values.
    ///
    fn pop_arguments_llvm<'ctx, D>(
        &mut self,
        context: &mut compiler_llvm_context::Context<'ctx, D>,
    ) -> Vec<inkwell::values::BasicValueEnum<'ctx>>
    where
        D: compiler_llvm_context::Dependency,
    {
        let input_size = self.instruction.input_size(&context.evmla().version);
        let mut arguments = Vec::with_capacity(input_size);
        for index in 0..input_size {
            let pointer = context.evmla().stack
                [self.stack.elements.len() - self.instruction.output_size() - index - 1]
                .to_llvm()
                .into_pointer_value();
            let value = context.build_load(
                compiler_llvm_context::Pointer::new_stack_field(context, pointer),
                format!("argument_{index}").as_str(),
            );
            arguments.push(value);
        }
        arguments
    }

    ///
    /// Pops the specified number of arguments.
    ///
    fn pop_arguments<'ctx, D>(
        &mut self,
        context: &mut compiler_llvm_context::Context<'ctx, D>,
    ) -> Vec<compiler_llvm_context::Argument<'ctx>>
    where
        D: compiler_llvm_context::Dependency,
    {
        let input_size = self.instruction.input_size(&context.evmla().version);
        let mut arguments = Vec::with_capacity(input_size);
        for index in 0..input_size {
            let argument = context.evmla().stack
                [self.stack.elements.len() - self.instruction.output_size() - index - 1]
                .to_owned();
            arguments.push(argument);
        }
        arguments
    }
}

impl<D> compiler_llvm_context::WriteLLVM<D> for Element
where
    D: compiler_llvm_context::Dependency,
{
    fn into_llvm(
        mut self,
        context: &mut compiler_llvm_context::Context<'_, D>,
    ) -> anyhow::Result<()> {
        let input_size = self.instruction.input_size(&context.evmla().version);
        let mut original = self.instruction.value.clone();

        let value = match self.instruction.name {
            InstructionName::PUSH
            | InstructionName::PUSH1
            | InstructionName::PUSH2
            | InstructionName::PUSH3
            | InstructionName::PUSH4
            | InstructionName::PUSH5
            | InstructionName::PUSH6
            | InstructionName::PUSH7
            | InstructionName::PUSH8
            | InstructionName::PUSH9
            | InstructionName::PUSH10
            | InstructionName::PUSH11
            | InstructionName::PUSH12
            | InstructionName::PUSH13
            | InstructionName::PUSH14
            | InstructionName::PUSH15
            | InstructionName::PUSH16
            | InstructionName::PUSH17
            | InstructionName::PUSH18
            | InstructionName::PUSH19
            | InstructionName::PUSH20
            | InstructionName::PUSH21
            | InstructionName::PUSH22
            | InstructionName::PUSH23
            | InstructionName::PUSH24
            | InstructionName::PUSH25
            | InstructionName::PUSH26
            | InstructionName::PUSH27
            | InstructionName::PUSH28
            | InstructionName::PUSH29
            | InstructionName::PUSH30
            | InstructionName::PUSH31
            | InstructionName::PUSH32 => crate::evmla::assembly::instruction::stack::push(
                context,
                self.instruction
                    .value
                    .ok_or_else(|| anyhow::anyhow!("Instruction value missing"))?,
            )
            .map(Some),
            InstructionName::PUSH_Tag => crate::evmla::assembly::instruction::stack::push_tag(
                context,
                self.instruction
                    .value
                    .ok_or_else(|| anyhow::anyhow!("Instruction value missing"))?,
            )
            .map(Some),
            InstructionName::PUSH_ContractHash => compiler_llvm_context::create::contract_hash(
                context,
                self.instruction
                    .value
                    .ok_or_else(|| anyhow::anyhow!("Instruction value missing"))?,
            )
            .map(Some),
            InstructionName::PUSH_ContractHashSize => compiler_llvm_context::create::header_size(
                context,
                self.instruction
                    .value
                    .ok_or_else(|| anyhow::anyhow!("Instruction value missing"))?,
            )
            .map(Some),
            InstructionName::PUSHLIB => {
                let path = self
                    .instruction
                    .value
                    .ok_or_else(|| anyhow::anyhow!("Instruction value missing"))?;

                Ok(context
                    .resolve_library(path.as_str())?
                    .as_basic_value_enum())
                .map(Some)
            }
            InstructionName::PUSH_Data => {
                let value = self
                    .instruction
                    .value
                    .ok_or_else(|| anyhow::anyhow!("Instruction value missing"))?;

                if value.len() > compiler_common::BYTE_LENGTH_FIELD * 2 {
                    Ok(Some(context.field_const(0).as_basic_value_enum()))
                } else {
                    crate::evmla::assembly::instruction::stack::push(context, value).map(Some)
                }
            }
            InstructionName::PUSHDEPLOYADDRESS => Ok(context.build_call(
                context.intrinsics().code_source,
                &[],
                "contract_deploy_address",
            )),

            InstructionName::DUP1 => crate::evmla::assembly::instruction::stack::dup(
                context,
                1,
                self.stack.elements.len(),
                &mut original,
            )
            .map(Some),
            InstructionName::DUP2 => crate::evmla::assembly::instruction::stack::dup(
                context,
                2,
                self.stack.elements.len(),
                &mut original,
            )
            .map(Some),
            InstructionName::DUP3 => crate::evmla::assembly::instruction::stack::dup(
                context,
                3,
                self.stack.elements.len(),
                &mut original,
            )
            .map(Some),
            InstructionName::DUP4 => crate::evmla::assembly::instruction::stack::dup(
                context,
                4,
                self.stack.elements.len(),
                &mut original,
            )
            .map(Some),
            InstructionName::DUP5 => crate::evmla::assembly::instruction::stack::dup(
                context,
                5,
                self.stack.elements.len(),
                &mut original,
            )
            .map(Some),
            InstructionName::DUP6 => crate::evmla::assembly::instruction::stack::dup(
                context,
                6,
                self.stack.elements.len(),
                &mut original,
            )
            .map(Some),
            InstructionName::DUP7 => crate::evmla::assembly::instruction::stack::dup(
                context,
                7,
                self.stack.elements.len(),
                &mut original,
            )
            .map(Some),
            InstructionName::DUP8 => crate::evmla::assembly::instruction::stack::dup(
                context,
                8,
                self.stack.elements.len(),
                &mut original,
            )
            .map(Some),
            InstructionName::DUP9 => crate::evmla::assembly::instruction::stack::dup(
                context,
                9,
                self.stack.elements.len(),
                &mut original,
            )
            .map(Some),
            InstructionName::DUP10 => crate::evmla::assembly::instruction::stack::dup(
                context,
                10,
                self.stack.elements.len(),
                &mut original,
            )
            .map(Some),
            InstructionName::DUP11 => crate::evmla::assembly::instruction::stack::dup(
                context,
                11,
                self.stack.elements.len(),
                &mut original,
            )
            .map(Some),
            InstructionName::DUP12 => crate::evmla::assembly::instruction::stack::dup(
                context,
                12,
                self.stack.elements.len(),
                &mut original,
            )
            .map(Some),
            InstructionName::DUP13 => crate::evmla::assembly::instruction::stack::dup(
                context,
                13,
                self.stack.elements.len(),
                &mut original,
            )
            .map(Some),
            InstructionName::DUP14 => crate::evmla::assembly::instruction::stack::dup(
                context,
                14,
                self.stack.elements.len(),
                &mut original,
            )
            .map(Some),
            InstructionName::DUP15 => crate::evmla::assembly::instruction::stack::dup(
                context,
                15,
                self.stack.elements.len(),
                &mut original,
            )
            .map(Some),
            InstructionName::DUP16 => crate::evmla::assembly::instruction::stack::dup(
                context,
                16,
                self.stack.elements.len(),
                &mut original,
            )
            .map(Some),

            InstructionName::SWAP1 => crate::evmla::assembly::instruction::stack::swap(
                context,
                1,
                self.stack.elements.len(),
            )
            .map(|_| None),
            InstructionName::SWAP2 => crate::evmla::assembly::instruction::stack::swap(
                context,
                2,
                self.stack.elements.len(),
            )
            .map(|_| None),
            InstructionName::SWAP3 => crate::evmla::assembly::instruction::stack::swap(
                context,
                3,
                self.stack.elements.len(),
            )
            .map(|_| None),
            InstructionName::SWAP4 => crate::evmla::assembly::instruction::stack::swap(
                context,
                4,
                self.stack.elements.len(),
            )
            .map(|_| None),
            InstructionName::SWAP5 => crate::evmla::assembly::instruction::stack::swap(
                context,
                5,
                self.stack.elements.len(),
            )
            .map(|_| None),
            InstructionName::SWAP6 => crate::evmla::assembly::instruction::stack::swap(
                context,
                6,
                self.stack.elements.len(),
            )
            .map(|_| None),
            InstructionName::SWAP7 => crate::evmla::assembly::instruction::stack::swap(
                context,
                7,
                self.stack.elements.len(),
            )
            .map(|_| None),
            InstructionName::SWAP8 => crate::evmla::assembly::instruction::stack::swap(
                context,
                8,
                self.stack.elements.len(),
            )
            .map(|_| None),
            InstructionName::SWAP9 => crate::evmla::assembly::instruction::stack::swap(
                context,
                9,
                self.stack.elements.len(),
            )
            .map(|_| None),
            InstructionName::SWAP10 => crate::evmla::assembly::instruction::stack::swap(
                context,
                10,
                self.stack.elements.len(),
            )
            .map(|_| None),
            InstructionName::SWAP11 => crate::evmla::assembly::instruction::stack::swap(
                context,
                11,
                self.stack.elements.len(),
            )
            .map(|_| None),
            InstructionName::SWAP12 => crate::evmla::assembly::instruction::stack::swap(
                context,
                12,
                self.stack.elements.len(),
            )
            .map(|_| None),
            InstructionName::SWAP13 => crate::evmla::assembly::instruction::stack::swap(
                context,
                13,
                self.stack.elements.len(),
            )
            .map(|_| None),
            InstructionName::SWAP14 => crate::evmla::assembly::instruction::stack::swap(
                context,
                14,
                self.stack.elements.len(),
            )
            .map(|_| None),
            InstructionName::SWAP15 => crate::evmla::assembly::instruction::stack::swap(
                context,
                15,
                self.stack.elements.len(),
            )
            .map(|_| None),
            InstructionName::SWAP16 => crate::evmla::assembly::instruction::stack::swap(
                context,
                16,
                self.stack.elements.len(),
            )
            .map(|_| None),

            InstructionName::POP => {
                crate::evmla::assembly::instruction::stack::pop(context).map(|_| None)
            }

            InstructionName::Tag => {
                let destination: num::BigUint = self
                    .instruction
                    .value
                    .expect("Always exists")
                    .parse()
                    .expect("Always valid");

                crate::evmla::assembly::instruction::jump::unconditional(
                    context,
                    destination,
                    self.stack.hash(),
                )
                .map(|_| None)
            }
            InstructionName::JUMP => {
                let destination = self.stack.pop_tag()?;

                crate::evmla::assembly::instruction::jump::unconditional(
                    context,
                    destination,
                    self.stack.hash(),
                )
                .map(|_| None)
            }
            InstructionName::JUMPI => {
                let destination = self.stack.pop_tag()?;
                self.stack.pop()?;

                crate::evmla::assembly::instruction::jump::conditional(
                    context,
                    destination,
                    self.stack.hash(),
                    self.stack.elements.len(),
                )
                .map(|_| None)
            }
            InstructionName::JUMPDEST => Ok(None),

            InstructionName::ADD => {
                let arguments = self.pop_arguments_llvm(context);
                compiler_llvm_context::arithmetic::addition(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            InstructionName::SUB => {
                let arguments = self.pop_arguments_llvm(context);
                compiler_llvm_context::arithmetic::subtraction(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            InstructionName::MUL => {
                let arguments = self.pop_arguments_llvm(context);
                compiler_llvm_context::arithmetic::multiplication(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            InstructionName::DIV => {
                let arguments = self.pop_arguments_llvm(context);
                compiler_llvm_context::arithmetic::division(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            InstructionName::MOD => {
                let arguments = self.pop_arguments_llvm(context);
                compiler_llvm_context::arithmetic::remainder(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            InstructionName::SDIV => {
                let arguments = self.pop_arguments_llvm(context);
                compiler_llvm_context::arithmetic::division_signed(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            InstructionName::SMOD => {
                let arguments = self.pop_arguments_llvm(context);
                compiler_llvm_context::arithmetic::remainder_signed(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }

            InstructionName::LT => {
                let arguments = self.pop_arguments_llvm(context);
                compiler_llvm_context::comparison::compare(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    inkwell::IntPredicate::ULT,
                )
                .map(Some)
            }
            InstructionName::GT => {
                let arguments = self.pop_arguments_llvm(context);
                compiler_llvm_context::comparison::compare(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    inkwell::IntPredicate::UGT,
                )
                .map(Some)
            }
            InstructionName::EQ => {
                let arguments = self.pop_arguments_llvm(context);
                compiler_llvm_context::comparison::compare(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    inkwell::IntPredicate::EQ,
                )
                .map(Some)
            }
            InstructionName::ISZERO => {
                let arguments = self.pop_arguments_llvm(context);
                compiler_llvm_context::comparison::compare(
                    context,
                    arguments[0].into_int_value(),
                    context.field_const(0),
                    inkwell::IntPredicate::EQ,
                )
                .map(Some)
            }
            InstructionName::SLT => {
                let arguments = self.pop_arguments_llvm(context);
                compiler_llvm_context::comparison::compare(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    inkwell::IntPredicate::SLT,
                )
                .map(Some)
            }
            InstructionName::SGT => {
                let arguments = self.pop_arguments_llvm(context);
                compiler_llvm_context::comparison::compare(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    inkwell::IntPredicate::SGT,
                )
                .map(Some)
            }

            InstructionName::OR => {
                let arguments = self.pop_arguments_llvm(context);
                compiler_llvm_context::bitwise::or(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            InstructionName::XOR => {
                let arguments = self.pop_arguments_llvm(context);
                compiler_llvm_context::bitwise::xor(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            InstructionName::NOT => {
                let arguments = self.pop_arguments_llvm(context);
                compiler_llvm_context::bitwise::xor(
                    context,
                    arguments[0].into_int_value(),
                    context.field_type().const_all_ones(),
                )
                .map(Some)
            }
            InstructionName::AND => {
                let arguments = self.pop_arguments_llvm(context);
                compiler_llvm_context::bitwise::and(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            InstructionName::SHL => {
                let arguments = self.pop_arguments_llvm(context);
                compiler_llvm_context::bitwise::shift_left(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            InstructionName::SHR => {
                let arguments = self.pop_arguments_llvm(context);
                compiler_llvm_context::bitwise::shift_right(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            InstructionName::SAR => {
                let arguments = self.pop_arguments_llvm(context);
                compiler_llvm_context::bitwise::shift_right_arithmetic(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            InstructionName::BYTE => {
                let arguments = self.pop_arguments_llvm(context);
                compiler_llvm_context::bitwise::byte(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }

            InstructionName::ADDMOD => {
                let arguments = self.pop_arguments_llvm(context);
                compiler_llvm_context::math::add_mod(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    arguments[2].into_int_value(),
                )
                .map(Some)
            }
            InstructionName::MULMOD => {
                let arguments = self.pop_arguments_llvm(context);
                compiler_llvm_context::math::mul_mod(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    arguments[2].into_int_value(),
                )
                .map(Some)
            }
            InstructionName::EXP => {
                let arguments = self.pop_arguments_llvm(context);
                compiler_llvm_context::math::exponent(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }
            InstructionName::SIGNEXTEND => {
                let arguments = self.pop_arguments_llvm(context);
                compiler_llvm_context::math::sign_extend(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(Some)
            }

            InstructionName::SHA3 => {
                let arguments = self.pop_arguments_llvm(context);
                let input_offset = arguments[0].into_int_value();
                let input_length = arguments[1].into_int_value();

                let function = compiler_llvm_context::Runtime::keccak256(context);
                Ok(context.build_call(
                    function,
                    &[
                        input_offset.as_basic_value_enum(),
                        input_length.as_basic_value_enum(),
                    ],
                    "sha3_call",
                ))
            }
            InstructionName::KECCAK256 => {
                let arguments = self.pop_arguments_llvm(context);
                let input_offset = arguments[0].into_int_value();
                let input_length = arguments[1].into_int_value();

                let function = compiler_llvm_context::Runtime::keccak256(context);
                Ok(context.build_call(
                    function,
                    &[
                        input_offset.as_basic_value_enum(),
                        input_length.as_basic_value_enum(),
                    ],
                    "keccak256_call",
                ))
            }

            InstructionName::MLOAD => {
                let arguments = self.pop_arguments_llvm(context);
                compiler_llvm_context::memory::load(context, arguments[0].into_int_value())
                    .map(Some)
            }
            InstructionName::MSTORE => {
                let arguments = self.pop_arguments_llvm(context);
                compiler_llvm_context::memory::store(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(|_| None)
            }
            InstructionName::MSTORE8 => {
                let arguments = self.pop_arguments_llvm(context);
                compiler_llvm_context::memory::store_byte(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(|_| None)
            }

            InstructionName::SLOAD => {
                let arguments = self.pop_arguments_llvm(context);
                compiler_llvm_context::storage::load(context, arguments[0].into_int_value())
                    .map(Some)
            }
            InstructionName::SSTORE => {
                let arguments = self.pop_arguments_llvm(context);
                compiler_llvm_context::storage::store(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(|_| None)
            }
            InstructionName::PUSHIMMUTABLE => {
                let key = self
                    .instruction
                    .value
                    .ok_or_else(|| anyhow::anyhow!("Instruction value missing"))?;

                let offset = context
                    .solidity_mut()
                    .get_or_allocate_immutable(key.as_str());

                let index = context.field_const(offset as u64);
                compiler_llvm_context::immutable::load(context, index).map(Some)
            }
            InstructionName::ASSIGNIMMUTABLE => {
                let mut arguments = self.pop_arguments_llvm(context);

                let key = self
                    .instruction
                    .value
                    .ok_or_else(|| anyhow::anyhow!("Instruction value missing"))?;

                let offset = context.solidity_mut().allocate_immutable(key.as_str());

                let index = context.field_const(offset as u64);
                let value = arguments.pop().expect("Always exists").into_int_value();
                compiler_llvm_context::immutable::store(context, index, value).map(|_| None)
            }

            InstructionName::CALLDATALOAD => {
                let arguments = self.pop_arguments_llvm(context);
                compiler_llvm_context::calldata::load(context, arguments[0].into_int_value())
                    .map(Some)
            }
            InstructionName::CALLDATASIZE => {
                compiler_llvm_context::calldata::size(context).map(Some)
            }
            InstructionName::CALLDATACOPY => {
                let arguments = self.pop_arguments_llvm(context);
                compiler_llvm_context::calldata::copy(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    arguments[2].into_int_value(),
                )
                .map(|_| None)
            }
            InstructionName::CODESIZE => {
                match context
                    .code_type()
                    .ok_or_else(|| anyhow::anyhow!("The contract code part type is undefined"))?
                {
                    compiler_llvm_context::CodeType::Deploy => {
                        compiler_llvm_context::calldata::size(context).map(Some)
                    }
                    compiler_llvm_context::CodeType::Runtime => {
                        Ok(Some(context.field_const(0).as_basic_value_enum()))
                    }
                }
            }
            InstructionName::CODECOPY => {
                let mut arguments =
                    Vec::with_capacity(self.instruction.input_size(&self.solc_version));
                let arguments_with_original = self.pop_arguments(context);
                for (index, argument) in arguments_with_original.iter().enumerate() {
                    let pointer = argument.value.into_pointer_value();
                    let value = context.build_load(
                        compiler_llvm_context::Pointer::new_stack_field(context, pointer),
                        format!("argument_{index}").as_str(),
                    );
                    arguments.push(value);
                }

                let parent = context.module().get_name().to_str().expect("Always valid");

                let original_destination = arguments_with_original[0].original.as_deref();
                let original_source = arguments_with_original[1].original.as_deref();

                match original_source {
                    Some(source)
                        if !source.chars().all(|char| char.is_ascii_hexdigit())
                            && source != parent =>
                    {
                        codecopy::contract_hash(
                            context,
                            arguments[0].into_int_value(),
                            arguments[1].into_int_value(),
                        )
                    }
                    Some(source)
                        if !source.chars().all(|char| char.is_ascii_hexdigit())
                            && source == parent =>
                    {
                        match original_destination {
                            Some(length) if length.to_ascii_uppercase().as_str() == "B" => {
                                codecopy::library_marker(context, length, "73")
                            }
                            _ => Ok(()),
                        }
                    }
                    Some(source)
                        if source.chars().all(|char| char.is_ascii_hexdigit())
                            && source.len() >= compiler_common::BYTE_LENGTH_FIELD * 2 =>
                    {
                        codecopy::static_data(context, arguments[0].into_int_value(), source)
                    }
                    Some(_) | None => {
                        match context.code_type().ok_or_else(|| {
                            anyhow::anyhow!("The contract code part type is undefined")
                        })? {
                            compiler_llvm_context::CodeType::Deploy => {
                                compiler_llvm_context::calldata::copy(
                                    context,
                                    arguments[0].into_int_value(),
                                    arguments[1].into_int_value(),
                                    arguments[2].into_int_value(),
                                )
                            }
                            compiler_llvm_context::CodeType::Runtime => Ok(()),
                        }
                    }
                }
                .map(|_| None)
            }
            InstructionName::PUSHSIZE => Ok(Some(context.field_const(0).as_basic_value_enum())),
            InstructionName::RETURNDATASIZE => {
                compiler_llvm_context::return_data::size(context).map(Some)
            }
            InstructionName::RETURNDATACOPY => {
                let arguments = self.pop_arguments_llvm(context);
                compiler_llvm_context::return_data::copy(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                    arguments[2].into_int_value(),
                )
                .map(|_| None)
            }
            InstructionName::EXTCODESIZE => {
                let arguments = self.pop_arguments_llvm(context);
                compiler_llvm_context::ext_code::size(context, arguments[0].into_int_value())
                    .map(Some)
            }
            InstructionName::EXTCODEHASH => {
                let arguments = self.pop_arguments_llvm(context);
                compiler_llvm_context::ext_code::hash(context, arguments[0].into_int_value())
                    .map(Some)
            }

            InstructionName::RETURN => {
                let arguments = self.pop_arguments_llvm(context);
                compiler_llvm_context::r#return::r#return(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(|_| None)
            }
            InstructionName::REVERT => {
                let arguments = self.pop_arguments_llvm(context);
                compiler_llvm_context::r#return::revert(
                    context,
                    arguments[0].into_int_value(),
                    arguments[1].into_int_value(),
                )
                .map(|_| None)
            }
            InstructionName::STOP => compiler_llvm_context::r#return::stop(context).map(|_| None),
            InstructionName::INVALID => {
                compiler_llvm_context::r#return::invalid(context).map(|_| None)
            }

            InstructionName::LOG0 => {
                let mut arguments = self.pop_arguments_llvm(context);
                compiler_llvm_context::event::log(
                    context,
                    arguments.remove(0).into_int_value(),
                    arguments.remove(0).into_int_value(),
                    arguments
                        .into_iter()
                        .map(|argument| argument.into_int_value())
                        .collect(),
                )
                .map(|_| None)
            }
            InstructionName::LOG1 => {
                let mut arguments = self.pop_arguments_llvm(context);
                compiler_llvm_context::event::log(
                    context,
                    arguments.remove(0).into_int_value(),
                    arguments.remove(0).into_int_value(),
                    arguments
                        .into_iter()
                        .map(|argument| argument.into_int_value())
                        .collect(),
                )
                .map(|_| None)
            }
            InstructionName::LOG2 => {
                let mut arguments = self.pop_arguments_llvm(context);
                compiler_llvm_context::event::log(
                    context,
                    arguments.remove(0).into_int_value(),
                    arguments.remove(0).into_int_value(),
                    arguments
                        .into_iter()
                        .map(|argument| argument.into_int_value())
                        .collect(),
                )
                .map(|_| None)
            }
            InstructionName::LOG3 => {
                let mut arguments = self.pop_arguments_llvm(context);
                compiler_llvm_context::event::log(
                    context,
                    arguments.remove(0).into_int_value(),
                    arguments.remove(0).into_int_value(),
                    arguments
                        .into_iter()
                        .map(|argument| argument.into_int_value())
                        .collect(),
                )
                .map(|_| None)
            }
            InstructionName::LOG4 => {
                let mut arguments = self.pop_arguments_llvm(context);
                compiler_llvm_context::event::log(
                    context,
                    arguments.remove(0).into_int_value(),
                    arguments.remove(0).into_int_value(),
                    arguments
                        .into_iter()
                        .map(|argument| argument.into_int_value())
                        .collect(),
                )
                .map(|_| None)
            }

            InstructionName::CALL => {
                let mut arguments = self.pop_arguments_llvm(context);

                let gas = arguments.remove(0).into_int_value();
                let address = arguments.remove(0).into_int_value();
                let value = arguments.remove(0).into_int_value();
                let input_offset = arguments.remove(0).into_int_value();
                let input_size = arguments.remove(0).into_int_value();
                let output_offset = arguments.remove(0).into_int_value();
                let output_size = arguments.remove(0).into_int_value();

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
                    None,
                )
                .map(Some)
            }
            InstructionName::STATICCALL => {
                let mut arguments = self.pop_arguments_llvm(context);

                let gas = arguments.remove(0).into_int_value();
                let address = arguments.remove(0).into_int_value();
                let input_offset = arguments.remove(0).into_int_value();
                let input_size = arguments.remove(0).into_int_value();
                let output_offset = arguments.remove(0).into_int_value();
                let output_size = arguments.remove(0).into_int_value();

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
                    None,
                )
                .map(Some)
            }
            InstructionName::DELEGATECALL => {
                let mut arguments = self.pop_arguments_llvm(context);

                let gas = arguments.remove(0).into_int_value();
                let address = arguments.remove(0).into_int_value();
                let input_offset = arguments.remove(0).into_int_value();
                let input_size = arguments.remove(0).into_int_value();
                let output_offset = arguments.remove(0).into_int_value();
                let output_size = arguments.remove(0).into_int_value();

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
                    None,
                )
                .map(Some)
            }

            InstructionName::CREATE | InstructionName::ZK_CREATE => {
                let arguments = self.pop_arguments_llvm(context);

                let value = arguments[0].into_int_value();
                let input_offset = arguments[1].into_int_value();
                let input_length = arguments[2].into_int_value();

                compiler_llvm_context::create::create(context, value, input_offset, input_length)
                    .map(Some)
            }
            InstructionName::CREATE2 | InstructionName::ZK_CREATE2 => {
                let arguments = self.pop_arguments_llvm(context);

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

            InstructionName::ADDRESS => {
                Ok(context.build_call(context.intrinsics().address, &[], "address"))
            }
            InstructionName::CALLER => {
                Ok(context.build_call(context.intrinsics().caller, &[], "caller"))
            }

            InstructionName::CALLVALUE => {
                compiler_llvm_context::ether_gas::value(context).map(Some)
            }
            InstructionName::GAS => compiler_llvm_context::ether_gas::gas(context).map(Some),
            InstructionName::BALANCE => {
                let arguments = self.pop_arguments_llvm(context);

                let address = arguments[0].into_int_value();
                compiler_llvm_context::ether_gas::balance(context, address).map(Some)
            }
            InstructionName::SELFBALANCE => {
                let address = context
                    .build_call(context.intrinsics().address, &[], "self_balance_address")
                    .expect("Always exists")
                    .into_int_value();

                compiler_llvm_context::ether_gas::balance(context, address).map(Some)
            }

            InstructionName::GASLIMIT => {
                compiler_llvm_context::contract_context::gas_limit(context).map(Some)
            }
            InstructionName::GASPRICE => {
                compiler_llvm_context::contract_context::gas_price(context).map(Some)
            }
            InstructionName::ORIGIN => {
                compiler_llvm_context::contract_context::origin(context).map(Some)
            }
            InstructionName::CHAINID => {
                compiler_llvm_context::contract_context::chain_id(context).map(Some)
            }
            InstructionName::TIMESTAMP => {
                compiler_llvm_context::contract_context::block_timestamp(context).map(Some)
            }
            InstructionName::NUMBER => {
                compiler_llvm_context::contract_context::block_number(context).map(Some)
            }
            InstructionName::BLOCKHASH => {
                let arguments = self.pop_arguments_llvm(context);
                let index = arguments[0].into_int_value();

                compiler_llvm_context::contract_context::block_hash(context, index).map(Some)
            }
            InstructionName::DIFFICULTY | InstructionName::PREVRANDAO => {
                compiler_llvm_context::contract_context::difficulty(context).map(Some)
            }
            InstructionName::COINBASE => {
                compiler_llvm_context::contract_context::coinbase(context).map(Some)
            }
            InstructionName::BASEFEE => {
                compiler_llvm_context::contract_context::basefee(context).map(Some)
            }
            InstructionName::MSIZE => {
                compiler_llvm_context::contract_context::msize(context).map(Some)
            }

            name @ InstructionName::CALLCODE => {
                let mut _arguments = self.pop_arguments(context);
                anyhow::bail!("The `{}` instruction is not supported", name)
            }
            name @ InstructionName::PC => {
                anyhow::bail!("The `{}` instruction is not supported", name)
            }
            name @ InstructionName::EXTCODECOPY => {
                let _arguments = self.pop_arguments_llvm(context);
                anyhow::bail!("The `{}` instruction is not supported", name)
            }
            name @ InstructionName::SELFDESTRUCT => {
                let _arguments = self.pop_arguments_llvm(context);
                anyhow::bail!("The `{}` instruction is not supported", name)
            }
        }?;

        if let Some(value) = value {
            let pointer = context.evmla().stack[self.stack.elements.len() - input_size - 1]
                .to_llvm()
                .into_pointer_value();
            context.build_store(
                compiler_llvm_context::Pointer::new_stack_field(context, pointer),
                value,
            );
            context.evmla_mut().stack[self.stack.elements.len() - input_size - 1].original =
                original;
        }

        Ok(())
    }
}

impl std::fmt::Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let input_size = self.instruction.input_size(&self.solc_version);
        let output_size = self.instruction.output_size();

        let mut stack = self.stack.to_owned();
        let output = Stack::new_with_elements(
            stack
                .elements
                .drain(stack.elements.len() - output_size..)
                .collect(),
        );
        let input = Stack::new_with_elements(
            stack
                .elements
                .drain(stack.elements.len() - input_size..)
                .collect(),
        );

        write!(f, "{:88}{}", self.instruction.to_string(), stack)?;
        if input_size != 0 {
            write!(f, " - {input}")?;
        }
        if output_size != 0 {
            write!(f, " + {output}")?;
        }
        writeln!(f)?;

        Ok(())
    }
}

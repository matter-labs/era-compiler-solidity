//!
//! The Ethereal IR function.
//!

pub mod block;
pub mod queue_element;
pub mod visited_element;

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::BitAnd;

use inkwell::types::BasicType;
use inkwell::values::BasicValue;
use num::CheckedAdd;
use num::CheckedMul;
use num::CheckedSub;
use num::Num;
use num::ToPrimitive;
use num::Zero;

use crate::evmla::assembly::instruction::name::Name as InstructionName;
use crate::evmla::assembly::instruction::Instruction;
use crate::evmla::ethereal_ir::function::block::element::stack::element::Element;
use crate::evmla::ethereal_ir::function::block::element::stack::Stack;
use crate::evmla::ethereal_ir::EtherealIR;

use self::block::element::stack::element::Element as StackElement;
use self::block::element::Element as BlockElement;
use self::block::Block;
use self::queue_element::QueueElement;
use self::visited_element::VisitedElement;

///
/// The Ethereal IR function.
///
#[derive(Debug, Clone)]
pub struct Function {
    /// The Solidity compiler version.
    pub solc_version: semver::Version,
    /// The separately labelled blocks.
    pub blocks: BTreeMap<compiler_llvm_context::FunctionBlockKey, Vec<Block>>,
    /// The function stack size.
    pub stack_size: usize,
}

impl Function {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        solc_version: semver::Version,
        blocks: &HashMap<compiler_llvm_context::FunctionBlockKey, Block>,
        visited: &mut HashSet<VisitedElement>,
    ) -> anyhow::Result<Self> {
        let mut function = Self {
            solc_version,
            blocks: BTreeMap::new(),
            stack_size: 0,
        };
        function.consume_block(
            blocks,
            visited,
            QueueElement::new(
                compiler_llvm_context::FunctionBlockKey::new(
                    compiler_llvm_context::CodeType::Deploy,
                    num::BigUint::zero(),
                ),
                None,
                Stack::new(),
            ),
        )?;
        function.consume_block(
            blocks,
            visited,
            QueueElement::new(
                compiler_llvm_context::FunctionBlockKey::new(
                    compiler_llvm_context::CodeType::Runtime,
                    num::BigUint::zero(),
                ),
                None,
                Stack::new(),
            ),
        )?;
        Ok(function.finalize())
    }

    ///
    /// Consumes the entry or a conditional block attached to another one.
    ///
    fn consume_block(
        &mut self,
        blocks: &HashMap<compiler_llvm_context::FunctionBlockKey, Block>,
        visited: &mut HashSet<VisitedElement>,
        mut queue_element: QueueElement,
    ) -> anyhow::Result<()> {
        let version = self.solc_version.to_owned();

        let mut queue = vec![];

        let visited_element =
            VisitedElement::new(queue_element.block_key.clone(), queue_element.stack.hash());
        if visited.contains(&visited_element) {
            return Ok(());
        }
        visited.insert(visited_element);

        let mut block = blocks
            .get(&queue_element.block_key)
            .cloned()
            .ok_or_else(|| {
                anyhow::anyhow!("Undeclared destination block {}", queue_element.block_key)
            })?;
        block.initial_stack = queue_element.stack.clone();
        let block = self.find_compatible_block(block);
        let block = self.insert_block(block);
        block.stack = block.initial_stack.clone();
        if let Some(predecessor) = queue_element.predecessor.take() {
            block.insert_predecessor(predecessor);
        }

        let mut block_size = 0;
        for block_element in block.elements.iter_mut() {
            block_size += 1;

            if Self::handle_instruction(
                block.key.code_type,
                &mut block.stack,
                block_element,
                &version,
                &mut queue,
                &mut queue_element,
            )
            .is_err()
            {
                block_element.stack = block.stack.clone();
                block_element.instruction = Instruction::invalid();
                break;
            }
        }
        block.elements.truncate(block_size);

        for element in queue.into_iter() {
            self.consume_block(blocks, visited, element)?;
        }

        Ok(())
    }

    ///
    /// Processes an instruction, returning an error, if there is an invalid stack state.
    ///
    /// The blocks with an invalid stack state are considered being partially unreachable, and
    /// the invalid part is truncated after terminating with an `INVALID` instruction.
    ///
    fn handle_instruction(
        code_type: compiler_llvm_context::CodeType,
        block_stack: &mut Stack,
        block_element: &mut BlockElement,
        version: &semver::Version,
        queue: &mut Vec<QueueElement>,
        queue_element: &mut QueueElement,
    ) -> anyhow::Result<()> {
        match block_element.instruction {
            Instruction {
                name: InstructionName::PUSH_Tag,
                value: Some(ref tag),
            } => {
                let tag: num::BigUint = tag.parse().expect("Always valid");
                block_stack.push(Element::Tag(tag.bitand(num::BigUint::from(u64::MAX))));

                block_element.stack = block_stack.clone();
            }
            Instruction {
                name: InstructionName::JUMP,
                ..
            } => {
                queue_element.predecessor = Some(queue_element.block_key.clone());

                block_element.stack = block_stack.clone();
                let destination = block_stack.pop_tag()?;
                let block_key = if destination > num::BigUint::from(u32::MAX) {
                    compiler_llvm_context::FunctionBlockKey::new(
                        compiler_llvm_context::CodeType::Runtime,
                        destination - num::BigUint::from(1u64 << 32),
                    )
                } else {
                    compiler_llvm_context::FunctionBlockKey::new(code_type, destination)
                };
                queue.push(QueueElement::new(
                    block_key,
                    queue_element.predecessor.clone(),
                    block_stack.to_owned(),
                ));
            }
            Instruction {
                name: InstructionName::JUMPI,
                ..
            } => {
                queue_element.predecessor = Some(queue_element.block_key.clone());

                block_element.stack = block_stack.clone();
                let destination = block_stack.pop_tag()?;
                let block_key = if destination > num::BigUint::from(u32::MAX) {
                    compiler_llvm_context::FunctionBlockKey::new(
                        compiler_llvm_context::CodeType::Runtime,
                        destination - num::BigUint::from(1u64 << 32),
                    )
                } else {
                    compiler_llvm_context::FunctionBlockKey::new(code_type, destination)
                };
                block_stack.pop()?;
                queue.push(QueueElement::new(
                    block_key,
                    queue_element.predecessor.clone(),
                    block_stack.to_owned(),
                ));
            }
            Instruction {
                name: InstructionName::Tag,
                value: Some(ref tag),
            } => {
                block_element.stack = block_stack.clone();

                let tag: num::BigUint = tag.parse().expect("Always valid");
                let block_key = compiler_llvm_context::FunctionBlockKey::new(code_type, tag);

                queue_element.predecessor = Some(queue_element.block_key.clone());
                queue_element.block_key = block_key.clone();
                queue.push(QueueElement::new(
                    block_key,
                    queue_element.predecessor.clone(),
                    block_stack.to_owned(),
                ));
            }

            Instruction {
                name: InstructionName::SWAP1,
                ..
            } => {
                block_stack.swap(1)?;
                block_element.stack = block_stack.clone();
            }
            Instruction {
                name: InstructionName::SWAP2,
                ..
            } => {
                block_stack.swap(2)?;
                block_element.stack = block_stack.clone();
            }
            Instruction {
                name: InstructionName::SWAP3,
                ..
            } => {
                block_stack.swap(3)?;
                block_element.stack = block_stack.clone();
            }
            Instruction {
                name: InstructionName::SWAP4,
                ..
            } => {
                block_stack.swap(4)?;
                block_element.stack = block_stack.clone();
            }
            Instruction {
                name: InstructionName::SWAP5,
                ..
            } => {
                block_stack.swap(5)?;
                block_element.stack = block_stack.clone();
            }
            Instruction {
                name: InstructionName::SWAP6,
                ..
            } => {
                block_stack.swap(6)?;
                block_element.stack = block_stack.clone();
            }
            Instruction {
                name: InstructionName::SWAP7,
                ..
            } => {
                block_stack.swap(7)?;
                block_element.stack = block_stack.clone();
            }
            Instruction {
                name: InstructionName::SWAP8,
                ..
            } => {
                block_stack.swap(8)?;
                block_element.stack = block_stack.clone();
            }
            Instruction {
                name: InstructionName::SWAP9,
                ..
            } => {
                block_stack.swap(9)?;
                block_element.stack = block_stack.clone();
            }
            Instruction {
                name: InstructionName::SWAP10,
                ..
            } => {
                block_stack.swap(10)?;
                block_element.stack = block_stack.clone();
            }
            Instruction {
                name: InstructionName::SWAP11,
                ..
            } => {
                block_stack.swap(11)?;
                block_element.stack = block_stack.clone();
            }
            Instruction {
                name: InstructionName::SWAP12,
                ..
            } => {
                block_stack.swap(12)?;
                block_element.stack = block_stack.clone();
            }
            Instruction {
                name: InstructionName::SWAP13,
                ..
            } => {
                block_stack.swap(13)?;
                block_element.stack = block_stack.clone();
            }
            Instruction {
                name: InstructionName::SWAP14,
                ..
            } => {
                block_stack.swap(14)?;
                block_element.stack = block_stack.clone();
            }
            Instruction {
                name: InstructionName::SWAP15,
                ..
            } => {
                block_stack.swap(15)?;
                block_element.stack = block_stack.clone();
            }
            Instruction {
                name: InstructionName::SWAP16,
                ..
            } => {
                block_stack.swap(16)?;
                block_element.stack = block_stack.clone();
            }

            Instruction {
                name: InstructionName::DUP1,
                ..
            } => {
                block_stack.dup(1)?;
                block_element.stack = block_stack.clone();
            }
            Instruction {
                name: InstructionName::DUP2,
                ..
            } => {
                block_stack.dup(2)?;
                block_element.stack = block_stack.clone();
            }
            Instruction {
                name: InstructionName::DUP3,
                ..
            } => {
                block_stack.dup(3)?;
                block_element.stack = block_stack.clone();
            }
            Instruction {
                name: InstructionName::DUP4,
                ..
            } => {
                block_stack.dup(4)?;
                block_element.stack = block_stack.clone();
            }
            Instruction {
                name: InstructionName::DUP5,
                ..
            } => {
                block_stack.dup(5)?;
                block_element.stack = block_stack.clone();
            }
            Instruction {
                name: InstructionName::DUP6,
                ..
            } => {
                block_stack.dup(6)?;
                block_element.stack = block_stack.clone();
            }
            Instruction {
                name: InstructionName::DUP7,
                ..
            } => {
                block_stack.dup(7)?;
                block_element.stack = block_stack.clone();
            }
            Instruction {
                name: InstructionName::DUP8,
                ..
            } => {
                block_stack.dup(8)?;
                block_element.stack = block_stack.clone();
            }
            Instruction {
                name: InstructionName::DUP9,
                ..
            } => {
                block_stack.dup(9)?;
                block_element.stack = block_stack.clone();
            }
            Instruction {
                name: InstructionName::DUP10,
                ..
            } => {
                block_stack.dup(10)?;
                block_element.stack = block_stack.clone();
            }
            Instruction {
                name: InstructionName::DUP11,
                ..
            } => {
                block_stack.dup(11)?;
                block_element.stack = block_stack.clone();
            }
            Instruction {
                name: InstructionName::DUP12,
                ..
            } => {
                block_stack.dup(12)?;
                block_element.stack = block_stack.clone();
            }
            Instruction {
                name: InstructionName::DUP13,
                ..
            } => {
                block_stack.dup(13)?;
                block_element.stack = block_stack.clone();
            }
            Instruction {
                name: InstructionName::DUP14,
                ..
            } => {
                block_stack.dup(14)?;
                block_element.stack = block_stack.clone();
            }
            Instruction {
                name: InstructionName::DUP15,
                ..
            } => {
                block_stack.dup(15)?;
                block_element.stack = block_stack.clone();
            }
            Instruction {
                name: InstructionName::DUP16,
                ..
            } => {
                block_stack.dup(16)?;
                block_element.stack = block_stack.clone();
            }

            Instruction {
                name:
                    InstructionName::PUSH
                    | InstructionName::PUSH_Data
                    | InstructionName::PUSH_ContractHash
                    | InstructionName::PUSH_ContractHashSize
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
                    | InstructionName::PUSH32
                    | InstructionName::PUSHLIB
                    | InstructionName::PUSHDEPLOYADDRESS,
                value: Some(ref constant),
            } => {
                let element = match num::BigUint::from_str_radix(
                    constant.as_str(),
                    compiler_common::BASE_HEXADECIMAL,
                ) {
                    Ok(value) => StackElement::Constant(value),
                    Err(_error) => StackElement::Path(constant.to_owned()),
                };
                block_stack.push(element);
                block_element.stack = block_stack.clone();
            }

            ref instruction @ Instruction {
                name: InstructionName::ADD,
                ..
            } => {
                let operand_2 = block_stack.elements.get(block_stack.elements.len() - 2);
                let operand_1 = block_stack.elements.last();

                let result = match (operand_1, operand_2) {
                    (Some(Element::Constant(operand_1)), Some(Element::Constant(operand_2))) => {
                        match operand_1.checked_add(operand_2) {
                            Some(result) => Element::Constant(result),
                            None => Element::Value,
                        }
                    }
                    _ => Element::Value,
                };

                block_stack.push(result);
                block_element.stack = block_stack.clone();
                let output = block_stack.pop()?;
                for _ in 0..instruction.input_size(version) {
                    block_stack.pop()?;
                }
                block_stack.push(output);
            }
            ref instruction @ Instruction {
                name: InstructionName::SUB,
                ..
            } => {
                let operand_2 = block_stack.elements.get(block_stack.elements.len() - 2);
                let operand_1 = block_stack.elements.last();

                let result = match (operand_1, operand_2) {
                    (Some(Element::Constant(operand_1)), Some(Element::Constant(operand_2))) => {
                        match operand_1.checked_sub(operand_2) {
                            Some(result) => Element::Constant(result),
                            None => Element::Value,
                        }
                    }
                    _ => Element::Value,
                };

                block_stack.push(result);
                block_element.stack = block_stack.clone();
                let output = block_stack.pop()?;
                for _ in 0..instruction.input_size(version) {
                    block_stack.pop()?;
                }
                block_stack.push(output);
            }
            ref instruction @ Instruction {
                name: InstructionName::MUL,
                ..
            } => {
                let operand_2 = block_stack.elements.get(block_stack.elements.len() - 2);
                let operand_1 = block_stack.elements.last();

                let result = match (operand_1, operand_2) {
                    (Some(Element::Constant(operand_1)), Some(Element::Constant(operand_2))) => {
                        match operand_1.checked_mul(operand_2) {
                            Some(result) => Element::Constant(result),
                            None => Element::Value,
                        }
                    }
                    _ => Element::Value,
                };

                block_stack.push(result);
                block_element.stack = block_stack.clone();
                let output = block_stack.pop()?;
                for _ in 0..instruction.input_size(version) {
                    block_stack.pop()?;
                }
                block_stack.push(output);
            }
            ref instruction @ Instruction {
                name: InstructionName::SHL,
                ..
            } => {
                let value = block_stack.elements.get(block_stack.elements.len() - 2);
                let offset = block_stack.elements.last();

                let result = match (value, offset) {
                    (Some(Element::Tag(tag)), Some(Element::Constant(offset))) => {
                        let offset = offset.to_u64().expect("Always valid");
                        Element::Tag(tag << offset)
                    }
                    (Some(Element::Constant(constant)), Some(Element::Constant(offset))) => {
                        let offset = offset.to_u64().expect("Always valid");
                        Element::Constant(constant << offset)
                    }
                    _ => Element::Value,
                };

                block_stack.push(result);
                block_element.stack = block_stack.clone();
                let output = block_stack.pop()?;
                for _ in 0..instruction.input_size(version) {
                    block_stack.pop()?;
                }
                block_stack.push(output);
            }
            ref instruction @ Instruction {
                name: InstructionName::SHR,
                ..
            } => {
                let value = block_stack.elements.get(block_stack.elements.len() - 2);
                let offset = block_stack.elements.last();

                let result = match (value, offset) {
                    (Some(Element::Tag(tag)), Some(Element::Constant(offset))) => {
                        let offset = offset.to_u64().expect("Always valid");
                        Element::Tag(tag >> offset)
                    }
                    (Some(Element::Constant(constant)), Some(Element::Constant(offset))) => {
                        let offset = offset.to_u64().expect("Always valid");
                        Element::Constant(constant >> offset)
                    }
                    _ => Element::Value,
                };

                block_stack.push(result);
                block_element.stack = block_stack.clone();
                let output = block_stack.pop()?;
                for _ in 0..instruction.input_size(version) {
                    block_stack.pop()?;
                }
                block_stack.push(output);
            }
            ref instruction @ Instruction {
                name: InstructionName::OR,
                ..
            } => {
                let operand_1 = block_stack.elements.get(block_stack.elements.len() - 2);
                let operand_2 = block_stack.elements.last();

                let result = match (operand_1, operand_2) {
                    (Some(Element::Tag(operand_1)), Some(Element::Tag(operand_2)))
                    | (Some(Element::Tag(operand_1)), Some(Element::Constant(operand_2)))
                    | (Some(Element::Constant(operand_1)), Some(Element::Tag(operand_2))) => {
                        Element::Tag(operand_1 | operand_2)
                    }
                    (Some(Element::Constant(operand_1)), Some(Element::Constant(operand_2))) => {
                        Element::Constant(operand_1 | operand_2)
                    }
                    _ => Element::Value,
                };

                block_stack.push(result);
                block_element.stack = block_stack.clone();
                let output = block_stack.pop()?;
                for _ in 0..instruction.input_size(version) {
                    block_stack.pop()?;
                }
                block_stack.push(output);
            }
            ref instruction @ Instruction {
                name: InstructionName::XOR,
                ..
            } => {
                let operand_1 = block_stack.elements.get(block_stack.elements.len() - 2);
                let operand_2 = block_stack.elements.last();

                let result = match (operand_1, operand_2) {
                    (Some(Element::Tag(operand_1)), Some(Element::Tag(operand_2)))
                    | (Some(Element::Tag(operand_1)), Some(Element::Constant(operand_2)))
                    | (Some(Element::Constant(operand_1)), Some(Element::Tag(operand_2))) => {
                        Element::Tag(operand_1 ^ operand_2)
                    }
                    (Some(Element::Constant(operand_1)), Some(Element::Constant(operand_2))) => {
                        Element::Constant(operand_1 ^ operand_2)
                    }
                    _ => Element::Value,
                };

                block_stack.push(result);
                block_element.stack = block_stack.clone();
                let output = block_stack.pop()?;
                for _ in 0..instruction.input_size(version) {
                    block_stack.pop()?;
                }
                block_stack.push(output);
            }
            ref instruction @ Instruction {
                name: InstructionName::AND,
                ..
            } => {
                let operand_1 = block_stack.elements.get(block_stack.elements.len() - 2);
                let operand_2 = block_stack.elements.last();

                let result = match (operand_1, operand_2) {
                    (Some(Element::Tag(operand_1)), Some(Element::Tag(operand_2)))
                    | (Some(Element::Tag(operand_1)), Some(Element::Constant(operand_2)))
                    | (Some(Element::Constant(operand_1)), Some(Element::Tag(operand_2))) => {
                        Element::Tag(operand_1 & operand_2)
                    }
                    (Some(Element::Constant(operand_1)), Some(Element::Constant(operand_2))) => {
                        Element::Constant(operand_1 & operand_2)
                    }
                    _ => Element::Value,
                };

                block_stack.push(result);
                block_element.stack = block_stack.clone();
                let output = block_stack.pop()?;
                for _ in 0..instruction.input_size(version) {
                    block_stack.pop()?;
                }
                block_stack.push(output);
            }

            ref instruction if instruction.output_size() == 1 => {
                block_stack.push(StackElement::Value);
                block_element.stack = block_stack.clone();
                let output = block_stack.pop()?;
                for _ in 0..instruction.input_size(version) {
                    block_stack.pop()?;
                }
                block_stack.push(output);
            }

            ref instruction => {
                block_element.stack = block_stack.clone();
                for _ in 0..instruction.input_size(version) {
                    block_stack.pop()?;
                }
            }
        }

        Ok(())
    }

    ///
    /// Finds a block with a compatible initial stack state and returns it, adding an
    /// additional allowed initial stack state hash.
    ///
    fn find_compatible_block(&mut self, block: Block) -> Block {
        let key = block.key.clone();

        if let Some(entry) = self.blocks.get_mut(&key) {
            for existing_block in entry.iter_mut() {
                let existing_block_length = existing_block.initial_stack.elements.len();
                let new_block_length = block.initial_stack.elements.len();

                if new_block_length > existing_block_length {
                    let stack_subslice =
                        &block.initial_stack.elements[new_block_length - existing_block_length..];
                    if stack_subslice == existing_block.initial_stack.elements.as_slice()
                        && stack_subslice
                            .iter()
                            .all(|element| element == &StackElement::Value)
                    {
                        existing_block.extra_hashes.push(block.initial_stack.hash());
                        return existing_block.to_owned();
                    }
                }
            }
        }

        block
    }

    ///
    /// Pushes a block into the function.
    ///
    fn insert_block(&mut self, block: Block) -> &mut Block {
        let key = block.key.clone();

        if let Some(entry) = self.blocks.get_mut(&key) {
            if entry.iter().all(|existing_block| {
                existing_block.initial_stack.hash() != block.initial_stack.hash()
            }) {
                entry.push(block);
            }
        } else {
            self.blocks.insert(key.clone(), vec![block]);
        }

        self.blocks
            .get_mut(&key)
            .expect("Always exists")
            .last_mut()
            .expect("Always exists")
    }

    ///
    /// Finalizes the function data.
    ///
    fn finalize(mut self) -> Self {
        for (_tag, blocks) in self.blocks.iter() {
            for block in blocks.iter() {
                for block_element in block.elements.iter() {
                    if block_element.stack.elements.len() > self.stack_size {
                        self.stack_size = block_element.stack.elements.len();
                    }
                }
            }
        }

        self
    }
}

impl<D> compiler_llvm_context::WriteLLVM<D> for Function
where
    D: compiler_llvm_context::Dependency,
{
    fn declare(&mut self, context: &mut compiler_llvm_context::Context<D>) -> anyhow::Result<()> {
        let function = context.add_function(
            EtherealIR::DEFAULT_ENTRY_FUNCTION_NAME,
            context.function_type(
                vec![context
                    .integer_type(compiler_common::BIT_LENGTH_BOOLEAN)
                    .as_basic_type_enum()],
                0,
                false,
            ),
            0,
            Some(inkwell::module::Linkage::Private),
        )?;
        function
            .borrow_mut()
            .set_evmla_data(compiler_llvm_context::FunctionEVMLAData::new(
                self.stack_size,
            ));

        Ok(())
    }

    fn into_llvm(self, context: &mut compiler_llvm_context::Context<D>) -> anyhow::Result<()> {
        context.set_current_function(EtherealIR::DEFAULT_ENTRY_FUNCTION_NAME)?;
        let is_deploy_code_flag = context
            .current_function()
            .borrow()
            .get_nth_param(0)
            .into_int_value();

        for (key, blocks) in self.blocks.iter() {
            for (index, block) in blocks.iter().enumerate() {
                let inner = context.append_basic_block(format!("block_{key}/{index}").as_str());
                let mut stack_hashes = vec![block.initial_stack.hash()];
                stack_hashes.extend_from_slice(block.extra_hashes.as_slice());
                let evmla_data = compiler_llvm_context::FunctionBlockEVMLAData::new(stack_hashes);
                let mut block = compiler_llvm_context::FunctionBlock::new(inner);
                block.set_evmla_data(evmla_data);
                context
                    .current_function()
                    .borrow_mut()
                    .evmla_mut()
                    .insert_block(key.to_owned(), block);
            }
        }

        context.set_basic_block(context.current_function().borrow().entry_block());
        let mut stack_variables = Vec::with_capacity(self.stack_size);
        for stack_index in 0..self.stack_size {
            let pointer = context.build_alloca(
                context.field_type(),
                format!("stack_var_{stack_index:03}").as_str(),
            );
            stack_variables.push(compiler_llvm_context::Argument::new(
                pointer.value.as_basic_value_enum(),
            ));
        }
        context.evmla_mut().stack = stack_variables;

        let deploy_code_block = context.current_function().borrow().evmla().find_block(
            &compiler_llvm_context::FunctionBlockKey::new(
                compiler_llvm_context::CodeType::Deploy,
                num::BigUint::zero(),
            ),
            &Stack::default().hash(),
        )?;
        let runtime_code_block = context.current_function().borrow().evmla().find_block(
            &compiler_llvm_context::FunctionBlockKey::new(
                compiler_llvm_context::CodeType::Runtime,
                num::BigUint::zero(),
            ),
            &Stack::default().hash(),
        )?;
        context.build_conditional_branch(
            is_deploy_code_flag,
            deploy_code_block.inner(),
            runtime_code_block.inner(),
        );

        for (key, blocks) in self.blocks.into_iter() {
            for (llvm_block, ir_block) in context
                .current_function()
                .borrow()
                .evmla()
                .blocks
                .get(&key)
                .cloned()
                .ok_or_else(|| anyhow::anyhow!("Undeclared function block {}", key))?
                .into_iter()
                .map(|block| block.inner())
                .zip(blocks)
            {
                context.set_basic_block(llvm_block);
                ir_block.into_llvm(context)?;
            }
        }

        context.set_basic_block(context.current_function().borrow().return_block());
        context.build_return(None);

        Ok(())
    }
}

impl std::fmt::Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "function main (max_sp = {}) {{", self.stack_size,)?;
        for (key, blocks) in self.blocks.iter() {
            for (index, block) in blocks.iter().enumerate() {
                writeln!(
                    f,
                    "{:92}{}",
                    format!(
                        "block_{}/{}: {}",
                        key,
                        index,
                        if block.predecessors.is_empty() {
                            "".to_owned()
                        } else {
                            format!("(predecessors: {:?})", block.predecessors)
                        }
                    ),
                    block.initial_stack,
                )?;
                write!(f, "{block}")?;
            }
        }
        writeln!(f, "}}")?;

        Ok(())
    }
}

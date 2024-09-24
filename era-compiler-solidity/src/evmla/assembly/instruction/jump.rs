//!
//! Translates the jump operations.
//!

use era_compiler_llvm_context::IEVMLAData;
use era_compiler_llvm_context::IEVMLAFunction;

///
/// Translates the unconditional jump.
///
pub fn unconditional<'ctx, C>(
    context: &mut C,
    destination: num::BigUint,
    stack_hash: [u8; era_compiler_common::BYTE_LENGTH_FIELD],
) -> anyhow::Result<()>
where
    C: era_compiler_llvm_context::IContext<'ctx>,
{
    let code_type = context
        .code_type()
        .ok_or_else(|| anyhow::anyhow!("Contract code part type is undefined"))?;
    let block_key = match code_type {
        era_compiler_llvm_context::CodeType::Deploy
            if destination > num::BigUint::from(u32::MAX) =>
        {
            era_compiler_llvm_context::BlockKey::new(
                era_compiler_llvm_context::CodeType::Runtime,
                destination.to_owned() - num::BigUint::from(1u64 << 32),
            )
        }
        code_type => era_compiler_llvm_context::BlockKey::new(code_type, destination),
    };

    let block = context
        .current_function()
        .borrow()
        .find_block(&block_key, &stack_hash)?;
    context.build_unconditional_branch(block.inner())?;

    Ok(())
}

///
/// Translates the conditional jump.
///
pub fn conditional<'ctx, C>(
    context: &mut C,
    destination: num::BigUint,
    stack_hash: [u8; era_compiler_common::BYTE_LENGTH_FIELD],
    stack_height: usize,
) -> anyhow::Result<()>
where
    C: era_compiler_llvm_context::IContext<'ctx>,
{
    let code_type = context
        .code_type()
        .ok_or_else(|| anyhow::anyhow!("Contract code part type is undefined"))?;
    let block_key = match code_type {
        era_compiler_llvm_context::CodeType::Deploy
            if destination > num::BigUint::from(u32::MAX) =>
        {
            era_compiler_llvm_context::BlockKey::new(
                era_compiler_llvm_context::CodeType::Runtime,
                destination.to_owned() - num::BigUint::from(1u64 << 32),
            )
        }
        code_type => era_compiler_llvm_context::BlockKey::new(code_type, destination),
    };

    let condition_pointer = context
        .evmla()
        .get_element(stack_height)
        .to_llvm()
        .into_pointer_value();
    let condition = context.build_load(
        era_compiler_llvm_context::Pointer::new_stack_field(context, condition_pointer),
        format!("conditional_{block_key}_condition").as_str(),
    )?;
    let condition = context.builder().build_int_compare(
        inkwell::IntPredicate::NE,
        condition.into_int_value(),
        context.field_const(0),
        format!("conditional_{block_key}_condition_compared").as_str(),
    )?;

    let then_block = context
        .current_function()
        .borrow()
        .find_block(&block_key, &stack_hash)?;
    let join_block =
        context.append_basic_block(format!("conditional_{block_key}_join_block").as_str());

    context.build_conditional_branch(condition, then_block.inner(), join_block)?;

    context.set_basic_block(join_block);

    Ok(())
}

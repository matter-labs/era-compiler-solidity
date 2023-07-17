//!
//! Translates the verbatim simulations.
//!

use crate::yul::parser::statement::expression::function_call::FunctionCall;

///
/// Translates the verbatim simulations.
///
pub fn verbatim<'ctx, D>(
    context: &mut compiler_llvm_context::Context<'ctx, D>,
    call: &mut FunctionCall,
    input_size: usize,
    output_size: usize,
) -> anyhow::Result<Option<inkwell::values::BasicValueEnum<'ctx>>>
where
    D: compiler_llvm_context::Dependency + Clone,
{
    if output_size > 1 {
        anyhow::bail!(
            "{} Verbatim instructions with multiple return values are not supported",
            call.location
        );
    }

    let mut arguments = call.pop_arguments::<D, 1>(context)?;
    let identifier = arguments[0]
        .original
        .take()
        .ok_or_else(|| anyhow::anyhow!("{} Verbatim literal is missing", call.location))?;
    match identifier.as_str() {
        identifier @ "to_l1" => {
            /// The number of arguments expected by this verbatim variant.
            const ARGUMENTS_COUNT: usize = 3;
            if input_size != ARGUMENTS_COUNT {
                anyhow::bail!(
                    "{} Internal function `{}` expected {} arguments, found {}",
                    call.location,
                    identifier,
                    ARGUMENTS_COUNT,
                    input_size
                );
            }

            let arguments = call.pop_arguments_llvm::<D, ARGUMENTS_COUNT>(context)?;
            compiler_llvm_context::zkevm_general::to_l1(
                context,
                arguments[0].into_int_value(),
                arguments[1].into_int_value(),
                arguments[2].into_int_value(),
            )
            .map(Some)
        }
        identifier @ "code_source" => {
            /// The number of arguments expected by this verbatim variant.
            const ARGUMENTS_COUNT: usize = 0;
            if input_size != ARGUMENTS_COUNT {
                anyhow::bail!(
                    "{} Internal function `{}` expected {} arguments, found {}",
                    call.location,
                    identifier,
                    ARGUMENTS_COUNT,
                    input_size
                );
            }

            compiler_llvm_context::zkevm_general::code_source(context).map(Some)
        }
        identifier @ "precompile" => {
            /// The number of arguments expected by this verbatim variant.
            const ARGUMENTS_COUNT: usize = 2;
            if input_size != ARGUMENTS_COUNT {
                anyhow::bail!(
                    "{} Internal function `{}` expected {} arguments, found {}",
                    call.location,
                    identifier,
                    ARGUMENTS_COUNT,
                    input_size
                );
            }

            let arguments = call.pop_arguments_llvm::<D, ARGUMENTS_COUNT>(context)?;
            compiler_llvm_context::zkevm_general::precompile(
                context,
                arguments[0].into_int_value(),
                arguments[1].into_int_value(),
            )
            .map(Some)
        }
        identifier @ "meta" => {
            /// The number of arguments expected by this verbatim variant.
            const ARGUMENTS_COUNT: usize = 0;
            if input_size != ARGUMENTS_COUNT {
                anyhow::bail!(
                    "{} Internal function `{}` expected {} arguments, found {}",
                    call.location,
                    identifier,
                    ARGUMENTS_COUNT,
                    input_size
                );
            }

            compiler_llvm_context::zkevm_general::meta(context).map(Some)
        }
        identifier @ "mimic_call" => {
            /// The number of arguments expected by this verbatim variant.
            const ARGUMENTS_COUNT: usize = 3;
            if input_size != ARGUMENTS_COUNT {
                anyhow::bail!(
                    "{} Internal function `{}` expected {} arguments, found {}",
                    call.location,
                    identifier,
                    ARGUMENTS_COUNT,
                    input_size
                );
            }

            let arguments = call.pop_arguments_llvm::<D, ARGUMENTS_COUNT>(context)?;
            compiler_llvm_context::zkevm_call::mimic(
                context,
                context.llvm_runtime().mimic_call,
                arguments[0].into_int_value(),
                arguments[1].into_int_value(),
                arguments[2],
                vec![context.field_const(0), context.field_const(0)],
            )
            .map(Some)
        }
        identifier @ "mimic_call_byref" => {
            /// The number of arguments expected by this verbatim variant.
            const ARGUMENTS_COUNT: usize = 2;
            if input_size != ARGUMENTS_COUNT {
                anyhow::bail!(
                    "{} Internal function `{}` expected {} arguments, found {}",
                    call.location,
                    identifier,
                    ARGUMENTS_COUNT,
                    input_size
                );
            }

            let arguments = call.pop_arguments_llvm::<D, ARGUMENTS_COUNT>(context)?;
            compiler_llvm_context::zkevm_call::mimic(
                context,
                context.llvm_runtime().mimic_call_byref,
                arguments[0].into_int_value(),
                arguments[1].into_int_value(),
                context.get_global(compiler_llvm_context::GLOBAL_ACTIVE_POINTER)?,
                vec![context.field_const(0), context.field_const(0)],
            )
            .map(Some)
        }
        identifier @ "system_mimic_call" => {
            /// The number of arguments expected by this verbatim variant.
            const ARGUMENTS_COUNT: usize = 7;
            if input_size != ARGUMENTS_COUNT {
                anyhow::bail!(
                    "{} Internal function `{}` expected {} arguments, found {}",
                    call.location,
                    identifier,
                    ARGUMENTS_COUNT,
                    input_size
                );
            }

            let arguments = call.pop_arguments_llvm::<D, ARGUMENTS_COUNT>(context)?;
            compiler_llvm_context::zkevm_call::mimic(
                context,
                context.llvm_runtime().mimic_call,
                arguments[0].into_int_value(),
                arguments[1].into_int_value(),
                arguments[2],
                vec![
                    arguments[3].into_int_value(),
                    arguments[4].into_int_value(),
                    arguments[5].into_int_value(),
                    arguments[6].into_int_value(),
                ],
            )
            .map(Some)
        }
        identifier @ "system_mimic_call_byref" => {
            /// The number of arguments expected by this verbatim variant.
            const ARGUMENTS_COUNT: usize = 6;
            if input_size != ARGUMENTS_COUNT {
                anyhow::bail!(
                    "{} Internal function `{}` expected {} arguments, found {}",
                    call.location,
                    identifier,
                    ARGUMENTS_COUNT,
                    input_size
                );
            }

            let arguments = call.pop_arguments_llvm::<D, ARGUMENTS_COUNT>(context)?;
            compiler_llvm_context::zkevm_call::mimic(
                context,
                context.llvm_runtime().mimic_call_byref,
                arguments[0].into_int_value(),
                arguments[1].into_int_value(),
                context.get_global(compiler_llvm_context::GLOBAL_ACTIVE_POINTER)?,
                vec![
                    arguments[2].into_int_value(),
                    arguments[3].into_int_value(),
                    arguments[4].into_int_value(),
                    arguments[5].into_int_value(),
                ],
            )
            .map(Some)
        }
        identifier @ "raw_call" => {
            /// The number of arguments expected by this verbatim variant.
            const ARGUMENTS_COUNT: usize = 4;
            if input_size != ARGUMENTS_COUNT {
                anyhow::bail!(
                    "{} Internal function `{}` expected {} arguments, found {}",
                    call.location,
                    identifier,
                    ARGUMENTS_COUNT,
                    input_size
                );
            }

            let arguments = call.pop_arguments_llvm::<D, ARGUMENTS_COUNT>(context)?;
            compiler_llvm_context::zkevm_call::raw_far(
                context,
                context.llvm_runtime().far_call,
                arguments[0].into_int_value(),
                arguments[1],
                arguments[2].into_int_value(),
                arguments[3].into_int_value(),
            )
            .map(Some)
        }
        identifier @ "raw_call_byref" => {
            /// The number of arguments expected by this verbatim variant.
            const ARGUMENTS_COUNT: usize = 3;
            if input_size != ARGUMENTS_COUNT {
                anyhow::bail!(
                    "{} Internal function `{}` expected {} arguments, found {}",
                    call.location,
                    identifier,
                    ARGUMENTS_COUNT,
                    input_size
                );
            }

            let arguments = call.pop_arguments_llvm::<D, ARGUMENTS_COUNT>(context)?;
            compiler_llvm_context::zkevm_call::raw_far(
                context,
                context.llvm_runtime().far_call_byref,
                arguments[0].into_int_value(),
                context.get_global(compiler_llvm_context::GLOBAL_ACTIVE_POINTER)?,
                arguments[1].into_int_value(),
                arguments[2].into_int_value(),
            )
            .map(Some)
        }
        identifier @ "system_call" => {
            /// The number of arguments expected by this verbatim variant.
            const ARGUMENTS_COUNT: usize = 6;
            if input_size != ARGUMENTS_COUNT {
                anyhow::bail!(
                    "{} Internal function `{}` expected {} arguments, found {}",
                    call.location,
                    identifier,
                    ARGUMENTS_COUNT,
                    input_size
                );
            }

            let arguments = call.pop_arguments_llvm::<D, ARGUMENTS_COUNT>(context)?;
            compiler_llvm_context::zkevm_call::system(
                context,
                context.llvm_runtime().far_call,
                arguments[0].into_int_value(),
                arguments[1],
                context.field_const(0),
                context.field_const(0),
                vec![
                    arguments[2].into_int_value(),
                    arguments[3].into_int_value(),
                    arguments[4].into_int_value(),
                    arguments[5].into_int_value(),
                ],
            )
            .map(Some)
        }
        identifier @ "system_call_byref" => {
            /// The number of arguments expected by this verbatim variant.
            const ARGUMENTS_COUNT: usize = 5;
            if input_size != ARGUMENTS_COUNT {
                anyhow::bail!(
                    "{} Internal function `{}` expected {} arguments, found {}",
                    call.location,
                    identifier,
                    ARGUMENTS_COUNT,
                    input_size
                );
            }

            let arguments = call.pop_arguments_llvm::<D, ARGUMENTS_COUNT>(context)?;
            compiler_llvm_context::zkevm_call::system(
                context,
                context.llvm_runtime().far_call_byref,
                arguments[0].into_int_value(),
                context.get_global(compiler_llvm_context::GLOBAL_ACTIVE_POINTER)?,
                context.field_const(0),
                context.field_const(0),
                vec![
                    arguments[1].into_int_value(),
                    arguments[2].into_int_value(),
                    arguments[3].into_int_value(),
                    arguments[4].into_int_value(),
                ],
            )
            .map(Some)
        }
        identifier @ "raw_static_call" => {
            /// The number of arguments expected by this verbatim variant.
            const ARGUMENTS_COUNT: usize = 4;
            if input_size != ARGUMENTS_COUNT {
                anyhow::bail!(
                    "{} Internal function `{}` expected {} arguments, found {}",
                    call.location,
                    identifier,
                    ARGUMENTS_COUNT,
                    input_size
                );
            }

            let arguments = call.pop_arguments_llvm::<D, ARGUMENTS_COUNT>(context)?;
            compiler_llvm_context::zkevm_call::raw_far(
                context,
                context.llvm_runtime().static_call,
                arguments[0].into_int_value(),
                arguments[1],
                arguments[2].into_int_value(),
                arguments[3].into_int_value(),
            )
            .map(Some)
        }
        identifier @ "raw_static_call_byref" => {
            /// The number of arguments expected by this verbatim variant.
            const ARGUMENTS_COUNT: usize = 3;
            if input_size != ARGUMENTS_COUNT {
                anyhow::bail!(
                    "{} Internal function `{}` expected {} arguments, found {}",
                    call.location,
                    identifier,
                    ARGUMENTS_COUNT,
                    input_size
                );
            }

            let arguments = call.pop_arguments_llvm::<D, ARGUMENTS_COUNT>(context)?;
            compiler_llvm_context::zkevm_call::raw_far(
                context,
                context.llvm_runtime().static_call_byref,
                arguments[0].into_int_value(),
                context.get_global(compiler_llvm_context::GLOBAL_ACTIVE_POINTER)?,
                arguments[1].into_int_value(),
                arguments[2].into_int_value(),
            )
            .map(Some)
        }
        identifier @ "system_static_call" => {
            /// The number of arguments expected by this verbatim variant.
            const ARGUMENTS_COUNT: usize = 6;
            if input_size != ARGUMENTS_COUNT {
                anyhow::bail!(
                    "{} Internal function `{}` expected {} arguments, found {}",
                    call.location,
                    identifier,
                    ARGUMENTS_COUNT,
                    input_size
                );
            }

            let arguments = call.pop_arguments_llvm::<D, ARGUMENTS_COUNT>(context)?;
            compiler_llvm_context::zkevm_call::system(
                context,
                context.llvm_runtime().static_call,
                arguments[0].into_int_value(),
                arguments[1],
                arguments[4].into_int_value(),
                arguments[5].into_int_value(),
                vec![arguments[2].into_int_value(), arguments[3].into_int_value()],
            )
            .map(Some)
        }
        identifier @ "system_static_call_byref" => {
            /// The number of arguments expected by this verbatim variant.
            const ARGUMENTS_COUNT: usize = 5;
            if input_size != ARGUMENTS_COUNT {
                anyhow::bail!(
                    "{} Internal function `{}` expected {} arguments, found {}",
                    call.location,
                    identifier,
                    ARGUMENTS_COUNT,
                    input_size
                );
            }

            let arguments = call.pop_arguments_llvm::<D, ARGUMENTS_COUNT>(context)?;
            compiler_llvm_context::zkevm_call::system(
                context,
                context.llvm_runtime().static_call_byref,
                arguments[0].into_int_value(),
                context.get_global(compiler_llvm_context::GLOBAL_ACTIVE_POINTER)?,
                arguments[3].into_int_value(),
                arguments[4].into_int_value(),
                vec![arguments[1].into_int_value(), arguments[2].into_int_value()],
            )
            .map(Some)
        }
        identifier @ "raw_delegate_call" => {
            /// The number of arguments expected by this verbatim variant.
            const ARGUMENTS_COUNT: usize = 4;
            if input_size != ARGUMENTS_COUNT {
                anyhow::bail!(
                    "{} Internal function `{}` expected {} arguments, found {}",
                    call.location,
                    identifier,
                    ARGUMENTS_COUNT,
                    input_size
                );
            }

            let arguments = call.pop_arguments_llvm::<D, ARGUMENTS_COUNT>(context)?;
            compiler_llvm_context::zkevm_call::raw_far(
                context,
                context.llvm_runtime().delegate_call,
                arguments[0].into_int_value(),
                arguments[1],
                arguments[2].into_int_value(),
                arguments[3].into_int_value(),
            )
            .map(Some)
        }
        identifier @ "raw_delegate_call_byref" => {
            /// The number of arguments expected by this verbatim variant.
            const ARGUMENTS_COUNT: usize = 3;
            if input_size != ARGUMENTS_COUNT {
                anyhow::bail!(
                    "{} Internal function `{}` expected {} arguments, found {}",
                    call.location,
                    identifier,
                    ARGUMENTS_COUNT,
                    input_size
                );
            }

            let arguments = call.pop_arguments_llvm::<D, ARGUMENTS_COUNT>(context)?;
            compiler_llvm_context::zkevm_call::raw_far(
                context,
                context.llvm_runtime().delegate_call_byref,
                arguments[0].into_int_value(),
                context.get_global(compiler_llvm_context::GLOBAL_ACTIVE_POINTER)?,
                arguments[1].into_int_value(),
                arguments[2].into_int_value(),
            )
            .map(Some)
        }
        identifier @ "system_delegate_call" => {
            /// The number of arguments expected by this verbatim variant.
            const ARGUMENTS_COUNT: usize = 6;
            if input_size != ARGUMENTS_COUNT {
                anyhow::bail!(
                    "{} Internal function `{}` expected {} arguments, found {}",
                    call.location,
                    identifier,
                    ARGUMENTS_COUNT,
                    input_size
                );
            }

            let arguments = call.pop_arguments_llvm::<D, ARGUMENTS_COUNT>(context)?;
            compiler_llvm_context::zkevm_call::system(
                context,
                context.llvm_runtime().delegate_call,
                arguments[0].into_int_value(),
                arguments[1],
                arguments[4].into_int_value(),
                arguments[5].into_int_value(),
                vec![arguments[2].into_int_value(), arguments[3].into_int_value()],
            )
            .map(Some)
        }
        identifier @ "system_delegate_call_byref" => {
            /// The number of arguments expected by this verbatim variant.
            const ARGUMENTS_COUNT: usize = 5;
            if input_size != ARGUMENTS_COUNT {
                anyhow::bail!(
                    "{} Internal function `{}` expected {} arguments, found {}",
                    call.location,
                    identifier,
                    ARGUMENTS_COUNT,
                    input_size
                );
            }

            let arguments = call.pop_arguments_llvm::<D, ARGUMENTS_COUNT>(context)?;
            compiler_llvm_context::zkevm_call::system(
                context,
                context.llvm_runtime().delegate_call_byref,
                arguments[0].into_int_value(),
                context.get_global(compiler_llvm_context::GLOBAL_ACTIVE_POINTER)?,
                arguments[3].into_int_value(),
                arguments[4].into_int_value(),
                vec![arguments[1].into_int_value(), arguments[2].into_int_value()],
            )
            .map(Some)
        }
        identifier @ "set_context_u128" => {
            /// The number of arguments expected by this verbatim variant.
            const ARGUMENTS_COUNT: usize = 1;
            if input_size != ARGUMENTS_COUNT {
                anyhow::bail!(
                    "{} Internal function `{}` expected {} arguments, found {}",
                    call.location,
                    identifier,
                    ARGUMENTS_COUNT,
                    input_size
                );
            }

            let arguments = call.pop_arguments_llvm::<D, ARGUMENTS_COUNT>(context)?;
            compiler_llvm_context::zkevm_general::set_context_value(
                context,
                arguments[0].into_int_value(),
            )
            .map(Some)
        }
        identifier @ "set_pubdata_price" => {
            /// The number of arguments expected by this verbatim variant.
            const ARGUMENTS_COUNT: usize = 1;
            if input_size != ARGUMENTS_COUNT {
                anyhow::bail!(
                    "{} Internal function `{}` expected {} arguments, found {}",
                    call.location,
                    identifier,
                    ARGUMENTS_COUNT,
                    input_size
                );
            }

            let arguments = call.pop_arguments_llvm::<D, ARGUMENTS_COUNT>(context)?;
            compiler_llvm_context::zkevm_general::set_pubdata_price(
                context,
                arguments[0].into_int_value(),
            )
            .map(Some)
        }
        identifier @ "increment_tx_counter" => {
            /// The number of arguments expected by this verbatim variant.
            const ARGUMENTS_COUNT: usize = 0;
            if input_size != ARGUMENTS_COUNT {
                anyhow::bail!(
                    "{} Internal function `{}` expected {} arguments, found {}",
                    call.location,
                    identifier,
                    ARGUMENTS_COUNT,
                    input_size
                );
            }

            compiler_llvm_context::zkevm_general::increment_tx_counter(context).map(Some)
        }
        identifier @ "event_initialize" => {
            /// The number of arguments expected by this verbatim variant.
            const ARGUMENTS_COUNT: usize = 2;
            if input_size != ARGUMENTS_COUNT {
                anyhow::bail!(
                    "{} Internal function `{}` expected {} arguments, found {}",
                    call.location,
                    identifier,
                    ARGUMENTS_COUNT,
                    input_size
                );
            }

            let arguments = call.pop_arguments_llvm::<D, ARGUMENTS_COUNT>(context)?;
            compiler_llvm_context::zkevm_general::event(
                context,
                arguments[0].into_int_value(),
                arguments[1].into_int_value(),
                true,
            )
            .map(Some)
        }
        identifier @ "event_write" => {
            /// The number of arguments expected by this verbatim variant.
            const ARGUMENTS_COUNT: usize = 2;
            if input_size != ARGUMENTS_COUNT {
                anyhow::bail!(
                    "{} Internal function `{}` expected {} arguments, found {}",
                    call.location,
                    identifier,
                    ARGUMENTS_COUNT,
                    input_size
                );
            }

            let arguments = call.pop_arguments_llvm::<D, ARGUMENTS_COUNT>(context)?;
            compiler_llvm_context::zkevm_general::event(
                context,
                arguments[0].into_int_value(),
                arguments[1].into_int_value(),
                false,
            )
            .map(Some)
        }
        identifier @ "calldata_ptr_to_active" => {
            /// The number of arguments expected by this verbatim variant.
            const ARGUMENTS_COUNT: usize = 0;
            if input_size != ARGUMENTS_COUNT {
                anyhow::bail!(
                    "{} Internal function `{}` expected {} arguments, found {}",
                    call.location,
                    identifier,
                    ARGUMENTS_COUNT,
                    input_size
                );
            }

            compiler_llvm_context::zkevm_abi::calldata_ptr_to_active(context).map(Some)
        }
        identifier @ "return_data_ptr_to_active" => {
            /// The number of arguments expected by this verbatim variant.
            const ARGUMENTS_COUNT: usize = 0;
            if input_size != ARGUMENTS_COUNT {
                anyhow::bail!(
                    "{} Internal function `{}` expected {} arguments, found {}",
                    call.location,
                    identifier,
                    ARGUMENTS_COUNT,
                    input_size
                );
            }

            compiler_llvm_context::zkevm_abi::return_data_ptr_to_active(context).map(Some)
        }
        identifier @ "active_ptr_add_assign" => {
            /// The number of arguments expected by this verbatim variant.
            const ARGUMENTS_COUNT: usize = 1;
            if input_size != ARGUMENTS_COUNT {
                anyhow::bail!(
                    "{} Internal function `{}` expected {} arguments, found {}",
                    call.location,
                    identifier,
                    ARGUMENTS_COUNT,
                    input_size
                );
            }

            let arguments = call.pop_arguments_llvm::<D, ARGUMENTS_COUNT>(context)?;
            compiler_llvm_context::zkevm_abi::active_ptr_add_assign(
                context,
                arguments[0].into_int_value(),
            )
            .map(Some)
        }
        identifier @ "active_ptr_shrink_assign" => {
            /// The number of arguments expected by this verbatim variant.
            const ARGUMENTS_COUNT: usize = 1;
            if input_size != ARGUMENTS_COUNT {
                anyhow::bail!(
                    "{} Internal function `{}` expected {} arguments, found {}",
                    call.location,
                    identifier,
                    ARGUMENTS_COUNT,
                    input_size
                );
            }

            let arguments = call.pop_arguments_llvm::<D, ARGUMENTS_COUNT>(context)?;
            compiler_llvm_context::zkevm_abi::active_ptr_shrink_assign(
                context,
                arguments[0].into_int_value(),
            )
            .map(Some)
        }
        identifier @ "active_ptr_pack_assign" => {
            /// The number of arguments expected by this verbatim variant.
            const ARGUMENTS_COUNT: usize = 1;
            if input_size != ARGUMENTS_COUNT {
                anyhow::bail!(
                    "{} Internal function `{}` expected {} arguments, found {}",
                    call.location,
                    identifier,
                    ARGUMENTS_COUNT,
                    input_size
                );
            }

            let arguments = call.pop_arguments_llvm::<D, ARGUMENTS_COUNT>(context)?;
            compiler_llvm_context::zkevm_abi::active_ptr_pack_assign(
                context,
                arguments[0].into_int_value(),
            )
            .map(Some)
        }
        identifier @ "mul_high" => {
            /// The number of arguments expected by this verbatim variant.
            const ARGUMENTS_COUNT: usize = 2;
            if input_size != ARGUMENTS_COUNT {
                anyhow::bail!(
                    "{} Internal function `{}` expected {} arguments, found {}",
                    call.location,
                    identifier,
                    ARGUMENTS_COUNT,
                    input_size
                );
            }

            let arguments = call.pop_arguments_llvm::<D, ARGUMENTS_COUNT>(context)?;
            compiler_llvm_context::zkevm_math::multiplication_512(
                context,
                arguments[0].into_int_value(),
                arguments[1].into_int_value(),
            )
            .map(Some)
        }
        identifier @ "throw" => {
            /// The number of arguments expected by this verbatim variant.
            const ARGUMENTS_COUNT: usize = 0;
            if input_size != ARGUMENTS_COUNT {
                anyhow::bail!(
                    "{} Internal function `{}` expected {} arguments, found {}",
                    call.location,
                    identifier,
                    ARGUMENTS_COUNT,
                    input_size
                );
            }

            compiler_llvm_context::throw(context).map(|_| None)
        }
        identifier
            if identifier.starts_with(compiler_llvm_context::GLOBAL_VERBATIM_GETTER_PREFIX) =>
        {
            /// The number of arguments expected by this verbatim variant.
            const ARGUMENTS_COUNT: usize = 0;
            if input_size != ARGUMENTS_COUNT {
                anyhow::bail!(
                    "{} Internal function `{}` expected {} arguments, found {}",
                    call.location,
                    identifier,
                    ARGUMENTS_COUNT,
                    input_size
                );
            }

            match identifier.strip_prefix(compiler_llvm_context::GLOBAL_VERBATIM_GETTER_PREFIX) {
                Some(identifier)
                    if identifier == compiler_llvm_context::GLOBAL_CALLDATA_POINTER =>
                {
                    context.get_global(identifier).map(Some)
                }
                Some(identifier) if identifier == compiler_llvm_context::GLOBAL_CALL_FLAGS => {
                    context.get_global(identifier).map(Some)
                }
                Some(identifier)
                    if identifier == compiler_llvm_context::GLOBAL_RETURN_DATA_POINTER =>
                {
                    context.get_global(identifier).map(Some)
                }
                Some(identifier)
                    if identifier.starts_with(compiler_llvm_context::GLOBAL_EXTRA_ABI_DATA) =>
                {
                    let stripped = identifier
                        .strip_prefix(compiler_llvm_context::GLOBAL_EXTRA_ABI_DATA)
                        .expect("Always exists");
                    let stripped = stripped.strip_prefix('_').ok_or_else(|| {
                        anyhow::anyhow!(
                            "{} Invalid global variable identifier `{:?}`",
                            call.location,
                            identifier
                        )
                    })?;
                    let index = stripped.parse::<u64>().map_err(|error| {
                        anyhow::anyhow!(
                            "{} Invalid global variable identifier `{:?}`: {}",
                            call.location,
                            identifier,
                            error,
                        )
                    })?;
                    if index >= (compiler_llvm_context::EXTRA_ABI_DATA_SIZE as u64) {
                        anyhow::bail!(
                            "{} Extra ABI data overflow. Only indexes `0..=9` are allowed",
                            call.location,
                        );
                    }
                    compiler_llvm_context::zkevm_abi::get_extra_abi_data(
                        context,
                        context.field_const(index),
                    )
                    .map(Some)
                }
                identifier => Err(anyhow::anyhow!(
                    "{} Invalid global variable identifier `{:?}`",
                    call.location,
                    identifier
                )),
            }
        }
        identifier => anyhow::bail!(
            "{} Found unknown internal function `{}`",
            call.location,
            identifier
        ),
    }
}

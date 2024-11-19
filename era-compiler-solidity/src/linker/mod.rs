//!
//! The linker.
//!

pub mod input;
pub mod output;

use self::input::Input;
use self::output::contract::Contract as OutputContract;
use self::output::Output;

///
/// The linker.
///
#[derive(Debug, Default)]
pub struct Linker {}

impl Linker {
    ///
    /// Links EraVM bytecode files.
    ///
    pub fn link_eravm(input: Input) -> anyhow::Result<Output> {
        let linker_symbols =
            era_solc::StandardJsonInputLibraries::try_from(input.libraries.as_slice())?
                .as_linker_symbols()?;
        let mut output = Output::default();

        input
            .bytecodes
            .into_iter()
            .try_for_each(|(path, bytecode)| -> anyhow::Result<()> {
                let bytecode =
                    hex::decode(bytecode.strip_prefix("0x").unwrap_or(bytecode.as_str()))?;

                let memory_buffer = inkwell::memory_buffer::MemoryBuffer::create_from_memory_range(
                    bytecode.as_slice(),
                    "bytecode",
                    false,
                );
                let already_linked = !memory_buffer.is_elf_eravm();

                let (memory_buffer_linked, bytecode_hash) =
                    era_compiler_llvm_context::eravm_link(memory_buffer, &linker_symbols)?;

                if let Some(bytecode_hash) = bytecode_hash {
                    let contract = OutputContract::new(
                        hex::encode(memory_buffer_linked.as_slice()),
                        hex::encode(bytecode_hash),
                    );
                    if already_linked {
                        output.ignored.insert(path.clone(), contract);
                    } else {
                        output.linked.insert(path.clone(), contract);
                    }
                }
                if memory_buffer_linked.is_elf_eravm() {
                    output.unlinked.insert(
                        path.clone(),
                        memory_buffer_linked.get_undefined_symbols_eravm(),
                    );
                }

                Ok(())
            })?;

        Ok(output)
    }
}

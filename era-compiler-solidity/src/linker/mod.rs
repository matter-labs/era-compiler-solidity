//!
//! The linker.
//!

pub mod input;
pub mod output;

use std::collections::BTreeMap;

use self::input::Input;
use self::output::contract::Contract as OutputContract;
use self::output::unlinked::Unlinked as OutputUnlinked;
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
        let mut unlinked_objects = Vec::new();
        let mut factory_dependencies = BTreeMap::new();

        for (path, bytecode_string) in input.bytecodes.into_iter() {
            let bytecode = hex::decode(
                bytecode_string
                    .strip_prefix("0x")
                    .unwrap_or(bytecode_string.as_str()),
            )
            .map_err(|error| {
                anyhow::anyhow!("Object `{path}` hexadecimal string decoding: {error}")
            })?;

            let memory_buffer = inkwell::memory_buffer::MemoryBuffer::create_from_memory_range(
                bytecode.as_slice(),
                path.as_str(),
                false,
            );

            if memory_buffer.is_elf_eravm() {
                dbg!(hex::encode(memory_buffer.as_slice()));
                unlinked_objects.push((path, memory_buffer));
            } else {
                let hash =
                    era_compiler_llvm_context::eravm_hash(&memory_buffer).map_err(|error| {
                        anyhow::anyhow!("Object `{path}` bytecode hashing: {error}")
                    })?;
                output.ignored.insert(
                    path.clone(),
                    OutputContract::new(bytecode_string, hex::encode(hash)),
                );
                factory_dependencies.insert(path, hash);
            }
        }

        loop {
            let mut linked_counter = 0;
            let mut remaining_objects = Vec::new();
            for (path, bytecode_buffer) in unlinked_objects.drain(..) {
                dbg!(hex::encode(bytecode_buffer.as_slice()));
                let (bytecode_buffer_after_linking, object_format) =
                    era_compiler_llvm_context::eravm_link(
                        bytecode_buffer,
                        &linker_symbols,
                        &factory_dependencies,
                    )?;
                match object_format {
                    era_compiler_common::ObjectFormat::ELF => {
                        remaining_objects.push((path, bytecode_buffer_after_linking));
                    }
                    era_compiler_common::ObjectFormat::Raw => {
                        let bytecode = hex::encode(bytecode_buffer_after_linking.as_slice());
                        let hash =
                            era_compiler_llvm_context::eravm_hash(&bytecode_buffer_after_linking)
                                .expect("Always valid");

                        output.linked.insert(
                            path.clone(),
                            OutputContract::new(bytecode, hex::encode(hash.as_slice())),
                        );

                        factory_dependencies.insert(path, hash);
                        linked_counter += 1;
                    }
                }
            }
            unlinked_objects = remaining_objects;
            if linked_counter == 0 {
                break;
            }
        }

        output.unlinked = unlinked_objects
            .into_iter()
            .map(|(path, bytecode_buffer)| {
                let (library_symbols, factory_dependencies) =
                    bytecode_buffer.get_undefined_references_eravm();
                let unlinked = OutputUnlinked::new(library_symbols, factory_dependencies);
                (path, unlinked)
            })
            .collect();
        Ok(output)
    }
}

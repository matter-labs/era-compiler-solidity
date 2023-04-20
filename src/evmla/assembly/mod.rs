//!
//! The `solc --asm-json` output.
//!

pub mod data;
pub mod instruction;

use std::collections::BTreeMap;
use std::collections::HashSet;

use serde::Deserialize;
use serde::Serialize;

use crate::evmla::ethereal_ir::entry_link::EntryLink;
use crate::evmla::ethereal_ir::EtherealIR;

use self::data::Data;
use self::instruction::name::Name as InstructionName;
use self::instruction::Instruction;

///
/// The JSON assembly.
///
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Assembly {
    /// The metadata string.
    #[serde(rename = ".auxdata")]
    pub auxdata: Option<String>,
    /// The deploy code instructions.
    #[serde(rename = ".code")]
    pub code: Option<Vec<Instruction>>,
    /// The runtime code.
    #[serde(rename = ".data")]
    pub data: Option<BTreeMap<String, Data>>,

    /// The full contract path.
    #[serde(skip)]
    pub full_path: Option<String>,
    /// The factory dependency paths.
    #[serde(skip)]
    pub factory_dependencies: HashSet<String>,
}

impl Assembly {
    ///
    /// Gets the contract `keccak256` hash.
    ///
    pub fn keccak256(&self) -> String {
        let json = serde_json::to_vec(self).expect("Always valid");
        compiler_llvm_context::keccak256(json.as_slice())
    }

    ///
    /// Sets the full contract path.
    ///
    pub fn set_full_path(&mut self, full_path: String) {
        self.full_path = Some(full_path);
    }

    ///
    /// Returns the full contract path if it is set, or `<undefined>` otherwise.
    ///
    /// # Panics
    /// If the `full_path` has not been set.
    ///
    pub fn full_path(&self) -> &str {
        self.full_path
            .as_deref()
            .unwrap_or_else(|| panic!("The full path of some contracts is unset"))
    }

    ///
    /// Replaces the deploy code dependencies with full contract path and returns the list.
    ///
    pub fn deploy_dependencies_pass(
        &mut self,
        full_path: &str,
        hash_data_mapping: &BTreeMap<String, String>,
    ) -> anyhow::Result<BTreeMap<String, String>> {
        let mut index_path_mapping = BTreeMap::new();
        let index = "0".repeat(compiler_common::BYTE_LENGTH_FIELD * 2);
        index_path_mapping.insert(index, full_path.to_owned());

        let dependencies = match self.data.as_mut() {
            Some(dependencies) => dependencies,
            None => return Ok(index_path_mapping),
        };
        for (index, data) in dependencies.iter_mut() {
            if index == "0" {
                continue;
            }

            let mut index_extended =
                "0".repeat(compiler_common::BYTE_LENGTH_FIELD * 2 - index.len());
            index_extended.push_str(index.as_str());

            *data = match data {
                Data::Assembly(assembly) => {
                    let hash = assembly.keccak256();
                    let full_path =
                        hash_data_mapping
                            .get(hash.as_str())
                            .cloned()
                            .ok_or_else(|| {
                                anyhow::anyhow!("Contract path not found for hash `{}`", hash)
                            })?;
                    self.factory_dependencies.insert(full_path.to_owned());

                    index_path_mapping.insert(index_extended, full_path.clone());
                    Data::Path(full_path)
                }
                Data::Hash(hash) => {
                    index_path_mapping.insert(index_extended, hash.to_owned());
                    continue;
                }
                _ => continue,
            };
        }

        Ok(index_path_mapping)
    }

    ///
    /// Replaces the runtime code dependencies with full contract path and returns the list.
    ///
    pub fn runtime_dependencies_pass(
        &mut self,
        full_path: &str,
        hash_data_mapping: &BTreeMap<String, String>,
    ) -> anyhow::Result<BTreeMap<String, String>> {
        let mut index_path_mapping = BTreeMap::new();
        let index = "0".repeat(compiler_common::BYTE_LENGTH_FIELD * 2);
        index_path_mapping.insert(index, full_path.to_owned());

        let dependencies = match self
            .data
            .as_mut()
            .and_then(|data| data.get_mut("0"))
            .and_then(|data| data.get_assembly_mut())
            .and_then(|assembly| assembly.data.as_mut())
        {
            Some(dependencies) => dependencies,
            None => return Ok(index_path_mapping),
        };
        for (index, data) in dependencies.iter_mut() {
            let mut index_extended =
                "0".repeat(compiler_common::BYTE_LENGTH_FIELD * 2 - index.len());
            index_extended.push_str(index.as_str());

            *data = match data {
                Data::Assembly(assembly) => {
                    let hash = assembly.keccak256();
                    let full_path =
                        hash_data_mapping
                            .get(hash.as_str())
                            .cloned()
                            .ok_or_else(|| {
                                anyhow::anyhow!("Contract path not found for hash `{}`", hash)
                            })?;
                    self.factory_dependencies.insert(full_path.to_owned());

                    index_path_mapping.insert(index_extended, full_path.clone());
                    Data::Path(full_path)
                }
                Data::Hash(hash) => {
                    index_path_mapping.insert(index_extended, hash.to_owned());
                    continue;
                }
                _ => continue,
            };
        }

        Ok(index_path_mapping)
    }
}

impl<D> compiler_llvm_context::WriteLLVM<D> for Assembly
where
    D: compiler_llvm_context::Dependency,
{
    fn declare(&mut self, context: &mut compiler_llvm_context::Context<D>) -> anyhow::Result<()> {
        let mut entry = compiler_llvm_context::EntryFunction::default();
        entry.declare(context)?;

        let mut runtime =
            compiler_llvm_context::Runtime::new(compiler_llvm_context::AddressSpace::Heap);
        runtime.declare(context)?;

        compiler_llvm_context::DeployCodeFunction::new(
            compiler_llvm_context::DummyLLVMWritable::default(),
        )
        .declare(context)?;
        compiler_llvm_context::RuntimeCodeFunction::new(
            compiler_llvm_context::DummyLLVMWritable::default(),
        )
        .declare(context)?;

        entry.into_llvm(context)?;

        runtime.into_llvm(context)?;

        Ok(())
    }

    fn into_llvm(self, context: &mut compiler_llvm_context::Context<D>) -> anyhow::Result<()> {
        let full_path = self.full_path().to_owned();

        if let Some(debug_config) = context.debug_config() {
            debug_config.dump_evmla(full_path.as_str(), self.to_string().as_str())?;
        }
        let deploy_code_blocks = EtherealIR::get_blocks(
            context.evmla().version.to_owned(),
            compiler_llvm_context::CodeType::Deploy,
            self.code
                .as_deref()
                .ok_or_else(|| anyhow::anyhow!("Deploy code instructions not found"))?,
        )?;

        let data = self
            .data
            .ok_or_else(|| anyhow::anyhow!("Runtime code data not found"))?
            .remove("0")
            .expect("Always exists");
        if let Some(debug_config) = context.debug_config() {
            debug_config.dump_evmla(full_path.as_str(), data.to_string().as_str())?;
        }
        let runtime_code_instructions = match data {
            Data::Assembly(assembly) => assembly
                .code
                .ok_or_else(|| anyhow::anyhow!("Runtime code instructions not found"))?,
            Data::Hash(hash) => {
                anyhow::bail!("Expected runtime code instructions, found hash `{}`", hash)
            }
            Data::Path(path) => {
                anyhow::bail!("Expected runtime code instructions, found path `{}`", path)
            }
        };
        let runtime_code_blocks = EtherealIR::get_blocks(
            context.evmla().version.to_owned(),
            compiler_llvm_context::CodeType::Runtime,
            runtime_code_instructions.as_slice(),
        )?;

        let mut blocks = deploy_code_blocks;
        blocks.extend(runtime_code_blocks);
        let mut ethereal_ir = EtherealIR::new(context.evmla().version.to_owned(), blocks)?;
        if let Some(debug_config) = context.debug_config() {
            debug_config.dump_ethir(full_path.as_str(), ethereal_ir.to_string().as_str())?;
        }
        ethereal_ir.declare(context)?;
        ethereal_ir.into_llvm(context)?;

        compiler_llvm_context::DeployCodeFunction::new(EntryLink::new(
            compiler_llvm_context::CodeType::Deploy,
        ))
        .into_llvm(context)?;
        compiler_llvm_context::RuntimeCodeFunction::new(EntryLink::new(
            compiler_llvm_context::CodeType::Runtime,
        ))
        .into_llvm(context)?;

        Ok(())
    }
}

impl std::fmt::Display for Assembly {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(instructions) = self.code.as_ref() {
            for (index, instruction) in instructions.iter().enumerate() {
                match instruction.name {
                    InstructionName::Tag => writeln!(f, "{index:03} {instruction}")?,
                    _ => writeln!(f, "{index:03}     {instruction}")?,
                }
            }
        }

        Ok(())
    }
}

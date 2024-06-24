//!
//! Kind of a [`DefinitionInfo`].
//!

use crate::easycrypt::syntax::definition::Definition;
use crate::easycrypt::syntax::proc::name::ProcName;
use crate::easycrypt::syntax::r#type::context_kind::ContextKind;
use crate::easycrypt::syntax::r#type::Type;
use crate::easycrypt::translator::definition_info::attributes::Attributes;
use crate::easycrypt::translator::definition_info::DefinitionInfo;
use crate::yul::path::full_name::FullName;
use crate::yul::path::Path;

use super::Kind;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProcKind {
    pub name: ProcName,

    pub attributes: Attributes,
}

pub fn formal_state_parameter_name(kind: &ContextKind) -> &'static str {
    match kind {
        ContextKind::Memory => "state_memory",
        ContextKind::Storage => "state_storage",
        ContextKind::TransientStorage => "state_transient",
        ContextKind::Other => "state_other",
    }
}

impl ProcKind {
    pub fn get_state_param_definition(&self, kind: ContextKind, path: &Path) -> Option<Definition> {
        let definition = Some(Definition {
            identifier: formal_state_parameter_name(&kind).to_string(),
            location: Some(path.clone()),
            r#type: Some(Type::Context(kind.clone())),
        });
        match kind {
            ContextKind::Memory if self.attributes.heap_user.needs_read_access() => definition,
            ContextKind::Storage if self.attributes.storage_user.needs_read_access() => definition,
            ContextKind::TransientStorage if self.attributes.transient_user.needs_read_access() => {
                definition
            }
            ContextKind::Other if self.attributes.transient_user.needs_read_access() => definition,
            _ => None,
        }
    }
    pub fn get_state_return_var_definition(
        &self,
        kind: ContextKind,
        path: &Path,
    ) -> Option<Definition> {
        let definition = Some(Definition {
            identifier: formal_state_parameter_name(&kind).to_string(),
            location: Some(path.clone()),
            r#type: Some(Type::Context(kind.clone())),
        });
        match kind {
            ContextKind::Memory if self.attributes.heap_user.needs_write_access() => definition,
            ContextKind::Storage if self.attributes.storage_user.needs_write_access() => definition,
            ContextKind::TransientStorage
                if self.attributes.transient_user.needs_write_access() =>
            {
                definition
            }
            ContextKind::Other if self.attributes.transient_user.needs_write_access() => definition,
            _ => None,
        }
    }
}

pub fn state_formal_parameters(proc_definition: &DefinitionInfo) -> Vec<(Definition, Type)> {
    if let DefinitionInfo {
        kind: Kind::Proc(proc_kind),
        yul_name: FullName { path, .. },
        ..
    } = proc_definition
    {
        ContextKind::ALL_KINDS
            .iter()
            .flat_map(|state_kind| {
                let definition = proc_kind.get_state_param_definition(state_kind.clone(), path)?;
                Some((definition, Type::Context(state_kind.clone())))
            })
            .collect()
    } else {
        Vec::new()
    }
}

pub fn state_return_vars(proc_definition: &DefinitionInfo) -> Vec<(Definition, Type)> {
    if let DefinitionInfo {
        kind: Kind::Proc(proc_kind),
        yul_name: FullName { path, .. },
        ..
    } = proc_definition
    {
        ContextKind::ALL_KINDS
            .iter()
            .flat_map(|state_kind| {
                let definition =
                    proc_kind.get_state_return_var_definition(state_kind.clone(), path)?;
                Some((definition, Type::Context(state_kind.clone())))
            })
            .collect()
    } else {
        Vec::new()
    }
}

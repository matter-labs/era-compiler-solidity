use crate::easycrypt::syntax::function::Function;
use crate::easycrypt::syntax::proc::Proc;
use crate::easycrypt::syntax::reference::Reference;

///
/// Top-level definition in an EasyCrypt module.
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TopDefinition {
    Proc(Proc),
    Function(Function),
}

impl TopDefinition {
    ///
    /// Create a reference to this definition.
    ///
    pub fn reference(&self) -> Reference {
        match self {
            TopDefinition::Proc(proc) => Reference {
                identifier: proc.name.to_string(),
                location: proc.location.clone(),
            },
            TopDefinition::Function(fun) => Reference {
                identifier: fun.name.to_string(),
                location: fun.location.clone(),
            },
        }
    }

    ///
    /// Returns `true` if the module definition is [`ProcDef`].
    ///
    /// [`ProcDef`]: ModuleDefinition::ProcDef
    ///
    #[must_use]
    pub fn is_proc_def(&self) -> bool {
        matches!(self, Self::Proc(..))
    }

    ///
    /// Returns `true` if the module definition is [`FunDef`].
    ///
    /// [`FunDef`]: ModuleDefinition::FunDef
    ///
    #[must_use]
    pub fn is_fun_def(&self) -> bool {
        matches!(self, Self::Function(..))
    }
}

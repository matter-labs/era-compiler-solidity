use std::fmt::Display;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum ContextKind {
    Memory,
    Storage,
    TransientStorage,
    Other,
}

impl Display for ContextKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ContextKind::Memory => "mem",
            ContextKind::Storage => "storage",
            ContextKind::TransientStorage => "transient_storage",
            ContextKind::Other => "context",
        })
    }
}

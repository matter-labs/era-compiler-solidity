//!
//! How is memory or storage affected by a procedure.
//!

#[allow(dead_code)]
/// How is memory or storage affected by a procedure.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Usage {
    pub read: bool,
    pub write: bool,
    pub meta: bool,
}

impl Usage {
    pub const READ: Usage = Usage {
        read: true,
        write: false,
        meta: false,
    };
    pub const WRITE: Usage = Usage {
        read: false,
        write: true,
        meta: false,
    };
    pub const META: Usage = Usage {
        read: false,
        write: false,
        meta: true,
    };
    pub const RW: Usage = Usage {
        read: true,
        write: true,
        meta: false,
    };

    pub fn union(&self, other: &Self) -> Self {
        Self {
            read: self.read || other.read,
            write: self.write || other.write,
            meta: self.meta || other.meta,
        }
    }
}

use core::fmt;

use serde::{Deserialize, Serialize};

/// The zero-based index of a block in the blockchain.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct BlockNo(pub u64);

impl BlockNo {
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}

impl fmt::Debug for BlockNo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BlockNo({})", self.0)
    }
}

impl fmt::Display for BlockNo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u64> for BlockNo {
    fn from(value: u64) -> Self {
        BlockNo(value)
    }
}

impl From<BlockNo> for u64 {
    fn from(value: BlockNo) -> Self {
        value.0
    }
}

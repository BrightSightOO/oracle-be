use borsh::{BorshDeserialize, BorshSerialize};
use borsh_size::BorshSize;

use crate::error::OracleError;

/// Basis points calculator.
///
/// 1 basis point is equivalent to 0.01%.
#[derive(Clone, Copy, BorshDeserialize, BorshSerialize, BorshSize)]
pub struct Bps {
    bps: u16,
}

impl Bps {
    /// The maximum basis points value, represents 100%.
    pub const MAX: u16 = 10_000;

    #[inline]
    pub const fn new(bps: u16) -> Option<Bps> {
        if bps > Bps::MAX {
            return None;
        }
        Some(Bps { bps })
    }

    #[inline]
    pub const fn get(self) -> u16 {
        self.bps
    }

    #[inline]
    pub const fn calculate(&self, amount: u64) -> u64 {
        let mut n = amount as u128;

        n *= self.get() as u128;
        n /= Bps::MAX as u128;

        n as u64
    }
}

impl TryFrom<u16> for Bps {
    type Error = OracleError;

    #[inline]
    fn try_from(bps: u16) -> Result<Self, Self::Error> {
        Bps::new(bps).ok_or(OracleError::InvalidBps)
    }
}

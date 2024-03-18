use solana_program::pubkey::{Pubkey, PUBKEY_BYTES};

use crate::syscalls;

/// Checks two pubkeys for equality in a computationally cheap way using `sol_memcmp`.
#[inline]
pub fn cmp_pubkeys(a: &Pubkey, b: &Pubkey) -> bool {
    let a: &[u8] = a.as_ref();
    let b: &[u8] = b.as_ref();
    // SAFETY: `a` and `b` are valid for reads of `PUBKEY_BYTES` bytes.
    unsafe { syscalls::memcmp(a.as_ptr(), b.as_ptr(), PUBKEY_BYTES) == 0 }
}

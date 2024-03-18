/// Copies `count` bytes from `src` to `dst`. The source and destination must
/// *not* overlap.
///
/// # Safety
///
/// Behavior is undefined if any of the following conditions are violated:
///
/// * `src` must be valid for reads of `count` bytes.
///
/// * `dst` must be valid for writes of `count` bytes.
///
/// * Both `src` and `dst` must be properly aligned.
///
/// * The region of memory beginning at `src` with a size of `count` bytes must
///   *not* overlap with the region of memory beginning at `dst` with the same size.
///
/// Note that even if `count` is `0`, the pointers must be non-null and properly aligned.
#[inline]
pub unsafe fn memcpy(dst: *mut u8, src: *const u8, count: usize) {
    #[cfg(target_os = "solana")]
    solana_program::syscalls::sol_memcpy_(dst, src, count as u64);

    #[cfg(not(target_os = "solana"))]
    std::ptr::copy_nonoverlapping(src, dst, count);
}

/// Copies `count` bytes from `src` to `dst`. The source and destination may overlap.
///
/// # Safety
///
/// Behavior is undefined if any of the following conditions are violated:
///
/// * `src` must be valid for reads of `count` bytes, and must remain valid even
///   when `dst` is written for `count` bytes. (This means if the memory ranges
///   overlap, the two pointers must not be subject to aliasing restrictions
///   relative to each other.)
///
/// * `dst` must be valid for writes of `count` bytes, and must remain valid even
///   when `src` is read for `count` bytes.
///
/// * Both `src` and `dst` must be properly aligned.
///
/// Note that even if `count` is `0`, the pointers must be non-null and properly aligned.
#[inline]
pub unsafe fn memmove(dst: *mut u8, src: *const u8, count: usize) {
    #[cfg(target_os = "solana")]
    solana_program::syscalls::sol_memmove_(dst, src, count as u64);

    #[cfg(not(target_os = "solana"))]
    std::ptr::copy(src, dst, count);
}

/// Lexicographically compares the first `count` bytes of `left` and `right`.
///
/// # Safety
///
/// Behavior is undefined if any of the following conditions are violated:
///
/// * `left` must be valid for reads of `count` bytes.
///
/// * `right` must be valid for reads of `count` bytes.
///
/// * Both `left` and `right` must be properly aligned.
///
/// Note that even if `count` is `0`, the pointers must be non-null and properly aligned.
#[inline]
pub unsafe fn memcmp(left: *const u8, right: *const u8, count: usize) -> i32 {
    #[cfg(target_os = "solana")]
    {
        let mut result = std::mem::MaybeUninit::uninit();
        solana_program::syscalls::sol_memcmp_(left, right, count as u64, result.as_mut_ptr());
        result.assume_init()
    }
    #[cfg(not(target_os = "solana"))]
    {
        let left = std::slice::from_raw_parts(left, count);
        let right = std::slice::from_raw_parts(right, count);
        left.cmp(right) as i32
    }
}

/// Sets `count` bytes of memory starting at `dst` to `val`.
///
/// # Safety
///
/// Behavior is undefined if any of the following conditions are violated:
///
/// * `dst` must be [valid] for writes of `count` bytes.
///
/// * `dst` must be properly aligned.
///
/// Note that even if `count` is `0`, the pointer must be non-null and properly aligned.
#[inline]
pub unsafe fn memset(dst: *mut u8, val: u8, count: usize) {
    #[cfg(target_os = "solana")]
    solana_program::syscalls::sol_memset_(dst, val, count as u64);

    #[cfg(not(target_os = "solana"))]
    std::ptr::write_bytes(dst, val, count);
}

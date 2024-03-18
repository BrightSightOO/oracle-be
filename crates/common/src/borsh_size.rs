use std::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroU128, NonZeroU16,
    NonZeroU32, NonZeroU64, NonZeroU8,
};

use solana_program::pubkey::{Pubkey, PUBKEY_BYTES};

pub trait BorshSize {
    /// The minimum size in bytes required to borsh serialize any form of the type.
    const SIZE: usize;
}

macro_rules! impl_sized {
    ($($ty:ty),* $(,)?) => {
        $(
            impl BorshSize for $ty {
                const SIZE: usize = std::mem::size_of::<$ty>();
            }
        )*
    };
}

impl_sized!(bool);
impl_sized!(char);

impl_sized!(f32, f64);
impl_sized!(u8, u16, u32, u64, u128);
impl_sized!(i8, i16, i32, i64, i128);

impl_sized!(NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128);
impl_sized!(NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128);

impl BorshSize for Pubkey {
    const SIZE: usize = PUBKEY_BYTES;
}

impl<T> BorshSize for Option<T>
where
    T: BorshSize,
{
    const SIZE: usize = 1 + T::SIZE;
}

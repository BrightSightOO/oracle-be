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

impl<T, const N: usize> BorshSize for [T; N]
where
    T: BorshSize,
{
    const SIZE: usize = N * T::SIZE;
}

impl BorshSize for Pubkey {
    const SIZE: usize = PUBKEY_BYTES;
}

impl<T> BorshSize for Option<T>
where
    T: BorshSize,
{
    const SIZE: usize = 1 + T::SIZE;
}

impl<T, E> BorshSize for Result<T, E>
where
    T: BorshSize,
    E: BorshSize,
{
    const SIZE: usize = 1 + max(T::SIZE, E::SIZE);
}

const fn max(x: usize, y: usize) -> usize {
    if x >= y { x } else { y }
}

macro_rules! tuple_impl {
    (
        @__stack [$($t:ident)*]
        @__rest []
    ) => {
        impl<$($t),*> BorshSize for ($($t,)*)
        where
            $($t: BorshSize,)*
        {
            const SIZE: usize = 0 $(+ $t::SIZE)*;
        }
    };
    (
        @__stack [$($t:ident)*]
        @__rest [$next:ident $($rest:ident)*]
    ) => {
        impl<$($t),*> BorshSize for ($($t,)*)
        where
            $($t: BorshSize,)*
        {
            const SIZE: usize = 0 $(+ $t::SIZE)*;
        }

        tuple_impl! {
            @__stack [$($t)* $next]
            @__rest [$($rest)*]
        }
    };
    ($($t:ident)*) => {
        tuple_impl! {
            @__stack []
            @__rest [$($t)*]
        }
    };
}

tuple_impl! {
     _1  _2  _3  _4  _5  _6  _7  _8  _9 _10
    _11 _12 _13 _14 _15 _16 _17 _18 _19 _20
    _21 _22 _23 _24 _25 _26 _27 _28 _29 _30
    _31 _32
}

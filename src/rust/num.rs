use rust::lang::*;

pub trait One: Mul<Self, Self> {
    fn one() -> Self;
}

macro_rules! one_impl(
    ($t:ty, $v:expr) => {
        impl One for $t {
            #[inline]
            fn one() -> $t { $v }
        }
    }
)

one_impl!(uint, 1u)
one_impl!(u8,  1u8)
one_impl!(u16, 1u16)
one_impl!(u32, 1u32)
one_impl!(u64, 1u64)

one_impl!(int, 1i)
one_impl!(i8,  1i8)
one_impl!(i16, 1i16)
one_impl!(i32, 1i32)
one_impl!(i64, 1i64)

one_impl!(f32, 1.0f32)
one_impl!(f64, 1.0f64)

macro_rules! trait_impl(
    ($name:ident for $($t:ty)*) => ($(
        impl $name for $t {}
    )*)
)

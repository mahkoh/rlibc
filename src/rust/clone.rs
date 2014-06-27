pub trait Clone {
    fn clone(&self) -> Self;

    #[inline(always)]
    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }
}

macro_rules! clone_impl(
    ($t:ty) => {
        impl Clone for $t {
            /// Return a deep copy of the value.
            #[inline]
            fn clone(&self) -> $t { *self }
        }
    }
)

clone_impl!(int)
clone_impl!(i8)
clone_impl!(i16)
clone_impl!(i32)
clone_impl!(i64)

clone_impl!(uint)
clone_impl!(u8)
clone_impl!(u16)
clone_impl!(u32)
clone_impl!(u64)

clone_impl!(f32)
clone_impl!(f64)

clone_impl!(())
clone_impl!(bool)
clone_impl!(char)

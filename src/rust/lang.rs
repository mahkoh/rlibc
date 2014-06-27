#[lang="ord"]
pub trait PartialOrd: PartialEq {
    fn lt(&self, other: &Self) -> bool;
    #[inline]
    fn le(&self, other: &Self) -> bool { !other.lt(self) }
    #[inline]
    fn gt(&self, other: &Self) -> bool {  other.lt(self) }
    #[inline]
    fn ge(&self, other: &Self) -> bool { !self.lt(other) }
}

macro_rules! ord_impl(
    ($($t:ty)*) => ($(
        impl PartialOrd for $t {
            #[inline]
            fn lt(&self, other: &$t) -> bool { (*self) < (*other) }
            #[inline]
            fn le(&self, other: &$t) -> bool { (*self) <= (*other) }
            #[inline]
            fn ge(&self, other: &$t) -> bool { (*self) >= (*other) }
            #[inline]
            fn gt(&self, other: &$t) -> bool { (*self) > (*other) }
        }
    )*)
)

ord_impl!(char uint u8 u16 u32 u64 int i8 i16 i32 i64 f32 f64)

#[lang="eq"]
pub trait PartialEq {
    fn eq(&self, other: &Self) -> bool;
    #[inline]
    fn ne(&self, other: &Self) -> bool { !self.eq(other) }
}

macro_rules! eq_impl(
    ($($t:ty)*) => ($(
        impl PartialEq for $t {
            #[inline]
            fn eq(&self, other: &$t) -> bool { (*self) == (*other) }
            #[inline]
            fn ne(&self, other: &$t) -> bool { (*self) != (*other) }
        }
    )*)
)

impl PartialEq for () {
    #[inline]
    fn eq(&self, _other: &()) -> bool { true }
    #[inline]
    fn ne(&self, _other: &()) -> bool { false }
}

eq_impl!(bool char uint u8 u16 u32 u64 int i8 i16 i32 i64 f32 f64)

#[lang="add"]
pub trait Add<RHS,Result> {
    fn add(&self, rhs: &RHS) -> Result;
}

macro_rules! add_impl(
    ($($t:ty)*) => ($(
        #[cfg(not(test))]
        impl Add<$t, $t> for $t {
            #[inline]
            fn add(&self, other: &$t) -> $t { (*self) + (*other) }
        }
    )*)
)

add_impl!(uint u8 u16 u32 u64 int i8 i16 i32 i64 f32 f64)

#[lang="drop"]
pub trait Drop {
    fn drop(&mut self);
}

#[lang="sub"]
pub trait Sub<RHS,Result> {
    fn sub(&self, rhs: &RHS) -> Result;
}

macro_rules! sub_impl(
    ($($t:ty)*) => ($(
        #[cfg(not(test))]
        impl Sub<$t, $t> for $t {
            #[inline]
            fn sub(&self, other: &$t) -> $t { (*self) - (*other) }
        }
    )*)
)

sub_impl!(uint u8 u16 u32 u64 int i8 i16 i32 i64 f32 f64)

#[lang="mul"]
pub trait Mul<RHS,Result> {
    fn mul(&self, rhs: &RHS) -> Result;
}

macro_rules! mul_impl(
    ($($t:ty)*) => ($(
        #[cfg(not(test))]
        impl Mul<$t, $t> for $t {
            #[inline]
            fn mul(&self, other: &$t) -> $t { (*self) * (*other) }
        }
    )*)
)

mul_impl!(uint u8 u16 u32 u64 int i8 i16 i32 i64 f32 f64)

#[lang="div"]
pub trait Div<RHS,Result> {
    fn div(&self, rhs: &RHS) -> Result;
}

#[lang="rem"]
pub trait Rem<RHS,Result> {
    fn rem(&self, rhs: &RHS) -> Result;
}

#[lang="neg"]
pub trait Neg<Result> {
    fn neg(&self) -> Result;
}

#[lang="not"]
pub trait Not<Result> {
    fn not(&self) -> Result;
}

#[lang="bitand"]
pub trait BitAnd<RHS,Result> {
    fn bitand(&self, rhs: &RHS) -> Result;
}

#[lang="bitor"]
pub trait BitOr<RHS,Result> {
    fn bitor(&self, rhs: &RHS) -> Result;
}

#[lang="bitxor"]
pub trait BitXor<RHS,Result> {
    fn bitxor(&self, rhs: &RHS) -> Result;
}

#[lang="shl"]
pub trait Shl<RHS,Result> {
    fn shl(&self, rhs: &RHS) -> Result;
}

#[lang="shr"]
pub trait Shr<RHS,Result> {
    fn shr(&self, rhs: &RHS) -> Result;
}

#[lang="index"]
pub trait Index<Index,Result> {
    fn index(&self, index: &Index) -> Result;
}

#[lang="deref"]
pub trait Deref<Result> {
    fn deref<'a>(&'a self) -> &'a Result;
}

#[lang="deref_mut"]
pub trait DerefMut<Result>: Deref<Result> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut Result;
}

#[lang="sized"]
pub trait Sized { }



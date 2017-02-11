use std;
use ::*;

#[repr(C)]
#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct long2(pub i64, pub i64);

extern "platform-intrinsic" {
  fn simd_add<T>(x: T, y: T) -> T;
  fn simd_sub<T>(x: T, y: T) -> T;
  fn simd_mul<T>(x: T, y: T) -> T;
  fn simd_div<T>(x: T, y: T) -> T;

  fn simd_shl<T>(x: T, y: T) -> T;
  fn simd_shr<T>(x: T, y: T) -> T;

  fn simd_and<T>(x: T, y: T) -> T;
  fn simd_or<T>(x: T, y: T) -> T;
  fn simd_xor<T>(x: T, y: T) -> T;

  fn simd_eq<T, U>(x: T, y: T) -> U;
  fn simd_ne<T, U>(x: T, y: T) -> U;
  fn simd_lt<T, U>(x: T, y: T) -> U;
  fn simd_le<T, U>(x: T, y: T) -> U;
  fn simd_gt<T, U>(x: T, y: T) -> U;
  fn simd_ge<T, U>(x: T, y: T) -> U;

  fn simd_cast<T, U>(x: T) -> U;

  fn simd_insert<T, E>(x: T, i: u32, e: E) -> T;
  fn simd_extract<T, E>(x: T, i: u32) -> E;
}

impl std::ops::Index<u32> for long2 {
  type Output = i64;

  #[inline]
  fn index(&self, index: u32) -> &i64 {
    return unsafe { simd_extract(self, index) };
  }
}

impl std::ops::Add for long2 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return unsafe { simd_add(self, other) };
  }
}

impl std::ops::Add<i64> for long2 {
  type Output = Self;

  #[inline]
  fn add(self, other: i64) -> Self {
    return unsafe { simd_add(self, long2::broadcast(other)) };
  }
}

impl std::ops::Add<long2> for i64 {
  type Output = long2;

  #[inline]
  fn add(self, other: long2) -> long2 {
    return unsafe { simd_add(long2::broadcast(self), other) };
  }
}

impl std::ops::Sub for long2 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return unsafe { simd_sub(self, other) };
  }
}

impl std::ops::Sub<i64> for long2 {
  type Output = Self;

  #[inline]
  fn sub(self, other: i64) -> Self {
    return unsafe { simd_sub(self, long2::broadcast(other)) };
  }
}

impl std::ops::Sub<long2> for i64 {
  type Output = long2;

  #[inline]
  fn sub(self, other: long2) -> long2 {
    return unsafe { simd_sub(long2::broadcast(self), other) };
  }
}

impl std::ops::Mul for long2 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return unsafe { simd_mul(self, other) };
  }
}

impl std::ops::Mul<i64> for long2 {
  type Output = Self;

  #[inline]
  fn mul(self, other: i64) -> Self {
    return unsafe { simd_mul(self, long2::broadcast(other)) };
  }
}

impl std::ops::Mul<long2> for i64 {
  type Output = long2;

  #[inline]
  fn mul(self, other: long2) -> long2 {
    return unsafe { simd_mul(long2::broadcast(self), other) };
  }
}

impl std::ops::Div for long2 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return unsafe { simd_div(self, other) };
  }
}

impl std::ops::Div<i64> for long2 {
  type Output = Self;

  #[inline]
  fn div(self, other: i64) -> Self {
    return unsafe { simd_div(self, long2::broadcast(other)) };
  }
}

impl std::ops::Div<long2> for i64 {
  type Output = long2;

  #[inline]
  fn div(self, other: long2) -> long2 {
    return unsafe { simd_div(long2::broadcast(self), other) };
  }
}

impl std::ops::BitAnd for long2 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: Self) -> Self {
    return unsafe { simd_and(self, other) };
  }
}

impl std::ops::BitAnd<i64> for long2 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: i64) -> Self {
    return unsafe { simd_and(self, long2::broadcast(other)) };
  }
}

impl std::ops::BitAnd<long2> for i64 {
  type Output = long2;

  #[inline]
  fn bitand(self, other: long2) -> long2 {
    return unsafe { simd_and(long2::broadcast(self), other) };
  }
}

impl std::ops::BitOr for long2 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: Self) -> Self {
    return unsafe { simd_or(self, other) };
  }
}

impl std::ops::BitOr<i64> for long2 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: i64) -> Self {
    return unsafe { simd_or(self, long2::broadcast(other)) };
  }
}

impl std::ops::BitOr<long2> for i64 {
  type Output = long2;

  #[inline]
  fn bitor(self, other: long2) -> long2 {
    return unsafe { simd_or(long2::broadcast(self), other) };
  }
}

impl std::ops::BitXor for long2 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: Self) -> Self {
    return unsafe { simd_xor(self, other) };
  }
}

impl std::ops::BitXor<i64> for long2 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: i64) -> Self {
    return unsafe { simd_xor(self, long2::broadcast(other)) };
  }
}

impl std::ops::BitXor<long2> for i64 {
  type Output = long2;

  #[inline]
  fn bitxor(self, other: long2) -> long2 {
    return unsafe { simd_xor(long2::broadcast(self), other) };
  }
}

impl std::ops::Shl<long2> for long2 {
  type Output = Self;

  #[inline]
  fn shl(self, other: Self) -> Self {
    return unsafe { simd_shl(self, other) };
  }
}

impl std::ops::Shl<i64> for long2 {
  type Output = Self;

  #[inline]
  fn shl(self, other: i64) -> Self {
    return unsafe { simd_shl(self, long2::broadcast(other)) };
  }
}

impl std::ops::Shl<long2> for i64 {
  type Output = long2;

  #[inline]
  fn shl(self, other: long2) -> long2 {
    return unsafe { simd_shl(long2::broadcast(self), other) };
  }
}

impl std::ops::Shr<long2> for long2 {
  type Output = Self;

  #[inline]
  fn shr(self, other: Self) -> Self {
    return unsafe { simd_shr(self, other) };
  }
}

impl std::ops::Shr<i64> for long2 {
  type Output = Self;

  #[inline]
  fn shr(self, other: i64) -> Self {
    return unsafe { simd_shr(self, long2::broadcast(other)) };
  }
}

impl std::ops::Shr<long2> for i64 {
  type Output = long2;

  #[inline]
  fn shr(self, other: long2) -> long2 {
    return unsafe { simd_shr(long2::broadcast(self), other) };
  }
}

impl std::ops::Not for long2 {
  type Output = Self;

  #[inline]
  fn not(self) -> Self {
    return self ^ -1;
  }
}

impl PartialEq for long2 {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    return simd::all(long2::eq(*self, *other));
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return simd::all(long2::ne(*self, *other));
  }
}

impl simd::Vector for long2 {
}

impl simd::Dot for long2 {
  type Output = i64;

  #[inline]
  fn dot(self, other: long2) -> i64 {
    return long2::reduce_add(self * other);
  }
}

impl simd::Logic for long2 {
  #[inline(always)]
  fn all(self) -> bool {
    return (self.0 & self.1) & std::i64::MIN != 0;
  }

  #[inline(always)]
  fn any(self) -> bool {
    return (self.0 | self.1) & std::i64::MIN != 0;
  }
}

impl simd::Select<long2> for long2 {
  #[inline(always)]
  fn select(self, a: long2, b: long2) -> long2 {
    return (self >> 63).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: long2, b: long2) -> long2 {
    return (x & !z) | (y & z);
  }
}

impl simd::Select<ulong2> for long2 {
  #[inline(always)]
  fn select(self, a: ulong2, b: ulong2) -> ulong2 {
    return (self >> 63).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: ulong2, b: ulong2) -> ulong2 {
    return ulong2::bitcast(self.bitselect(long2::bitcast(x), long2::bitcast(y)));
  }
}

impl simd::Select<double2> for long2 {
  #[inline(always)]
  fn select(self, a: double2, b: double2) -> double2 {
    return (self >> 63).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: double2, b: double2) -> double2 {
    return double2::bitcast(self.bitselect(long2::bitcast(x), long2::bitcast(y)));
  }
}

impl long2 {
  #[inline]
  pub fn bitcast<T>(x: T) -> long2 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: i64) -> long2 {
    return long2(x, x);
  }

  #[inline]
  pub fn extract(self, i: u32) -> i64 {
    return unsafe { simd_extract(self, i) };
  }

  #[inline]
  pub fn replace(self, i: u32, x: i64) -> long2 {
    return unsafe { simd_insert(self, i, x) };
  }

  #[inline]
  pub fn eq(x: long2, y: long2) -> long2 {
    return unsafe { simd_eq(x, y) };
  }

  #[inline]
  pub fn ne(x: long2, y: long2) -> long2 {
    return unsafe { simd_ne(x, y) };
  }

  #[inline]
  pub fn lt(x: long2, y: long2) -> long2 {
    return unsafe { simd_lt(x, y) };
  }

  #[inline]
  pub fn le(x: long2, y: long2) -> long2 {
    return unsafe { simd_le(x, y) };
  }

  #[inline]
  pub fn gt(x: long2, y: long2) -> long2 {
    return unsafe { simd_gt(x, y) };
  }

  #[inline]
  pub fn ge(x: long2, y: long2) -> long2 {
    return unsafe { simd_ge(x, y) };
  }

  #[inline]
  pub fn madd(x: long2, y: long2, z: long2) -> long2 {
    return x * y + z;
  }

  #[inline]
  pub fn abs(x: long2) -> long2 {
    let mask = x >> 63;
    return (x ^ mask) - mask;
  }

  #[inline]
  pub fn max(x: long2, y: long2) -> long2 {
    return long2::bitselect(x, y, long2::gt(y, x));
  }

  #[inline]
  pub fn min(x: long2, y: long2) -> long2 {
    return long2::bitselect(x, y, long2::lt(y, x));
  }

  #[inline]
  pub fn clamp(x: long2, min: long2, max: long2) -> long2 {
    return long2::min(long2::max(x, min), max);
  }

  #[inline]
  pub fn reduce_add(x: long2) -> i64 {
    return x.0 + x.1;
  }

  #[inline]
  pub fn reduce_min(x: long2) -> i64 {
    return std::cmp::min(x.0, x.1);
  }

  #[inline]
  pub fn reduce_max(x: long2) -> i64 {
    return std::cmp::max(x.0, x.1);
  }

  #[inline]
  pub fn to_char(x: long2) -> char2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_char_sat(x: long2) -> char2 {
    return long2::to_char(long2::clamp(x, long2::broadcast(std::i8::MIN as i64), long2::broadcast(std::i8::MAX as i64)));
  }

  #[inline]
  pub fn to_uchar(x: long2) -> uchar2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_uchar_sat(x: long2) -> uchar2 {
    return long2::to_uchar(long2::clamp(x, long2::broadcast(std::u8::MIN as i64), long2::broadcast(std::u8::MAX as i64)));
  }

  #[inline]
  pub fn to_short(x: long2) -> short2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_short_sat(x: long2) -> short2 {
    return long2::to_short(long2::clamp(x, long2::broadcast(std::i16::MIN as i64), long2::broadcast(std::i16::MAX as i64)));
  }

  #[inline]
  pub fn to_ushort(x: long2) -> ushort2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_ushort_sat(x: long2) -> ushort2 {
    return long2::to_ushort(long2::clamp(x, long2::broadcast(std::u16::MIN as i64), long2::broadcast(std::u16::MAX as i64)));
  }

  #[inline]
  pub fn to_int(x: long2) -> int2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_int_sat(x: long2) -> int2 {
    return long2::to_int(long2::clamp(x, long2::broadcast(std::i32::MIN as i64), long2::broadcast(std::i32::MAX as i64)));
  }

  #[inline]
  pub fn to_uint(x: long2) -> uint2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_uint_sat(x: long2) -> uint2 {
    return long2::to_uint(long2::clamp(x, long2::broadcast(std::u32::MIN as i64), long2::broadcast(std::u32::MAX as i64)));
  }

  #[inline]
  pub fn to_float(x: long2) -> float2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_long(x: long2) -> long2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_long_sat(x: long2) -> long2 {
    return x;
  }

  #[inline]
  pub fn to_ulong(x: long2) -> ulong2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_ulong_sat(x: long2) -> ulong2 {
    return long2::to_ulong(long2::max(x, long2::broadcast(0)));
  }

  #[inline]
  pub fn to_double(x: long2) -> double2 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn lo(self) -> i64 {
    return self.0;
  }

  #[inline]
  pub fn hi(self) -> i64 {
    return self.1;
  }

  #[inline]
  pub fn odd(self) -> i64 {
    return self.1;
  }

  #[inline]
  pub fn even(self) -> i64 {
    return self.0;
  }
}

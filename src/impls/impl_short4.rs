use std;
use ::*;

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

  fn simd_cast<T, U>(x: T) -> U;
}

impl std::ops::Add for short4 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return unsafe { simd_add(self, other) };
  }
}

impl std::ops::Add<i16> for short4 {
  type Output = Self;

  #[inline]
  fn add(self, other: i16) -> Self {
    return unsafe { simd_add(self, short4::broadcast(other)) };
  }
}

impl std::ops::Add<short4> for i16 {
  type Output = short4;

  #[inline]
  fn add(self, other: short4) -> short4 {
    return unsafe { simd_add(short4::broadcast(self), other) };
  }
}

impl std::ops::Sub for short4 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return unsafe { simd_sub(self, other) };
  }
}

impl std::ops::Sub<i16> for short4 {
  type Output = Self;

  #[inline]
  fn sub(self, other: i16) -> Self {
    return unsafe { simd_sub(self, short4::broadcast(other)) };
  }
}

impl std::ops::Sub<short4> for i16 {
  type Output = short4;

  #[inline]
  fn sub(self, other: short4) -> short4 {
    return unsafe { simd_sub(short4::broadcast(self), other) };
  }
}

impl std::ops::Mul for short4 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return unsafe { simd_mul(self, other) };
  }
}

impl std::ops::Mul<i16> for short4 {
  type Output = Self;

  #[inline]
  fn mul(self, other: i16) -> Self {
    return unsafe { simd_mul(self, short4::broadcast(other)) };
  }
}

impl std::ops::Mul<short4> for i16 {
  type Output = short4;

  #[inline]
  fn mul(self, other: short4) -> short4 {
    return unsafe { simd_mul(short4::broadcast(self), other) };
  }
}

impl std::ops::Div for short4 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return unsafe { simd_div(self, other) };
  }
}

impl std::ops::Div<i16> for short4 {
  type Output = Self;

  #[inline]
  fn div(self, other: i16) -> Self {
    return unsafe { simd_div(self, short4::broadcast(other)) };
  }
}

impl std::ops::Div<short4> for i16 {
  type Output = short4;

  #[inline]
  fn div(self, other: short4) -> short4 {
    return unsafe { simd_div(short4::broadcast(self), other) };
  }
}

impl std::ops::BitAnd for short4 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: Self) -> Self {
    return unsafe { simd_and(self, other) };
  }
}

impl std::ops::BitAnd<i16> for short4 {
  type Output = Self;

  #[inline]
  fn bitand(self, other: i16) -> Self {
    return unsafe { simd_and(self, short4::broadcast(other)) };
  }
}

impl std::ops::BitAnd<short4> for i16 {
  type Output = short4;

  #[inline]
  fn bitand(self, other: short4) -> short4 {
    return unsafe { simd_and(short4::broadcast(self), other) };
  }
}

impl std::ops::BitOr for short4 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: Self) -> Self {
    return unsafe { simd_or(self, other) };
  }
}

impl std::ops::BitOr<i16> for short4 {
  type Output = Self;

  #[inline]
  fn bitor(self, other: i16) -> Self {
    return unsafe { simd_or(self, short4::broadcast(other)) };
  }
}

impl std::ops::BitOr<short4> for i16 {
  type Output = short4;

  #[inline]
  fn bitor(self, other: short4) -> short4 {
    return unsafe { simd_or(short4::broadcast(self), other) };
  }
}

impl std::ops::BitXor for short4 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: Self) -> Self {
    return unsafe { simd_xor(self, other) };
  }
}

impl std::ops::BitXor<i16> for short4 {
  type Output = Self;

  #[inline]
  fn bitxor(self, other: i16) -> Self {
    return unsafe { simd_xor(self, short4::broadcast(other)) };
  }
}

impl std::ops::BitXor<short4> for i16 {
  type Output = short4;

  #[inline]
  fn bitxor(self, other: short4) -> short4 {
    return unsafe { simd_xor(short4::broadcast(self), other) };
  }
}

impl std::ops::Shl<short4> for short4 {
  type Output = Self;

  #[inline]
  fn shl(self, other: Self) -> Self {
    return unsafe { simd_shl(self, other) };
  }
}

impl std::ops::Shl<i16> for short4 {
  type Output = Self;

  #[inline]
  fn shl(self, other: i16) -> Self {
    return unsafe { simd_shl(self, short4::broadcast(other)) };
  }
}

impl std::ops::Shl<short4> for i16 {
  type Output = short4;

  #[inline]
  fn shl(self, other: short4) -> short4 {
    return unsafe { simd_shl(short4::broadcast(self), other) };
  }
}

impl std::ops::Shr<short4> for short4 {
  type Output = Self;

  #[inline]
  fn shr(self, other: Self) -> Self {
    return unsafe { simd_shr(self, other) };
  }
}

impl std::ops::Shr<i16> for short4 {
  type Output = Self;

  #[inline]
  fn shr(self, other: i16) -> Self {
    return unsafe { simd_shr(self, short4::broadcast(other)) };
  }
}

impl std::ops::Shr<short4> for i16 {
  type Output = short4;

  #[inline]
  fn shr(self, other: short4) -> short4 {
    return unsafe { simd_shr(short4::broadcast(self), other) };
  }
}

impl std::ops::Not for short4 {
  type Output = Self;

  #[inline]
  fn not(self) -> Self {
    return self ^ -1;
  }
}

impl PartialEq for short4 {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    return simd::all(simd::eq(*self, *other));
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return simd::all(simd::ne(*self, *other));
  }
}

impl simd::Vector for short4 {
  type Scalar = i16;
  type Boolean = short4;

  #[inline(always)]
  fn abs(self) -> Self {
    let mask = self >> 15;
    return (self ^ mask) - mask;
  }

  #[inline(always)]
  fn max(self, other: Self) -> Self {
    return simd::bitselect(simd::gt(other, self), self, other);
  }

  #[inline(always)]
  fn min(self, other: Self) -> Self {
    return simd::bitselect(simd::lt(other, self), self, other);
  }
}

impl simd::Dot for short4 {
  type DotProduct = i16;

  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return simd::reduce_add(self * other);
  }
}

impl simd::Logic for short4 {
  #[inline(always)]
  fn all(self) -> bool {
    return (self.0 & self.1 & self.2 & self.3) & std::i16::MIN != 0;
  }

  #[inline(always)]
  fn any(self) -> bool {
    return (self.0 | self.1 | self.2 | self.3) & std::i16::MIN != 0;
  }
}

impl simd::Reduce for short4 {
  #[inline(always)]
  fn reduce_add(self) -> Self::Scalar {
    return simd::reduce_add(self.lo() + self.hi());
  }

  #[inline(always)]
  fn reduce_min(self) -> Self::Scalar {
    return simd::reduce_min(simd::min(self.lo(), self.hi()));
  }

  #[inline(always)]
  fn reduce_max(self) -> Self::Scalar {
    return simd::reduce_max(simd::max(self.lo(), self.hi()));
  }
}

impl simd::Select<short4> for short4 {
  #[inline(always)]
  fn select(self, a: short4, b: short4) -> short4 {
    return (self >> 15).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: short4, b: short4) -> short4 {
    return (a & !self) | (b & self);
  }
}

impl simd::Select<ushort4> for short4 {
  #[inline(always)]
  fn select(self, a: ushort4, b: ushort4) -> ushort4 {
    return (self >> 15).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: ushort4, b: ushort4) -> ushort4 {
    return ushort4::bitcast(self.bitselect(short4::bitcast(a), short4::bitcast(b)));
  }
}

impl short4 {
  #[inline]
  pub fn bitcast<T>(x: T) -> short4 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: i16) -> Self {
    return short4(x, x, x, x);
  }

  #[inline]
  pub fn madd(x: short4, y: short4, z: short4) -> short4 {
    return x * y + z;
  }

  #[inline]
  pub fn to_char(x: short4) -> char4 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_char_sat(x: short4) -> char4 {
    return short4::to_char(simd::clamp(x, short4::broadcast(std::i8::MIN as i16), short4::broadcast(std::i8::MAX as i16)));
  }

  #[inline]
  pub fn to_uchar(x: short4) -> uchar4 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_uchar_sat(x: short4) -> uchar4 {
    return short4::to_uchar(simd::clamp(x, short4::broadcast(std::u8::MIN as i16), short4::broadcast(std::u8::MAX as i16)));
  }

  #[inline]
  pub fn to_short(x: short4) -> short4 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_short_sat(x: short4) -> short4 {
    return x;
  }

  #[inline]
  pub fn to_ushort(x: short4) -> ushort4 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_ushort_sat(x: short4) -> ushort4 {
    return short4::to_ushort(simd::max(x, short4::broadcast(0)));
  }

  #[inline]
  pub fn to_int(x: short4) -> int4 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_int_sat(x: short4) -> int4 {
    return short4::to_int(x);
  }

  #[inline]
  pub fn to_uint(x: short4) -> uint4 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_uint_sat(x: short4) -> uint4 {
    return short4::to_uint(simd::max(x, short4::broadcast(0)));
  }

  #[inline]
  pub fn to_float(x: short4) -> float4 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_long(x: short4) -> long4 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_long_sat(x: short4) -> long4 {
    return short4::to_long(x);
  }

  #[inline]
  pub fn to_ulong(x: short4) -> ulong4 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_ulong_sat(x: short4) -> ulong4 {
    return short4::to_ulong(simd::max(x, short4::broadcast(0)));
  }

  #[inline]
  pub fn to_double(x: short4) -> double4 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn lo(self) -> short2 {
    return short2(self.0, self.1);
  }

  #[inline]
  pub fn hi(self) -> short2 {
    return short2(self.2, self.3);
  }

  #[inline]
  pub fn odd(self) -> short2 {
    return short2(self.1, self.3);
  }

  #[inline]
  pub fn even(self) -> short2 {
    return short2(self.0, self.2);
  }
}
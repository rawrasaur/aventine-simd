use std;
use ::*;

impl Vector for short2 {
  type Scalar = i16;
  type Boolean = short2;

  type CharVector = char2;
  type ShortVector = short2;
  type IntVector = int2;
  type LongVector = long2;

  type UCharVector = uchar2;
  type UShortVector = ushort2;
  type UIntVector = uint2;
  type ULongVector = ulong2;

  type FloatVector = float2;
  type DoubleVector = double2;

  #[inline(always)]
  fn abs(self) -> Self {
    let mask = self >> 15;

    return (self ^ mask) - mask;
  }

  #[inline(always)]
  fn max(self, other: Self) -> Self {
    return gt(other, self).bitselect(self, other);
  }

  #[inline(always)]
  fn min(self, other: Self) -> Self {
    return lt(other, self).bitselect(self, other);
  }

  #[inline(always)]
  fn reduce_add(self) -> Self::Scalar {
    return self.0 + self.1;
  }

  #[inline(always)]
  fn reduce_min(self) -> Self::Scalar {
    return std::cmp::min(self.0, self.1);
  }

  #[inline(always)]
  fn reduce_max(self) -> Self::Scalar {
    return std::cmp::max(self.0, self.1);
  }

  #[inline(always)]
  fn to_char_sat(self) -> char2 {
    return short2::to_char(self.clamp(Self::broadcast(std::i8::MIN as i16), Self::broadcast(std::i8::MAX as i16)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar2 {
    return short2::to_uchar(self.clamp(Self::broadcast(std::u8::MIN as i16), Self::broadcast(std::u8::MAX as i16)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short2 {
    return self;
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort2 {
    return short2::to_ushort(self.max(Self::from(0)));
  }

  #[inline(always)]
  fn to_int_sat(self) -> int2 {
    return short2::to_int(self);
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint2 {
    return short2::to_uint(self.max(Self::from(0)));
  }

  #[inline(always)]
  fn to_long_sat(self) -> long2 {
    return short2::to_long(self);
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong2 {
    return short2::to_ulong(self.max(Self::from(0)));
  }
}

impl Dot<short2> for short2 {
  type DotProduct = i16;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return reduce_add(self * other);
  }
}

impl Integer for short2 {
  #[inline(always)]
  fn reduce_and(self) -> Self::Scalar {
    return self.0 & self.1
  }

  #[inline(always)]
  fn reduce_or(self) -> Self::Scalar {
    return self.0 | self.1
  }

  #[inline(always)]
  fn reduce_xor(self) -> Self::Scalar {
    return self.0 ^ self.1
  }

  #[inline(always)]
  fn all(self) -> bool {
    return self.reduce_and() & std::i16::MIN != 0;
  }

  #[inline(always)]
  fn any(self) -> bool {
    return self.reduce_or() & std::i16::MIN != 0;
  }
}

impl Select<short2> for short2 {
  #[inline(always)]
  fn select(self, a: short2, b: short2) -> short2 {
    return (self >> 15).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: short2, b: short2) -> short2 {
    return (a & !self) | (b & self);
  }
}

impl Select<ushort2> for short2 {
  #[inline(always)]
  fn select(self, a: ushort2, b: ushort2) -> ushort2 {
    return (self >> 15).bitselect(a, b);
  }

  #[inline(always)]
  fn bitselect(self, a: ushort2, b: ushort2) -> ushort2 {
    return ushort2::bitcast(self.bitselect(short2::bitcast(a), short2::bitcast(b)));
  }
}

impl short2 {
  #[inline]
  pub fn bitcast<T>(x: T) -> short2 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn lo(self) -> i16 {
    return self.0;
  }

  #[inline]
  pub fn hi(self) -> i16 {
    return self.1;
  }

  #[inline]
  pub fn odd(self) -> i16 {
    return self.1;
  }

  #[inline]
  pub fn even(self) -> i16 {
    return self.0;
  }
}
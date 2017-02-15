use std;
use ::*;

impl Vector for ushort16 {
  type Scalar = u16;
  type Boolean = short16;

  type CharVector = char16;
  type ShortVector = short16;
  type IntVector = int16;
  type LongVector = long16;

  type UCharVector = uchar16;
  type UShortVector = ushort16;
  type UIntVector = uint16;
  type ULongVector = ulong16;

  type FloatVector = float16;
  type DoubleVector = double16;

  #[inline(always)]
  fn abs(self) -> Self {
    return self;
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
    return reduce_add(self.lo() + self.hi());
  }

  #[inline(always)]
  fn reduce_min(self) -> Self::Scalar {
    return reduce_min(min(self.lo(), self.hi()));
  }

  #[inline(always)]
  fn reduce_max(self) -> Self::Scalar {
    return reduce_max(max(self.lo(), self.hi()));
  }

  #[inline(always)]
  fn to_char_sat(self) -> char16 {
    return ushort16::to_char(self.min(Self::broadcast(std::i8::MAX as u16)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar16 {
    return ushort16::to_uchar(self.min(Self::broadcast(std::u8::MAX as u16)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short16 {
    return ushort16::to_short(self.min(Self::broadcast(std::i16::MAX as u16)));
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort16 {
    return self;
  }

  #[inline(always)]
  fn to_int_sat(self) -> int16 {
    return ushort16::to_int(self.min(Self::broadcast(std::i32::MAX as u16)));
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint16 {
    return ushort16::to_uint(self);
  }

  #[inline(always)]
  fn to_long_sat(self) -> long16 {
    return ushort16::to_long(self.min(Self::broadcast(std::i64::MAX as u16)));
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong16 {
    return ushort16::to_ulong(self);
  }
}

impl Dot<ushort16> for ushort16 {
  type DotProduct = u16;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return reduce_add(self * other);
  }
}

impl Integer for ushort16 {
  #[inline(always)]
  fn reduce_and(self) -> Self::Scalar {
    return (self.lo() & self.hi()).reduce_and();
  }

  #[inline(always)]
  fn reduce_or(self) -> Self::Scalar {
    return (self.lo() | self.hi()).reduce_or();
  }

  #[inline(always)]
  fn reduce_xor(self) -> Self::Scalar {
    return (self.lo() ^ self.hi()).reduce_xor();
  }

  #[inline(always)]
  fn all(self) -> bool {
    return self.reduce_and() & 0x8000 != 0;
  }

  #[inline(always)]
  fn any(self) -> bool {
    return self.reduce_or() & 0x8000 != 0;
  }
}

impl ushort16 {
  #[inline]
  pub fn bitcast<T>(x: T) -> ushort16 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn lo(self) -> ushort8 {
    return ushort8(self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7);
  }

  #[inline]
  pub fn hi(self) -> ushort8 {
    return ushort8(self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15);
  }

  #[inline]
  pub fn odd(self) -> ushort8 {
    return ushort8(self.1, self.3, self.5, self.7, self.9, self.11, self.13, self.15);
  }

  #[inline]
  pub fn even(self) -> ushort8 {
    return ushort8(self.0, self.2, self.4, self.6, self.8, self.10, self.12, self.14);
  }
}

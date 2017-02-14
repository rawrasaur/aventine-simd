use std;
use ::*;

impl Vector for uchar16 {
  type Scalar = u8;
  type Boolean = char16;

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
    return bitselect(gt(other, self), self, other);
  }

  #[inline(always)]
  fn min(self, other: Self) -> Self {
    return bitselect(lt(other, self), self, other);
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
    return uchar16::to_char(min(self, broadcast(std::i8::MAX as u8)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar16 {
    return self;
  }

  #[inline(always)]
  fn to_short_sat(self) -> short16 {
    return uchar16::to_short(min(self, broadcast(std::i16::MAX as u8)));
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort16 {
    return uchar16::to_ushort(self);
  }

  #[inline(always)]
  fn to_int_sat(self) -> int16 {
    return uchar16::to_int(min(self, broadcast(std::i32::MAX as u8)));
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint16 {
    return uchar16::to_uint(self);
  }

  #[inline(always)]
  fn to_long_sat(self) -> long16 {
    return uchar16::to_long(min(self, broadcast(std::i64::MAX as u8)));
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong16 {
    return uchar16::to_ulong(self);
  }
}

impl Dot<uchar16> for uchar16 {
  type DotProduct = u8;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return reduce_add(self * other);
  }
}

impl Integer for uchar16 {
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
    return self.reduce_and() & 0x80 != 0;
  }

  #[inline(always)]
  fn any(self) -> bool {
    return self.reduce_or() & 0x80 != 0;
  }
}

impl uchar16 {
  #[inline]
  pub fn bitcast<T>(x: T) -> uchar16 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn lo(self) -> uchar8 {
    return uchar8(self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7);
  }

  #[inline]
  pub fn hi(self) -> uchar8 {
    return uchar8(self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15);
  }

  #[inline]
  pub fn odd(self) -> uchar8 {
    return uchar8(self.1, self.3, self.5, self.7, self.9, self.11, self.13, self.15);
  }

  #[inline]
  pub fn even(self) -> uchar8 {
    return uchar8(self.0, self.2, self.4, self.6, self.8, self.10, self.12, self.14);
  }
}

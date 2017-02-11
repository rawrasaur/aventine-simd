use std;
use ::*;

impl Vector for ushort4 {
  type Scalar = u16;
  type Boolean = short4;

  type CharVector = char4;
  type ShortVector = short4;
  type IntVector = int4;
  type LongVector = long4;

  type UCharVector = uchar4;
  type UShortVector = ushort4;
  type UIntVector = uint4;
  type ULongVector = ulong4;

  type FloatVector = float4;
  type DoubleVector = double4;

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
  fn to_char_sat(self) -> char4 {
    return ushort4::to_char(min(self, ushort4::broadcast(std::i8::MAX as u16)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar4 {
    return ushort4::to_uchar(min(self, ushort4::broadcast(std::u8::MAX as u16)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short4 {
    return ushort4::to_short(min(self, ushort4::broadcast(std::i16::MAX as u16)));
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort4 {
    return self;
  }

  #[inline(always)]
  fn to_int_sat(self) -> int4 {
    return ushort4::to_int(min(self, ushort4::broadcast(std::i32::MAX as u16)));
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint4 {
    return ushort4::to_uint(self);
  }

  #[inline(always)]
  fn to_long_sat(self) -> long4 {
    return ushort4::to_long(min(self, ushort4::broadcast(std::i64::MAX as u16)));
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong4 {
    return ushort4::to_ulong(self);
  }
}

impl Dot for ushort4 {
  type DotProduct = u16;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return reduce_add(self * other);
  }
}

impl Integer for ushort4 {
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

impl ushort4 {
  #[inline]
  pub fn bitcast<T>(x: T) -> ushort4 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: u16) -> Self {
    return ushort4(x, x, x, x);
  }

  #[inline]
  pub fn lo(self) -> ushort2 {
    return ushort2(self.0, self.1);
  }

  #[inline]
  pub fn hi(self) -> ushort2 {
    return ushort2(self.2, self.3);
  }

  #[inline]
  pub fn odd(self) -> ushort2 {
    return ushort2(self.1, self.3);
  }

  #[inline]
  pub fn even(self) -> ushort2 {
    return ushort2(self.0, self.2);
  }
}

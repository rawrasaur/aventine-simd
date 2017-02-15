use std;
use ::*;

impl Vector for uint4 {
  type Scalar = u32;
  type Boolean = int4;

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
  fn map_unary(self, f: &Fn(Self::Scalar) -> Self::Scalar) -> Self {
    return uint4(f(self.0), f(self.1), f(self.2), f(self.3));
  }

  #[inline(always)]
  fn map_binary(self, other: Self, f: &Fn(Self::Scalar, Self::Scalar) -> Self::Scalar) -> Self {
    return uint4(f(self.0, other.0), f(self.1, other.1), f(self.2, other.2), f(self.3, other.3));
  }

  #[inline(always)]
  fn abs(self) -> Self {
    return self;
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
    return uint4::to_char(self.min(Self::broadcast(std::i8::MAX as u32)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar4 {
    return uint4::to_uchar(self.min(Self::broadcast(std::u8::MAX as u32)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short4 {
    return uint4::to_short(self.min(Self::broadcast(std::i16::MAX as u32)));
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort4 {
    return uint4::to_ushort(self.min(Self::broadcast(std::u16::MAX as u32)));
  }

  #[inline(always)]
  fn to_int_sat(self) -> int4 {
    return uint4::to_int(self.min(Self::broadcast(std::i32::MAX as u32)));
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint4 {
    return self;
  }

  #[inline(always)]
  fn to_long_sat(self) -> long4 {
    return uint4::to_long(self.min(Self::broadcast(std::i64::MAX as u32)));
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong4 {
    return uint4::to_ulong(self);
  }
}

impl Dot<uint4> for uint4 {
  type DotProduct = u32;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return reduce_add(self * other);
  }
}

impl Integer for uint4 {
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
    return self.reduce_and() & 0x80000000 != 0;
  }

  #[inline(always)]
  fn any(self) -> bool {
    return self.reduce_or() & 0x80000000 != 0;
  }
}

impl uint4 {
  #[inline]
  pub fn bitcast<T>(x: T) -> uint4 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn lo(self) -> uint2 {
    return uint2(self.0, self.1);
  }

  #[inline]
  pub fn hi(self) -> uint2 {
    return uint2(self.2, self.3);
  }

  #[inline]
  pub fn odd(self) -> uint2 {
    return uint2(self.1, self.3);
  }

  #[inline]
  pub fn even(self) -> uint2 {
    return uint2(self.0, self.2);
  }
}

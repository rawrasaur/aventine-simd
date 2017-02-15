use std;
use ::*;

impl Vector for ushort2 {
  type Scalar = u16;
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
  fn map_unary(self, f: &Fn(Self::Scalar) -> Self::Scalar) -> Self {
    return ushort2(f(self.0), f(self.1));
  }

  #[inline(always)]
  fn map_binary(self, other: Self, f: &Fn(Self::Scalar, Self::Scalar) -> Self::Scalar) -> Self {
    return ushort2(f(self.0, other.0), f(self.1, other.1));
  }

  #[inline(always)]
  fn reduce(self, f: &Fn(Self::Scalar, Self::Scalar) -> Self::Scalar) -> Self::Scalar {
    return f(self.1, self.0);
  }

  #[inline(always)]
  fn abs(self) -> Self {
    return self;
  }

  #[inline(always)]
  fn to_char_sat(self) -> char2 {
    return ushort2::to_char(self.min(Self::broadcast(std::i8::MAX as u16)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar2 {
    return ushort2::to_uchar(self.min(Self::broadcast(std::u8::MAX as u16)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short2 {
    return ushort2::to_short(self.min(Self::broadcast(std::i16::MAX as u16)));
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort2 {
    return self;
  }

  #[inline(always)]
  fn to_int_sat(self) -> int2 {
    return ushort2::to_int(self.min(Self::broadcast(std::i32::MAX as u16)));
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint2 {
    return ushort2::to_uint(self);
  }

  #[inline(always)]
  fn to_long_sat(self) -> long2 {
    return ushort2::to_long(self.min(Self::broadcast(std::i64::MAX as u16)));
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong2 {
    return ushort2::to_ulong(self);
  }
}

impl Dot<ushort2> for ushort2 {
  type DotProduct = u16;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return reduce_add(self * other);
  }
}

impl Integer for ushort2 {
  type IntegerScalar = u16;

  const SIGN_MASK: u16 = 0x8000;
}

impl ushort2 {
  #[inline(always)]
  pub fn lo(self) -> u16 {
    return self.0;
  }

  #[inline(always)]
  pub fn hi(self) -> u16 {
    return self.1;
  }

  #[inline(always)]
  pub fn odd(self) -> u16 {
    return self.1;
  }

  #[inline(always)]
  pub fn even(self) -> u16 {
    return self.0;
  }
}

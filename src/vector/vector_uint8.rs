use std;
use ::*;

impl Vector for uint8 {
  type Scalar = u32;
  type Boolean = int8;

  type CharVector = char8;
  type ShortVector = short8;
  type IntVector = int8;
  type LongVector = long8;

  type UCharVector = uchar8;
  type UShortVector = ushort8;
  type UIntVector = uint8;
  type ULongVector = ulong8;

  type FloatVector = float8;
  type DoubleVector = double8;

  #[inline(always)]
  fn map_unary(self, f: &Fn(Self::Scalar) -> Self::Scalar) -> Self {
    return uint8(f(self.0), f(self.1), f(self.2), f(self.3), f(self.4), f(self.5), f(self.6), f(self.7));
  }

  #[inline(always)]
  fn map_binary(self, other: Self, f: &Fn(Self::Scalar, Self::Scalar) -> Self::Scalar) -> Self {
    return uint8(f(self.0, other.0), f(self.1, other.1), f(self.2, other.2), f(self.3, other.3), f(self.4, other.4), f(self.5, other.5), f(self.6, other.6), f(self.7, other.7));
  }

  #[inline(always)]
  fn reduce(self, f: &Fn(Self::Scalar, Self::Scalar) -> Self::Scalar) -> Self::Scalar {
    return f(self.7, f(self.6, f(self.5, f(self.4, f(self.3, f(self.2, f(self.1, self.0)))))));
  }

  #[inline(always)]
  fn abs(self) -> Self {
    return self;
  }

  #[inline(always)]
  fn to_char_sat(self) -> char8 {
    return uint8::to_char(self.min(Self::broadcast(std::i8::MAX as u32)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar8 {
    return uint8::to_uchar(self.min(Self::broadcast(std::u8::MAX as u32)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short8 {
    return uint8::to_short(self.min(Self::broadcast(std::i16::MAX as u32)));
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort8 {
    return uint8::to_ushort(self.min(Self::broadcast(std::u16::MAX as u32)));
  }

  #[inline(always)]
  fn to_int_sat(self) -> int8 {
    return uint8::to_int(self.min(Self::broadcast(std::i32::MAX as u32)));
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint8 {
    return self;
  }

  #[inline(always)]
  fn to_long_sat(self) -> long8 {
    return uint8::to_long(self.min(Self::broadcast(std::i64::MAX as u32)));
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong8 {
    return uint8::to_ulong(self);
  }
}

impl Dot<uint8> for uint8 {
  type DotProduct = u32;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return reduce_add(self * other);
  }
}

impl Integer for uint8 {
  type IntegerScalar = u32;

  const SIGN_MASK: u32 = 0x80000000;
}

impl uint8 {
  #[inline(always)]
  pub fn lo(self) -> uint4 {
    return uint4(self.0, self.1, self.2, self.3);
  }

  #[inline(always)]
  pub fn hi(self) -> uint4 {
    return uint4(self.4, self.5, self.6, self.7);
  }

  #[inline(always)]
  pub fn odd(self) -> uint4 {
    return uint4(self.1, self.3, self.5, self.7);
  }

  #[inline(always)]
  pub fn even(self) -> uint4 {
    return uint4(self.0, self.2, self.4, self.6);
  }
}

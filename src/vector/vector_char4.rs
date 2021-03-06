use std;
use ::*;

impl Vector for char4 {
  type Scalar = i8;
  type Boolean = char4;

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
    return char4(f(self.0), f(self.1), f(self.2), f(self.3));
  }

  #[inline(always)]
  fn map_binary(self, other: Self, f: &Fn(Self::Scalar, Self::Scalar) -> Self::Scalar) -> Self {
    return char4(f(self.0, other.0), f(self.1, other.1), f(self.2, other.2), f(self.3, other.3));
  }

  #[inline(always)]
  fn reduce(self, f: &Fn(Self::Scalar, Self::Scalar) -> Self::Scalar) -> Self::Scalar {
    return f(self.3, f(self.2, f(self.1, self.0)));
  }

  #[inline(always)]
  fn abs(self) -> Self {
    let mask = self >> 7;

    return (self ^ mask) - mask;
  }

  #[inline(always)]
  fn to_char_sat(self) -> char4 {
    return self;
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar4 {
    return char4::to_uchar(self.max(Self::from(0)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short4 {
    return char4::to_short(self);
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort4 {
    return char4::to_ushort(self.max(Self::from(0)));
  }

  #[inline(always)]
  fn to_int_sat(self) -> int4 {
    return char4::to_int(self);
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint4 {
    return char4::to_uint(self.max(Self::from(0)));
  }

  #[inline(always)]
  fn to_long_sat(self) -> long4 {
    return char4::to_long(self);
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong4 {
    return char4::to_ulong(self.max(Self::from(0)));
  }
}

impl Dot<char4> for char4 {
  type DotProduct = i8;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return reduce_add(self * other);
  }
}

impl Integer for char4 {
  type IntegerScalar = i8;

  const SIGN_MASK: i8 = std::i8::MIN;
}

impl Select<char4> for char4 {
  const MASK_SHIFT: i8 = 7;

  #[inline(always)]
  fn bitselect(self, a: char4, b: char4) -> char4 {
    return (a & !self) | (b & self);
  }
}

impl Select<uchar4> for char4 {
  const MASK_SHIFT: i8 = 7;

  #[inline(always)]
  fn bitselect(self, a: uchar4, b: uchar4) -> uchar4 {
    return uchar4::bitcast(self.bitselect(char4::bitcast(a), char4::bitcast(b)));
  }
}

impl char4 {
  #[inline(always)]
  pub fn lo(self) -> char2 {
    return char2(self.0, self.1);
  }

  #[inline(always)]
  pub fn hi(self) -> char2 {
    return char2(self.2, self.3);
  }

  #[inline(always)]
  pub fn odd(self) -> char2 {
    return char2(self.1, self.3);
  }

  #[inline(always)]
  pub fn even(self) -> char2 {
    return char2(self.0, self.2);
  }
}

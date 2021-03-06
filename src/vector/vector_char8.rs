use std;
use ::*;

impl Vector for char8 {
  type Scalar = i8;
  type Boolean = char8;

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
    return char8(f(self.0), f(self.1), f(self.2), f(self.3), f(self.4), f(self.5), f(self.6), f(self.7));
  }

  #[inline(always)]
  fn map_binary(self, other: Self, f: &Fn(Self::Scalar, Self::Scalar) -> Self::Scalar) -> Self {
    return char8(f(self.0, other.0), f(self.1, other.1), f(self.2, other.2), f(self.3, other.3), f(self.4, other.4), f(self.5, other.5), f(self.6, other.6), f(self.7, other.7));
  }

  #[inline(always)]
  fn reduce(self, f: &Fn(Self::Scalar, Self::Scalar) -> Self::Scalar) -> Self::Scalar {
    return f(self.7, f(self.6, f(self.5, f(self.4, f(self.3, f(self.2, f(self.1, self.0)))))));
  }

  #[inline(always)]
  fn abs(self) -> Self {
    let mask = self >> 7;

    return (self ^ mask) - mask;
  }

  #[inline(always)]
  fn to_char_sat(self) -> char8 {
    return self;
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar8 {
    return char8::to_uchar(self.max(Self::from(0)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short8 {
    return char8::to_short(self);
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort8 {
    return char8::to_ushort(self.max(Self::from(0)));
  }

  #[inline(always)]
  fn to_int_sat(self) -> int8 {
    return char8::to_int(self);
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint8 {
    return char8::to_uint(self.max(Self::from(0)));
  }

  #[inline(always)]
  fn to_long_sat(self) -> long8 {
    return char8::to_long(self);
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong8 {
    return char8::to_ulong(self.max(Self::from(0)));
  }
}

impl Dot<char8> for char8 {
  type DotProduct = i8;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return reduce_add(self * other);
  }
}

impl Integer for char8 {
  type IntegerScalar = i8;

  const SIGN_MASK: i8 = std::i8::MIN;
}

impl Select<char8> for char8 {
  const MASK_SHIFT: i8 = 7;

  #[inline(always)]
  fn bitselect(self, a: char8, b: char8) -> char8 {
    return (a & !self) | (b & self);
  }
}

impl Select<uchar8> for char8 {
  const MASK_SHIFT: i8 = 7;

  #[inline(always)]
  fn bitselect(self, a: uchar8, b: uchar8) -> uchar8 {
    return uchar8::bitcast(self.bitselect(char8::bitcast(a), char8::bitcast(b)));
  }
}

impl char8 {
  #[inline(always)]
  pub fn lo(self) -> char4 {
    return char4(self.0, self.1, self.2, self.3);
  }

  #[inline(always)]
  pub fn hi(self) -> char4 {
    return char4(self.4, self.5, self.6, self.7);
  }

  #[inline(always)]
  pub fn odd(self) -> char4 {
    return char4(self.1, self.3, self.5, self.7);
  }

  #[inline(always)]
  pub fn even(self) -> char4 {
    return char4(self.0, self.2, self.4, self.6);
  }
}

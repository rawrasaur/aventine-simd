use std;
use ::*;

impl Vector for int2 {
  type Scalar = i32;
  type Boolean = int2;

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
    return int2(f(self.0), f(self.1));
  }

  #[inline(always)]
  fn map_binary(self, other: Self, f: &Fn(Self::Scalar, Self::Scalar) -> Self::Scalar) -> Self {
    return int2(f(self.0, other.0), f(self.1, other.1));
  }

  #[inline(always)]
  fn reduce(self, f: &Fn(Self::Scalar, Self::Scalar) -> Self::Scalar) -> Self::Scalar {
    return f(self.1, self.0);
  }

  #[inline(always)]
  fn abs(self) -> Self {
    let mask = self >> 31;

    return (self ^ mask) - mask;
  }

  #[inline(always)]
  fn to_char_sat(self) -> char2 {
    return int2::to_char(self.clamp(Self::broadcast(std::i8::MIN as i32), Self::broadcast(std::i8::MAX as i32)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar2 {
    return int2::to_uchar(self.clamp(Self::broadcast(std::u8::MIN as i32), Self::broadcast(std::u8::MAX as i32)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short2 {
    return int2::to_short(self.clamp(Self::broadcast(std::i16::MIN as i32), Self::broadcast(std::i16::MAX as i32)));
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort2 {
    return int2::to_ushort(self.clamp(Self::broadcast(std::u16::MIN as i32), Self::broadcast(std::u16::MAX as i32)));
  }

  #[inline(always)]
  fn to_int_sat(self) -> int2 {
    return self;
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint2 {
    return int2::to_uint(self.max(Self::from(0)));
  }

  #[inline(always)]
  fn to_long_sat(self) -> long2 {
    return int2::to_long(self);
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong2 {
    return int2::to_ulong(self.max(Self::from(0)));
  }
}

impl Dot<int2> for int2 {
  type DotProduct = i32;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return reduce_add(self * other);
  }
}

impl Integer for int2 {
  type IntegerScalar = i32;

  const SIGN_MASK: i32 = std::i32::MIN;
}

impl Select<int2> for int2 {
  const MASK_SHIFT: i32 = 31;

  #[inline(always)]
  fn bitselect(self, a: int2, b: int2) -> int2 {
    return (a & !self) | (b & self);
  }
}

impl Select<uint2> for int2 {
  const MASK_SHIFT: i32 = 31;

  #[inline(always)]
  fn bitselect(self, a: uint2, b: uint2) -> uint2 {
    return uint2::bitcast(self.bitselect(int2::bitcast(a), int2::bitcast(b)));
  }
}

impl Select<float2> for int2 {
  const MASK_SHIFT: i32 = 31;

  #[inline(always)]
  fn bitselect(self, a: float2, b: float2) -> float2 {
    return float2::bitcast(self.bitselect(int2::bitcast(a), int2::bitcast(b)));
  }
}

impl int2 {
  #[inline(always)]
  pub fn lo(self) -> i32 {
    return self.0;
  }

  #[inline(always)]
  pub fn hi(self) -> i32 {
    return self.1;
  }

  #[inline(always)]
  pub fn odd(self) -> i32 {
    return self.1;
  }

  #[inline(always)]
  pub fn even(self) -> i32 {
    return self.0;
  }
}

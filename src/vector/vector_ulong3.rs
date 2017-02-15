use std;
use ::*;

impl Vector for ulong3 {
  type Scalar = u64;
  type Boolean = long3;

  type CharVector = char3;
  type ShortVector = short3;
  type IntVector = int3;
  type LongVector = long3;

  type UCharVector = uchar3;
  type UShortVector = ushort3;
  type UIntVector = uint3;
  type ULongVector = ulong3;

  type FloatVector = float3;
  type DoubleVector = double3;

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
    return self.0 + self.1 + self.2;
  }

  #[inline(always)]
  fn reduce_min(self) -> Self::Scalar {
    return std::cmp::min(reduce_min(self.lo()), self.2);
  }

  #[inline(always)]
  fn reduce_max(self) -> Self::Scalar {
    return std::cmp::max(reduce_max(self.lo()), self.2);
  }

  #[inline(always)]
  fn to_char_sat(self) -> char3 {
    return ulong3::to_char(self.min(Self::broadcast(std::i8::MAX as u64)));
  }

  #[inline(always)]
  fn to_uchar_sat(self) -> uchar3 {
    return ulong3::to_uchar(self.min(Self::broadcast(std::u8::MAX as u64)));
  }

  #[inline(always)]
  fn to_short_sat(self) -> short3 {
    return ulong3::to_short(self.min(Self::broadcast(std::i16::MAX as u64)));
  }

  #[inline(always)]
  fn to_ushort_sat(self) -> ushort3 {
    return ulong3::to_ushort(self.min(Self::broadcast(std::u16::MAX as u64)));
  }

  #[inline(always)]
  fn to_int_sat(self) -> int3 {
    return ulong3::to_int(self.min(Self::broadcast(std::i32::MAX as u64)));
  }

  #[inline(always)]
  fn to_uint_sat(self) -> uint3 {
    return ulong3::to_uint(self.min(Self::broadcast(std::u32::MAX as u64)));
  }

  #[inline(always)]
  fn to_long_sat(self) -> long3 {
    return ulong3::to_long(self.min(Self::broadcast(std::i64::MAX as u64)));
  }

  #[inline(always)]
  fn to_ulong_sat(self) -> ulong3 {
    return self;
  }
}

impl Dot<ulong3> for ulong3 {
  type DotProduct = u64;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return reduce_add(self * other);
  }
}

impl Integer for ulong3 {
  #[inline(always)]
  fn reduce_and(self) -> Self::Scalar {
    return self.0 & self.1 & self.2
  }

  #[inline(always)]
  fn reduce_or(self) -> Self::Scalar {
    return self.0 | self.1 | self.2
  }

  #[inline(always)]
  fn reduce_xor(self) -> Self::Scalar {
    return self.0 ^ self.1 ^ self.2
  }

  #[inline(always)]
  fn all(self) -> bool {
    return self.reduce_and() & 0x8000000000000000 != 0;
  }

  #[inline(always)]
  fn any(self) -> bool {
    return self.reduce_or() & 0x8000000000000000 != 0;
  }
}

impl ulong3 {
  #[inline]
  pub fn bitcast<T>(x: T) -> ulong3 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn lo(self) -> ulong2 {
    return ulong2(self.0, self.1);
  }

  #[inline]
  pub fn hi(self) -> ulong2 {
    return ulong2(self.2, 0);
  }

  #[inline]
  pub fn odd(self) -> ulong2 {
    return ulong2(self.1, 0);
  }

  #[inline]
  pub fn even(self) -> ulong2 {
    return ulong2(self.0, self.2);
  }
}
use std;
use ::*;
use ::simd::*;

extern "platform-intrinsic" {
  fn simd_add<T>(x: T, y: T) -> T;
  fn simd_sub<T>(x: T, y: T) -> T;
  fn simd_mul<T>(x: T, y: T) -> T;
  fn simd_div<T>(x: T, y: T) -> T;

  fn simd_cast<T, U>(x: T) -> U;
}

impl std::ops::Add for double3 {
  type Output = Self;

  #[inline]
  fn add(self, other: Self) -> Self {
    return unsafe { simd_add(self, other) };
  }
}

impl std::ops::Add<f64> for double3 {
  type Output = Self;

  #[inline]
  fn add(self, other: f64) -> Self {
    return unsafe { simd_add(self, double3::broadcast(other)) };
  }
}

impl std::ops::Add<double3> for f64 {
  type Output = double3;

  #[inline]
  fn add(self, other: double3) -> double3 {
    return unsafe { simd_add(double3::broadcast(self), other) };
  }
}

impl std::ops::Sub for double3 {
  type Output = Self;

  #[inline]
  fn sub(self, other: Self) -> Self {
    return unsafe { simd_sub(self, other) };
  }
}

impl std::ops::Sub<f64> for double3 {
  type Output = Self;

  #[inline]
  fn sub(self, other: f64) -> Self {
    return unsafe { simd_sub(self, double3::broadcast(other)) };
  }
}

impl std::ops::Sub<double3> for f64 {
  type Output = double3;

  #[inline]
  fn sub(self, other: double3) -> double3 {
    return unsafe { simd_sub(double3::broadcast(self), other) };
  }
}

impl std::ops::Mul for double3 {
  type Output = Self;

  #[inline]
  fn mul(self, other: Self) -> Self {
    return unsafe { simd_mul(self, other) };
  }
}

impl std::ops::Mul<f64> for double3 {
  type Output = Self;

  #[inline]
  fn mul(self, other: f64) -> Self {
    return unsafe { simd_mul(self, double3::broadcast(other)) };
  }
}

impl std::ops::Mul<double3> for f64 {
  type Output = double3;

  #[inline]
  fn mul(self, other: double3) -> double3 {
    return unsafe { simd_mul(double3::broadcast(self), other) };
  }
}

impl std::ops::Div for double3 {
  type Output = Self;

  #[inline]
  fn div(self, other: Self) -> Self {
    return unsafe { simd_div(self, other) };
  }
}

impl std::ops::Div<f64> for double3 {
  type Output = Self;

  #[inline]
  fn div(self, other: f64) -> Self {
    return unsafe { simd_div(self, double3::broadcast(other)) };
  }
}

impl std::ops::Div<double3> for f64 {
  type Output = double3;

  #[inline]
  fn div(self, other: double3) -> double3 {
    return unsafe { simd_div(double3::broadcast(self), other) };
  }
}

impl PartialEq for double3 {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    return simd::eq(*self, *other).all();
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return simd::ne(*self, *other).all();
  }
}

impl simd::Vector for double3 {
  type Scalar = f64;
  type Boolean = long3;

  #[inline(always)]
  fn abs(self) -> Self {
    return simd::bitselect(long3::broadcast(std::i64::MAX), double3::broadcast(0.0), self);
  }

  #[inline(always)]
  fn max(self, other: Self) -> Self {
    return double3(self.0.max(other.0), self.1.max(other.1), self.2.max(other.2));
  }

  #[inline(always)]
  fn min(self, other: Self) -> Self {
    return double3(self.0.min(other.0), self.1.min(other.1), self.2.min(other.2));
  }
}

impl simd::Cross for double3 {
  type CrossProduct = double3;

  #[inline(always)]
  fn cross(self, other: Self) -> Self::CrossProduct {
    let a = self * double3(other.2, other.1, other.0) - double3(self.2, self.1, self.0) * other;

    return double3(a.2, a.1, a.0);
  }
}

impl simd::Dot for double3 {
  type DotProduct = f64;
  #[inline(always)]
  fn dot(self, other: Self) -> Self::DotProduct {
    return simd::reduce_add(self * other);
  }
}

impl simd::Float for double3 {
  #[inline(always)]
  fn copysign(self, magnitude: Self) -> Self {
    return simd::bitselect(long3::broadcast(std::i64::MAX), magnitude, self);
  }

  #[inline(always)]
  fn sign(self) -> Self {
    let (zero, one) = (double3::broadcast(0.0), double3::broadcast(1.0));

    return simd::bitselect(simd::eq(self, zero) | simd::ne(self, self), one.copysign(self), zero);
  }

  #[inline(always)]
  fn sqrt(self) -> Self {
    return double3(self.0.sqrt(), self.1.sqrt(), self.2.sqrt());
  }

  #[inline(always)]
  fn recip(self) -> Self {
    return 1.0 / self;
  }

  #[inline(always)]
  fn rsqrt(self) -> Self {
    return self.sqrt().recip();
  }

  #[inline(always)]
  fn fract(self) -> Self {
    return double3(self.0.fract(), self.1.fract(), self.2.fract());
  }

  #[inline(always)]
  fn ceil(self) -> Self {
    return double3(self.0.ceil(), self.1.ceil(), self.2.ceil());
  }

  #[inline(always)]
  fn floor(self) -> Self {
    return double3(self.0.floor(), self.1.floor(), self.2.floor());
  }

  #[inline(always)]
  fn trunc(self) -> Self {
    return double3(self.0.trunc(), self.1.trunc(), self.2.trunc());
  }

  #[inline(always)]
  fn mix(self, a: Self, b: Self) -> Self {
    return a + self * (b - a);
  }

  #[inline(always)]
  fn step(self, edge: Self) -> Self {
    return simd::bitselect(simd::lt(self, edge), double3::broadcast(1.0), double3::broadcast(0.0));
  }

  #[inline(always)]
  fn smoothstep(self, edge0: Self, edge1: Self) -> Self {
    let t = simd::clamp((self - edge0) / (edge1 - edge0), double3::broadcast(0.0), double3::broadcast(1.0));

    return t * t * (3.0 - 2.0 * t);
  }

  #[inline(always)]
  fn sin(self) -> Self {
    return double3(self.0.sin(), self.1.sin(), self.2.sin());
  }

  #[inline(always)]
  fn cos(self) -> Self {
    return double3(self.0.cos(), self.1.cos(), self.2.cos());
  }
}

impl simd::Geometry for double3 {
  #[inline(always)]
  fn project(self, onto: Self) -> Self {
    return (self.dot(onto) / onto.dot(onto)) * onto;
  }

  #[inline(always)]
  fn length(self) -> Self::Scalar {
    return self.length_squared().sqrt();
  }

  #[inline(always)]
  fn length_squared(self) -> Self::Scalar {
    return self.dot(self);
  }

  #[inline(always)]
  fn norm_one(self) -> Self::Scalar {
    return self.abs().reduce_add();
  }

  #[inline(always)]
  fn norm_inf(self) -> Self::Scalar {
    return self.abs().reduce_max();
  }

  #[inline(always)]
  fn distance(self, other: Self) -> Self::Scalar {
    return (self - other).length();
  }

  #[inline(always)]
  fn distance_squared(self, other: Self) -> Self::Scalar {
    return (self - other).length_squared();
  }

  #[inline(always)]
  fn normalize(self) -> Self {
    return self * simd::rsqrt(double3::broadcast(self.length_squared()));
  }

  #[inline(always)]
  fn reflect(self, n: Self) -> Self {
    return self - 2.0 * self.dot(n) * n;
  }

  #[inline(always)]
  fn refract(self, n: Self, eta: Self::Scalar) -> Self {
    let dp = self.dot(n);
    let k = 1.0 - eta * eta * (1.0 - dp * dp);

    return if k >= 0.0 { eta * self - (eta * dp + k.sqrt()) } else { double3::broadcast(0.0) };
  }
}

impl simd::Reduce for double3 {
  #[inline(always)]
  fn reduce_add(self) -> Self::Scalar {
    return self.0 + self.1 + self.2;
  }

  #[inline(always)]
  fn reduce_min(self) -> Self::Scalar {
    return self.2.min(simd::reduce_min(self.lo()));
  }

  #[inline(always)]
  fn reduce_max(self) -> Self::Scalar {
    return self.2.max(simd::reduce_max(self.lo()));
  }
}

impl double3 {
  #[inline]
  pub fn bitcast<T>(x: T) -> double3 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: f64) -> Self {
    return double3(x, x, x);
  }

  #[inline]
  pub fn madd(x: double3, y: double3, z: double3) -> double3 {
    return x * y + z;
  }

  #[inline]
  pub fn to_char(x: double3) -> char3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_char_sat(x: double3) -> char3 {
    return double3::to_char(simd::clamp(x, double3::broadcast(std::i8::MIN as f64), double3::broadcast(std::i8::MAX as f64)));
  }

  #[inline]
  pub fn to_uchar(x: double3) -> uchar3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_uchar_sat(x: double3) -> uchar3 {
    return double3::to_uchar(simd::clamp(x, double3::broadcast(std::u8::MIN as f64), double3::broadcast(std::u8::MAX as f64)));
  }

  #[inline]
  pub fn to_short(x: double3) -> short3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_short_sat(x: double3) -> short3 {
    return double3::to_short(simd::clamp(x, double3::broadcast(std::i16::MIN as f64), double3::broadcast(std::i16::MAX as f64)));
  }

  #[inline]
  pub fn to_ushort(x: double3) -> ushort3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_ushort_sat(x: double3) -> ushort3 {
    return double3::to_ushort(simd::clamp(x, double3::broadcast(std::u16::MIN as f64), double3::broadcast(std::u16::MAX as f64)));
  }

  #[inline]
  pub fn to_int(x: double3) -> int3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_int_sat(x: double3) -> int3 {
    return double3::to_int(simd::clamp(x, double3::broadcast(std::i32::MIN as f64), double3::broadcast(std::i32::MAX as f64)));
  }

  #[inline]
  pub fn to_uint(x: double3) -> uint3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_uint_sat(x: double3) -> uint3 {
    return double3::to_uint(simd::clamp(x, double3::broadcast(std::u32::MIN as f64), double3::broadcast(std::u32::MAX as f64)));
  }

  #[inline]
  pub fn to_float(x: double3) -> float3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_long(x: double3) -> long3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_long_sat(x: double3) -> long3 {
    return double3::to_long(simd::clamp(x, double3::broadcast(std::i64::MIN as f64), double3::broadcast(std::i64::MAX as f64)));
  }

  #[inline]
  pub fn to_ulong(x: double3) -> ulong3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_ulong_sat(x: double3) -> ulong3 {
    return double3::to_ulong(simd::clamp(x, double3::broadcast(std::u64::MIN as f64), double3::broadcast(std::u64::MAX as f64)));
  }

  #[inline]
  pub fn to_double(x: double3) -> double3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn lo(self) -> double2 {
    return double2(self.0, self.1);
  }

  #[inline]
  pub fn hi(self) -> double2 {
    return double2(self.2, 0.0);
  }

  #[inline]
  pub fn odd(self) -> double2 {
    return double2(self.1, 0.0);
  }

  #[inline]
  pub fn even(self) -> double2 {
    return double2(self.0, self.2);
  }
}

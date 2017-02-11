use std;
use ::*;

#[repr(C)]
#[repr(simd)]
#[derive(Copy, Clone, Debug)]
pub struct double3(pub f64, pub f64, pub f64);

extern "platform-intrinsic" {
  fn simd_add<T>(x: T, y: T) -> T;
  fn simd_sub<T>(x: T, y: T) -> T;
  fn simd_mul<T>(x: T, y: T) -> T;
  fn simd_div<T>(x: T, y: T) -> T;

  fn simd_eq<T, U>(x: T, y: T) -> U;
  fn simd_ne<T, U>(x: T, y: T) -> U;
  fn simd_lt<T, U>(x: T, y: T) -> U;
  fn simd_le<T, U>(x: T, y: T) -> U;
  fn simd_gt<T, U>(x: T, y: T) -> U;
  fn simd_ge<T, U>(x: T, y: T) -> U;

  fn simd_cast<T, U>(x: T) -> U;

  fn simd_insert<T, E>(x: T, i: u32, e: E) -> T;
  fn simd_extract<T, E>(x: T, i: u32) -> E;
}

impl std::ops::Index<u32> for double3 {
  type Output = f64;

  #[inline]
  fn index(&self, index: u32) -> &f64 {
    return unsafe { simd_extract(self, index) };
  }
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
    return simd::all(double3::eq(*self, *other));
  }

  #[inline]
  fn ne(&self, other: &Self) -> bool {
    return simd::all(double3::ne(*self, *other));
  }
}

impl simd::Vector for double3 {
}

impl simd::Dot for double3 {
  type Output = f64;

  #[inline]
  fn dot(self, other: double3) -> f64 {
    return double3::reduce_add(self * other);
  }
}

impl double3 {
  #[inline]
  pub fn bitcast<T>(x: T) -> double3 {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<Self>());

    return unsafe { std::mem::transmute_copy(&x) };
  }

  #[inline]
  pub fn broadcast(x: f64) -> double3 {
    return double3(x, x, x);
  }

  #[inline]
  pub fn extract(self, i: u32) -> f64 {
    return unsafe { simd_extract(self, i) };
  }

  #[inline]
  pub fn replace(self, i: u32, x: f64) -> double3 {
    return unsafe { simd_insert(self, i, x) };
  }

  #[inline]
  pub fn eq(x: double3, y: double3) -> long3 {
    return unsafe { simd_eq(x, y) };
  }

  #[inline]
  pub fn ne(x: double3, y: double3) -> long3 {
    return unsafe { simd_ne(x, y) };
  }

  #[inline]
  pub fn lt(x: double3, y: double3) -> long3 {
    return unsafe { simd_lt(x, y) };
  }

  #[inline]
  pub fn le(x: double3, y: double3) -> long3 {
    return unsafe { simd_le(x, y) };
  }

  #[inline]
  pub fn gt(x: double3, y: double3) -> long3 {
    return unsafe { simd_gt(x, y) };
  }

  #[inline]
  pub fn ge(x: double3, y: double3) -> long3 {
    return unsafe { simd_ge(x, y) };
  }

  #[inline]
  pub fn madd(x: double3, y: double3, z: double3) -> double3 {
    return x * y + z;
  }

  #[inline]
  pub fn abs(x: double3) -> double3 {
    return double3::bitselect(double3::broadcast(0.0), x, long3::broadcast(std::i64::MAX));
  }

  #[inline]
  pub fn max(x: double3, y: double3) -> double3 {
    return double3(x.0.max(y.0), x.1.max(y.1), x.2.max(y.2));
  }

  #[inline]
  pub fn min(x: double3, y: double3) -> double3 {
    return double3(x.0.min(y.0), x.1.min(y.1), x.2.min(y.2));
  }

  #[inline]
  pub fn clamp(x: double3, min: double3, max: double3) -> double3 {
    return double3::min(double3::max(x, min), max);
  }

  #[inline]
  pub fn sign(x: double3) -> double3 {
    let (zero, one) = (double3::broadcast(0.0), double3::broadcast(1.0));
    return double3::bitselect(double3::copysign(one, x), zero, double3::eq(x, zero) | double3::ne(x, x));
  }

  #[inline]
  pub fn mix(x: double3, y: double3, t: double3) -> double3 {
    return x + t * (y - x);
  }

  #[inline]
  pub fn recip(x: double3) -> double3 {
    return 1.0 / x;
  }

  #[inline]
  pub fn rsqrt(x: double3) -> double3 {
    return 1.0 / double3::sqrt(x);
  }

  #[inline]
  pub fn fract(x: double3) -> double3 {
    return double3(x.0.fract(), x.1.fract(), x.2.fract());
  }

  #[inline]
  pub fn step(edge: double3, x: double3) -> double3 {
    return double3::bitselect(double3::broadcast(1.0), double3::broadcast(0.0), double3::lt(x, edge));
  }

  #[inline]
  pub fn smoothstep(edge0: double3, edge1: double3, x: double3) -> double3 {
    let t = double3::clamp((x - edge0) / (edge1 - edge0), double3::broadcast(0.0), double3::broadcast(1.0));

    return t * t * (3.0 - 2.0 * t);
  }

  #[inline]
  pub fn reduce_add(x: double3) -> f64 {
    return x.0 + x.1 + x.2;
  }

  #[inline]
  pub fn reduce_min(x: double3) -> f64 {
    return x.2.min(double2::reduce_min(x.lo()));
  }

  #[inline]
  pub fn reduce_max(x: double3) -> f64 {
    return x.2.max(double2::reduce_max(x.lo()));
  }

  #[inline]
  pub fn copysign(x: double3, y: double3) -> double3 {
    return double3::bitselect(y, x, long3::broadcast(std::i64::MAX));
  }

  #[inline]
  pub fn sqrt(x: double3) -> double3 {
    return double3(x.0.sqrt(), x.1.sqrt(), x.2.sqrt());
  }

  #[inline]
  pub fn ceil(x: double3) -> double3 {
    return double3(x.0.ceil(), x.1.ceil(), x.2.ceil());
  }

  #[inline]
  pub fn floor(x: double3) -> double3 {
    return double3(x.0.floor(), x.1.floor(), x.2.floor());
  }

  #[inline]
  pub fn trunc(x: double3) -> double3 {
    return double3(x.0.trunc(), x.1.trunc(), x.2.trunc());
  }

  #[inline]
  pub fn sin(x: double3) -> double3 {
    return double3(x.0.sin(), x.1.sin(), x.2.sin());
  }

  #[inline]
  pub fn cos(x: double3) -> double3 {
    return double3(x.0.cos(), x.1.cos(), x.2.cos());
  }

  #[inline]
  pub fn dot(x: double3, y: double3) -> f64 {
    return double3::reduce_add(x * y);
  }

  #[inline]
  pub fn project(x: double3, y: double3) -> double3 {
    return double3::dot(x, y) / double3::dot(y, y) * y;
  }

  #[inline]
  pub fn length(x: double3) -> f64 {
    return double3::length_squared(x).sqrt();
  }

  #[inline]
  pub fn length_squared(x: double3) -> f64 {
    return double3::dot(x, x);
  }

  #[inline]
  pub fn norm_one(x: double3) -> f64 {
    return double3::reduce_add(double3::abs(x));
  }

  #[inline]
  pub fn norm_inf(x: double3) -> f64 {
    return double3::reduce_max(double3::abs(x));
  }

  #[inline]
  pub fn distance(x: double3, y: double3) -> f64 {
    return double3::length(x - y);
  }

  #[inline]
  pub fn distance_squared(x: double3, y: double3) -> f64 {
    return double3::length_squared(x - y);
  }

  #[inline]
  pub fn normalize(x: double3) -> double3 {
    return x * double3::rsqrt(double3::broadcast(double3::length_squared(x)));
  }

  #[inline]
  pub fn cross(x: double3, y: double3) -> double3 {
    let a = x * double3(y.2, y.1, y.0) - double3(x.2, x.1, x.0) * y;
    return double3(a.2, a.1, a.0);
  }

  #[inline]
  pub fn reflect(x: double3, n: double3) -> double3 {
    return x - 2.0 * double3::dot(x, n) * n;
  }

  #[inline]
  pub fn refract(x: double3, n: double3, eta: f64) -> double3 {
    let dp = double3::dot(x, n);
    let k = 1.0 - eta * eta * (1.0 - dp * dp);
    return if k >= 0.0 { eta * x - (eta * dp + k.sqrt()) } else { double3::broadcast(0.0) };
  }

  #[inline]
  pub fn to_char(x: double3) -> char3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_char_sat(x: double3) -> char3 {
    return double3::to_char(double3::clamp(x, double3::broadcast(std::i8::MIN as f64), double3::broadcast(std::i8::MAX as f64)));
  }

  #[inline]
  pub fn to_uchar(x: double3) -> uchar3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_uchar_sat(x: double3) -> uchar3 {
    return double3::to_uchar(double3::clamp(x, double3::broadcast(std::u8::MIN as f64), double3::broadcast(std::u8::MAX as f64)));
  }

  #[inline]
  pub fn to_short(x: double3) -> short3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_short_sat(x: double3) -> short3 {
    return double3::to_short(double3::clamp(x, double3::broadcast(std::i16::MIN as f64), double3::broadcast(std::i16::MAX as f64)));
  }

  #[inline]
  pub fn to_ushort(x: double3) -> ushort3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_ushort_sat(x: double3) -> ushort3 {
    return double3::to_ushort(double3::clamp(x, double3::broadcast(std::u16::MIN as f64), double3::broadcast(std::u16::MAX as f64)));
  }

  #[inline]
  pub fn to_int(x: double3) -> int3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_int_sat(x: double3) -> int3 {
    return double3::to_int(double3::clamp(x, double3::broadcast(std::i32::MIN as f64), double3::broadcast(std::i32::MAX as f64)));
  }

  #[inline]
  pub fn to_uint(x: double3) -> uint3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_uint_sat(x: double3) -> uint3 {
    return double3::to_uint(double3::clamp(x, double3::broadcast(std::u32::MIN as f64), double3::broadcast(std::u32::MAX as f64)));
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
    return double3::to_long(double3::clamp(x, double3::broadcast(std::i64::MIN as f64), double3::broadcast(std::i64::MAX as f64)));
  }

  #[inline]
  pub fn to_ulong(x: double3) -> ulong3 {
    return unsafe { simd_cast(x) };
  }

  #[inline]
  pub fn to_ulong_sat(x: double3) -> ulong3 {
    return double3::to_ulong(double3::clamp(x, double3::broadcast(std::u64::MIN as f64), double3::broadcast(std::u64::MAX as f64)));
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

use std;
use ::*;

extern {
  fn __invert_d4(a: double4x4) -> double4x4;
}

impl std::ops::Add for double4x4 {
  type Output = Self;

  #[inline(always)]
  fn add(self, other: Self) -> Self {
    return double4x4(self.0 + other.0, self.1 + other.1, self.2 + other.2, self.3 + other.3);
  }
}

impl std::ops::Sub for double4x4 {
  type Output = Self;

  #[inline(always)]
  fn sub(self, other: Self) -> Self {
    return double4x4(self.0 - other.0, self.1 - other.1, self.2 - other.2, self.3 - other.3);
  }
}

impl std::ops::Mul<double4x4> for double4x4 {
  type Output = double4x4;

  #[inline(always)]
  fn mul(self, other: double4x4) -> Self::Output {
    return self.dot(other);
  }
}

impl std::ops::Mul<double4> for double4x4 {
  type Output = double4;

  #[inline(always)]
  fn mul(self, other: double4) -> Self::Output {
    return self.dot(other);
  }
}

impl std::ops::Mul<f64> for double4x4 {
  type Output = Self;

  #[inline(always)]
  fn mul(self, other: f64) -> Self {
    let a = double4::broadcast(other);

    return double4x4(a * self.0, a * self.1, a * self.2, a * self.3);
  }
}

impl Dot<double4x4> for double4x4 {
  type DotProduct = double4x4;

  #[inline(always)]
  fn dot(self, other: double4x4) -> Self::DotProduct {
    return double4x4(self.dot(other.0), self.dot(other.1), self.dot(other.2), self.dot(other.3));
  }
}

impl Dot<double4> for double4x4 {
  type DotProduct = double4;

  #[inline(always)]
  fn dot(self, other: double4) -> Self::DotProduct {
    return self.0 * other.0 + self.1 * other.1 + self.2 * other.2 + self.3 * other.3;
  }
}

impl PartialEq for double4x4 {
  #[inline]
  fn eq(&self, other: &double4x4) -> bool {
    return (self.0.eq(other.0) & self.1.eq(other.1) & self.2.eq(other.2) & self.3.eq(other.3)).all()
  }
}

impl double4x4 {
  #[inline(always)]
  pub fn from_columns(c0: double4, c1: double4, c2: double4, c3: double4) -> double4x4 {
    return double4x4(c0, c1, c2, c3);
  }

  #[inline(always)]
  pub fn from_rows(r0: double4, r1: double4, r2: double4, r3: double4) -> double4x4 {
    return double4x4(r0, r1, r2, r3).transpose();
  }

  #[inline(always)]
  pub fn identity() -> double4x4 {
    return double4x4(double4(1.0, 0.0, 0.0, 0.0), double4(0.0, 1.0, 0.0, 0.0), double4(0.0, 0.0, 1.0, 0.0), double4(0.0, 0.0, 0.0, 1.0));
  }

  #[inline(always)]
  pub fn from_scale(scale: f64) -> double4x4 {
    return double4x4(double4(scale, 0.0, 0.0, 0.0), double4(0.0, scale, 0.0, 0.0), double4(0.0, 0.0, scale, 0.0), double4(0.0, 0.0, 0.0, 1.0));
  }

  #[inline(always)]
  pub fn from_translation(x: f64, y: f64, z: f64) -> double4x4 {
    return double4x4(double4(1.0, 0.0, 0.0, 0.0), double4(0.0, 1.0, 0.0, 0.0), double4(0.0, 0.0, 1.0, 0.0), double4(x, y, z, 1.0));
  }

  #[inline(always)]
  pub fn from_euler_angles(roll: f64, pitch: f64, yaw: f64) -> double4x4 {
    let (sr, cr) = roll.sin_cos();
    let (sp, cp) = pitch.sin_cos();
    let (sy, cy) = yaw.sin_cos();

    return double4x4(
      double4(cy * cp, sy * cp, -sp, 0.0),
      double4(cy * sp * sr - sy * cr, sy * sp * sr + cy * cr, cp * sr, 0.0),
      double4(cy * sp * cr + sy * sr, sy * sp * cr - cy * sr, cp * cr, 0.0),
      double4(0.0, 0.0, 0.0, 1.0)
    );
  }

  #[inline(always)]
  pub fn look_at(eye: double3, center: double3, up: double3) -> double4x4 {
    let z = (center - eye).normalize();
    let x = up.cross(z).normalize();
    let y = z.cross(x);

    let p = double4(x.0, y.0, z.0, 0.0);
    let q = double4(x.1, y.1, z.1, 0.0);
    let r = double4(x.2, y.2, z.2, 0.0);
    let s = double4(-x.dot(eye), -y.dot(eye), -z.dot(eye), 1.0);

    return double4x4(p, q, r, s);
  }

  #[inline(always)]
  pub fn perspective(width: f64, height: f64, near: f64, far: f64) -> double4x4 {
    let z_near = 2.0 * near;
    let z_far = far / (far - near);

    let p = double4(z_near / width, 0.0, 0.0, 0.0);
    let q = double4(0.0, z_near / height, 0.0, 0.0);
    let r = double4(0.0, 0.0, z_far, 1.0);
    let s = double4(0.0, 0.0, -near * z_far, 0.0);

    return double4x4(p, q, r, s);
  }

  #[inline(always)]
  pub fn perspective_fov(fov_y: f64, aspect: f64, near: f64, far: f64) -> double4x4 {
    let y_scale = 1.0 / (0.5 * fov_y).tan();
    let x_scale = y_scale / aspect;
    let z_scale = far / (far - near);

    let p = double4(x_scale, 0.0, 0.0, 0.0);
    let q = double4(0.0, y_scale, 0.0, 0.0);
    let r = double4(0.0, 0.0, z_scale, 1.0);
    let s = double4(0.0, 0.0, -near * z_scale, 0.0);

    return double4x4(p, q, r, s);
  }

  #[inline(always)]
  pub fn orthographic(left: f64, right: f64, bottom: f64, top: f64, near: f64, far: f64) -> double4x4 {
    let s_length = 1.0 / (right - left);
    let s_height = 1.0 / (top - bottom);
    let s_depth = 1.0 / (far - near);

    let p = double4(2.0 * s_length, 0.0, 0.0, 0.0);
    let q = double4(0.0, 2.0 * s_height, 0.0, 0.0);
    let r = double4(0.0, 0.0, s_depth, 0.0);
    let s = double4(0.0, 0.0, -near * s_depth, 1.0);

    return double4x4(p, q, r, s);
  }

  #[inline(always)]
  pub fn linear_combination(a: f64, x: double4x4, b: f64, y: double4x4) -> double4x4 {
    let a = double4::broadcast(a);
    let b = double4::broadcast(b);
    return double4x4(a * x.0 + b * y.0, a * x.1 + b * y.1, a * x.2 + b * y.2, a * x.3 + b * y.3);
  }

  #[inline(always)]
  pub fn transpose(self) -> double4x4 {
    let c0 = double4((self.0).0, (self.1).0, (self.2).0, (self.3).0);
    let c1 = double4((self.0).1, (self.1).1, (self.2).1, (self.3).1);
    let c2 = double4((self.0).2, (self.1).2, (self.2).2, (self.3).2);
    let c3 = double4((self.0).3, (self.1).3, (self.2).3, (self.3).3);

    return double4x4(c0, c1, c2, c3);
  }

  #[inline(always)]
  pub fn inverse(self) -> double4x4 {
    return unsafe { __invert_d4(self) };
  }
}

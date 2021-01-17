use std::ops::{Add, AddAssign};

pub fn set_panic_hook() {
  // When the `console_error_panic_hook` feature is enabled, we can call the
  // `set_panic_hook` function at least once during initialization, and then
  // we will get better error messages if our code ever panics.
  //
  // For more details see
  // https://github.com/rustwasm/console_error_panic_hook#readme
  #[cfg(feature = "console_error_panic_hook")]
  console_error_panic_hook::set_once();
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec2<T>(pub T, pub T);

impl<T> Vec2<T> {
  fn matr_mult() {

  }
}

impl<T: Add<Output = T>> Add for Vec2<T> {
  type Output = Self;

  fn add(self, other: Self) -> Self::Output {
    Self(self.0 + other.0, self.1 + other.1)
  }
}

impl<T: AddAssign> AddAssign for Vec2<T> {
  fn add_assign(&mut self, other: Self) {
    self.0 += other.0;
    self.1 += other.1;
  }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3<T>(pub T, pub T, pub T);

impl<T: Add<Output = T>> Add for Vec3<T> {
  type Output = Self;

  fn add(self, other: Self) -> Self::Output {
    Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
  }
}

impl<T: AddAssign> AddAssign for Vec3<T> {
  fn add_assign(&mut self, other: Self) {
    self.0 += other.0;
    self.1 += other.1;
    self.2 += other.2;
  }
}
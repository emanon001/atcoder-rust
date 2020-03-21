#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ModInt(u32);

const MOD: u32 = 1_000_000_007;

impl ModInt {
  pub fn inv(self) -> Self {
    if self.0 == 0 {
      panic!();
    }
    self.pow(MOD - 2)
  }

  pub fn pow(self, e: u32) -> Self {
    if e == 0 {
      return Self::new(1);
    }
    let mut res = self.pow(e >> 1);
    res *= res;
    if e & 1 == 1 {
      res *= self;
    }
    res
  }

  fn new(n: i64) -> Self {
    let mut n = n % (MOD as i64);
    if n.is_negative() {
      n += MOD as i64;
    }
    Self(n as u32)
  }
}

impl From<i32> for ModInt {
  fn from(n: i32) -> Self {
    ModInt::from(n as i64)
  }
}

impl From<i64> for ModInt {
  fn from(n: i64) -> Self {
    Self::new(n)
  }
}

impl From<isize> for ModInt {
  fn from(n: isize) -> Self {
    ModInt::from(n as i64)
  }
}

impl From<u32> for ModInt {
  fn from(n: u32) -> Self {
    ModInt::from(n as u64)
  }
}

impl From<u64> for ModInt {
  fn from(n: u64) -> Self {
    Self::new(n as i64)
  }
}

impl From<usize> for ModInt {
  fn from(n: usize) -> Self {
    ModInt::from(n as u64)
  }
}

impl Into<i32> for ModInt {
  fn into(self) -> i32 {
    self.0 as i32
  }
}

impl Into<i64> for ModInt {
  fn into(self) -> i64 {
    self.0 as i64
  }
}

impl Into<isize> for ModInt {
  fn into(self) -> isize {
    self.0 as isize
  }
}

impl Into<u32> for ModInt {
  fn into(self) -> u32 {
    self.0
  }
}

impl Into<u64> for ModInt {
  fn into(self) -> u64 {
    self.0 as u64
  }
}

impl Into<usize> for ModInt {
  fn into(self) -> usize {
    self.0 as usize
  }
}

impl std::fmt::Display for ModInt {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl std::ops::Add for ModInt {
  type Output = Self;

  fn add(self, rhs: Self) -> Self {
    Self::new((self.0 + rhs.0) as i64)
  }
}

impl std::ops::AddAssign for ModInt {
  fn add_assign(&mut self, rhs: Self) {
    *self = *self + rhs;
  }
}

impl std::ops::Div for ModInt {
  type Output = Self;

  fn div(self, rhs: Self) -> Self {
    self * rhs.inv()
  }
}

impl std::ops::DivAssign for ModInt {
  fn div_assign(&mut self, rhs: Self) {
    *self = *self / rhs;
  }
}

impl std::ops::Mul for ModInt {
  type Output = Self;

  fn mul(self, rhs: Self) -> Self {
    Self::new((self.0 as i64) * (rhs.0 as i64))
  }
}

impl std::ops::MulAssign for ModInt {
  fn mul_assign(&mut self, rhs: Self) {
    *self = *self * rhs;
  }
}

impl std::ops::Sub for ModInt {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self {
    Self::new((self.0 as i64) - (rhs.0 as i64))
  }
}

impl std::ops::SubAssign for ModInt {
  fn sub_assign(&mut self, rhs: Self) {
    *self = *self - rhs;
  }
}

#[cfg(test)]
mod tests {
  use super::ModInt;

  #[test]
  fn pow() {
    assert_eq!(ModInt::from(1), ModInt::from(2).pow(0));
    assert_eq!(ModInt::from(2), ModInt::from(2).pow(1));
    assert_eq!(ModInt::from(4), ModInt::from(2).pow(2));
    assert_eq!(ModInt::from(536_870_912), ModInt::from(2).pow(29));
    assert_eq!(ModInt::from(73_741_817), ModInt::from(2).pow(30));
  }

  #[test]
  fn inv() {
    fn t(n: i64) {
      assert_eq!(ModInt::from(1), ModInt::from(n) * ModInt::from(n).inv());
    }
    t(1_000_000_006);
    t(1_000_000_008);
    t(-1_000_000_006);
    t(-1_000_000_008);
  }

  #[test]
  #[should_panic]
  fn inv_panic() {
    fn t(n: i64) {
      assert_eq!(ModInt::from(1), ModInt::from(n) * ModInt::from(n).inv());
    }
    t(1_000_000_007);
  }

  #[test]
  fn from() {
    // i32
    assert_eq!(1_000_000_006_i32, ModInt::from(-1_i32).into());
    assert_eq!(0_i32, ModInt::from(0_i32).into());
    assert_eq!(1_i32, ModInt::from(1_i32).into());
    assert_eq!(1_000_000_006_i32, ModInt::from(1_000_000_006_i32).into());
    assert_eq!(0_i32, ModInt::from(1_000_000_007_i32).into());
    assert_eq!(1_i32, ModInt::from(1_000_000_007_i32 + 1).into());
    // i64
    assert_eq!(1_000_000_006_i32, ModInt::from(1_000_000_006_i64).into());
    assert_eq!(0_i32, ModInt::from(1_000_000_007_i64).into());
    assert_eq!(1_i32, ModInt::from(1_000_000_008_i64).into());
    assert_eq!(1_i32, ModInt::from(1_000_000_007_000_000_001_i64).into());
    // u32
    assert_eq!(1_000_000_006_i32, ModInt::from(1_000_000_006_u32).into());
    assert_eq!(0_i32, ModInt::from(1_000_000_007_u32).into());
    assert_eq!(1_i32, ModInt::from(1_000_000_008_u32).into());
    // u64
    assert_eq!(1_000_000_006_i32, ModInt::from(1_000_000_006_u64).into());
    assert_eq!(0_i32, ModInt::from(1_000_000_007_u64).into());
    assert_eq!(1_i32, ModInt::from(1_000_000_008_u64).into());
    assert_eq!(1_i32, ModInt::from(1_000_000_007_000_000_001_u64).into());
  }

  #[test]
  fn into() {
    assert_eq!(1_000_000_006_i32, ModInt::from(1_000_000_006).into());
    assert_eq!(1_000_000_006_i64, ModInt::from(1_000_000_006).into());
    assert_eq!(1_000_000_006_isize, ModInt::from(1_000_000_006).into());
    assert_eq!(1_000_000_006_u32, ModInt::from(1_000_000_006).into());
    assert_eq!(1_000_000_006_u64, ModInt::from(1_000_000_006).into());
    assert_eq!(1_000_000_006_usize, ModInt::from(1_000_000_006).into());
  }

  #[test]
  fn fmt() {
    assert_eq!("1000000006", format!("{}", ModInt::from(1_000_000_006)));
    assert_eq!("0", format!("{}", ModInt::from(1_000_000_007)));
    assert_eq!("1", format!("{}", ModInt::from(1_000_000_008)));
  }

  #[test]
  fn add() {
    assert_eq!(
      ModInt::from(1_000_000_005),
      ModInt::from(1_000_000_005) + ModInt::from(0)
    );
    assert_eq!(
      ModInt::from(1_000_000_006),
      ModInt::from(1_000_000_005) + ModInt::from(1)
    );
    assert_eq!(
      ModInt::from(0),
      ModInt::from(1_000_000_005) + ModInt::from(2)
    );
    assert_eq!(
      ModInt::from(1),
      ModInt::from(1_000_000_005) + ModInt::from(3)
    );
  }

  #[test]
  fn add_assign() {
    let mut n = ModInt::from(1_000_000_006);
    n += ModInt::from(1);
    assert_eq!(ModInt::from(0), n);

    let mut n = ModInt::from(1_000_000_006);
    n += ModInt::from(2);
    assert_eq!(ModInt::from(1), n);
  }

  #[test]
  fn div() {
    assert_eq!(ModInt::from(1), ModInt::from(1) / ModInt::from(1));
    assert_eq!(
      ModInt::from(0),
      ModInt::from(1_000_000_007) / ModInt::from(1)
    );
    assert_eq!(
      ModInt::from(1),
      ModInt::from(1_000_000_006) / ModInt::from(1_000_000_006)
    );
  }

  #[test]
  fn div_assign() {
    let mut n = ModInt::from(1_000_000_007);
    n /= ModInt::from(1);
    assert_eq!(ModInt::from(0), n);

    let mut n = ModInt::from(1_000_000_006);
    n /= ModInt::from(1_000_000_006);
    assert_eq!(ModInt::from(1), n);
  }

  #[test]
  fn mul() {
    assert_eq!(ModInt::from(0), ModInt::from(1) * ModInt::from(0));
    assert_eq!(ModInt::from(6), ModInt::from(2) * ModInt::from(3));
    assert_eq!(
      ModInt::from(1_000_000_006),
      ModInt::from(2) * ModInt::from(500_000_003)
    );
    assert_eq!(ModInt::from(1), ModInt::from(2) * ModInt::from(500_000_004));
  }

  #[test]
  fn mul_assign() {
    let mut n = ModInt::from(2);
    n *= ModInt::from(500_000_003);
    assert_eq!(ModInt::from(1_000_000_006), n);

    let mut n = ModInt::from(2);
    n *= ModInt::from(500_000_004);
    assert_eq!(ModInt::from(1), n);
  }

  #[test]
  fn sub() {
    assert_eq!(ModInt::from(1), ModInt::from(1) - ModInt::from(0));
    assert_eq!(ModInt::from(1), ModInt::from(3) - ModInt::from(2));
    assert_eq!(ModInt::from(0), ModInt::from(3) - ModInt::from(3));
    assert_eq!(
      ModInt::from(1_000_000_006),
      ModInt::from(1_000_000_007) - ModInt::from(1)
    );
    assert_eq!(
      ModInt::from(0),
      ModInt::from(1_000_000_008) - ModInt::from(1)
    );
  }

  #[test]
  fn sub_assign() {
    let mut n = ModInt::from(1_000_000_007);
    n -= ModInt::from(1);
    assert_eq!(ModInt::from(1_000_000_006), n);

    let mut n = ModInt::from(1_000_000_008);
    n -= ModInt::from(1);
    assert_eq!(ModInt::from(0), n);
  }
}

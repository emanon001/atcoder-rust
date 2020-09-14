use cargo_snippet::snippet;

#[snippet("mod_int")]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ModInt(u32);

#[snippet("mod_int")]
impl ModInt {
    pub const MOD: u32 = 1_000_000_007;

    pub fn inv(self) -> Self {
        if self.0 == 0 {
            panic!();
        }
        self.pow(Self::MOD - 2)
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
        let m = Self::MOD as i64;
        let mut n = n % m;
        if n.is_negative() {
            n += m;
        }
        Self(n as u32)
    }
}

#[snippet("mod_int")]
macro_rules! impl_from {
    ($T:ty) => {
        impl From<$T> for ModInt {
            fn from(n: $T) -> Self {
                use std::convert::TryFrom;
                Self::new(i64::try_from(n).unwrap())
            }
        }
    };
}

#[snippet("mod_int")]
impl_from!(i32);
#[snippet("mod_int")]
impl_from!(i64);
#[snippet("mod_int")]
impl_from!(isize);
#[snippet("mod_int")]
impl_from!(u32);
#[snippet("mod_int")]
impl_from!(u64);
#[snippet("mod_int")]
impl_from!(usize);

#[snippet("mod_int")]
macro_rules! impl_into {
    ($T:ty) => {
        impl Into<$T> for ModInt {
            fn into(self) -> $T {
                self.0 as $T
            }
        }
    };
}

#[snippet("mod_int")]
impl_into!(i32);
#[snippet("mod_int")]
impl_into!(i64);
#[snippet("mod_int")]
impl_into!(isize);
#[snippet("mod_int")]
impl_into!(u32);
#[snippet("mod_int")]
impl_into!(u64);
#[snippet("mod_int")]
impl_into!(usize);

#[snippet("mod_int")]
impl std::fmt::Display for ModInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[snippet("mod_int")]
impl std::ops::Add for ModInt {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new((self.0 + rhs.0) as i64)
    }
}

#[snippet("mod_int")]
impl std::ops::AddAssign for ModInt {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

#[snippet("mod_int")]
impl std::ops::Div for ModInt {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        self * rhs.inv()
    }
}

#[snippet("mod_int")]
impl std::ops::DivAssign for ModInt {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

#[snippet("mod_int")]
impl std::ops::Mul for ModInt {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self::new((self.0 as i64) * (rhs.0 as i64))
    }
}

#[snippet("mod_int")]
impl std::ops::MulAssign for ModInt {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

#[snippet("mod_int")]
impl std::ops::Sub for ModInt {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new((self.0 as i64) - (rhs.0 as i64))
    }
}

#[snippet("mod_int")]
impl std::ops::SubAssign for ModInt {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

#[snippet("mod_int")]
impl num::Zero for ModInt {
    fn zero() -> Self {
        Self::new(0)
    }

    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
}

#[snippet("mod_int")]
impl num::One for ModInt {
    fn one() -> Self {
        Self::new(1)
    }

    fn is_one(&self) -> bool {
        *self == Self::one()
    }
}

#[cfg(test)]
mod tests {
    use super::ModInt;
    use num::{One, Zero};

    #[test]
    fn one() {
        assert_eq!(ModInt::one(), ModInt::from(1));
    }

    #[test]
    fn pow() {
        assert_eq!(ModInt::from(2).pow(0), ModInt::from(1));
        assert_eq!(ModInt::from(2).pow(1), ModInt::from(2));
        assert_eq!(ModInt::from(2).pow(2), ModInt::from(4));
        assert_eq!(ModInt::from(2).pow(29), ModInt::from(536_870_912));
        assert_eq!(ModInt::from(2).pow(30), ModInt::from(73_741_817));
    }

    #[test]
    fn inv() {
        fn t(n: i64) {
            assert_eq!(ModInt::from(n) * ModInt::from(n).inv(), ModInt::from(1));
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
            assert_eq!(ModInt::from(n) * ModInt::from(n).inv(), ModInt::from(1));
        }
        t(1_000_000_007);
    }

    #[test]
    fn zero() {
        assert_eq!(ModInt::zero(), ModInt::from(0));
    }

    #[test]
    fn from() {
        // 型推論のために、(expected, actual) の順番で記述している
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
        // 型推論のために、(expected, actual) の順番で記述している
        assert_eq!(1_000_000_006_i32, ModInt::from(1_000_000_006).into());
        assert_eq!(1_000_000_006_i64, ModInt::from(1_000_000_006).into());
        assert_eq!(1_000_000_006_isize, ModInt::from(1_000_000_006).into());
        assert_eq!(1_000_000_006_u32, ModInt::from(1_000_000_006).into());
        assert_eq!(1_000_000_006_u64, ModInt::from(1_000_000_006).into());
        assert_eq!(1_000_000_006_usize, ModInt::from(1_000_000_006).into());
    }

    #[test]
    fn fmt() {
        assert_eq!(format!("{}", ModInt::from(1_000_000_006)), "1000000006");
        assert_eq!(format!("{}", ModInt::from(1_000_000_007)), "0");
        assert_eq!(format!("{}", ModInt::from(1_000_000_008)), "1");
    }

    #[test]
    fn add() {
        assert_eq!(
            ModInt::from(1_000_000_005) + ModInt::from(0),
            ModInt::from(1_000_000_005),
        );
        assert_eq!(
            ModInt::from(1_000_000_005) + ModInt::from(1),
            ModInt::from(1_000_000_006),
        );
        assert_eq!(
            ModInt::from(1_000_000_005) + ModInt::from(2),
            ModInt::from(0),
        );
        assert_eq!(
            ModInt::from(1_000_000_005) + ModInt::from(3),
            ModInt::from(1),
        );
    }

    #[test]
    fn add_assign() {
        let mut n = ModInt::from(1_000_000_006);
        n += ModInt::from(1);
        assert_eq!(n, ModInt::from(0));

        let mut n = ModInt::from(1_000_000_006);
        n += ModInt::from(2);
        assert_eq!(n, ModInt::from(1));
    }

    #[test]
    fn div() {
        assert_eq!(ModInt::from(1), ModInt::from(1) / ModInt::from(1));
        assert_eq!(
            ModInt::from(1_000_000_007) / ModInt::from(1),
            ModInt::from(0),
        );
        assert_eq!(
            ModInt::from(1_000_000_006) / ModInt::from(1_000_000_006),
            ModInt::from(1),
        );
    }

    #[test]
    fn div_assign() {
        let mut n = ModInt::from(1_000_000_007);
        n /= ModInt::from(1);
        assert_eq!(n, ModInt::from(0));

        let mut n = ModInt::from(1_000_000_006);
        n /= ModInt::from(1_000_000_006);
        assert_eq!(n, ModInt::from(1));
    }

    #[test]
    fn mul() {
        assert_eq!(ModInt::from(0), ModInt::from(1) * ModInt::from(0));
        assert_eq!(ModInt::from(6), ModInt::from(2) * ModInt::from(3));
        assert_eq!(
            ModInt::from(2) * ModInt::from(500_000_003),
            ModInt::from(1_000_000_006),
        );
        assert_eq!(ModInt::from(2) * ModInt::from(500_000_004), ModInt::from(1));
    }

    #[test]
    fn mul_assign() {
        let mut n = ModInt::from(2);
        n *= ModInt::from(500_000_003);
        assert_eq!(n, ModInt::from(1_000_000_006));

        let mut n = ModInt::from(2);
        n *= ModInt::from(500_000_004);
        assert_eq!(n, ModInt::from(1));
    }

    #[test]
    fn sub() {
        assert_eq!(ModInt::from(1), ModInt::from(1) - ModInt::from(0));
        assert_eq!(ModInt::from(1), ModInt::from(3) - ModInt::from(2));
        assert_eq!(ModInt::from(0), ModInt::from(3) - ModInt::from(3));
        assert_eq!(
            ModInt::from(1_000_000_007) - ModInt::from(1),
            ModInt::from(1_000_000_006),
        );
        assert_eq!(
            ModInt::from(1_000_000_008) - ModInt::from(1),
            ModInt::from(0),
        );
    }

    #[test]
    fn sub_assign() {
        let mut n = ModInt::from(1_000_000_007);
        n -= ModInt::from(1);
        assert_eq!(n, ModInt::from(1_000_000_006));

        let mut n = ModInt::from(1_000_000_008);
        n -= ModInt::from(1);
        assert_eq!(n, ModInt::from(0));
    }
}

use cargo_snippet::snippet;

#[snippet("mod_int")]
macro_rules! impl_mod_int {
    ($T: ident, $M: literal) => {
        #[derive(Copy, Clone, Debug, Eq, PartialEq)]
        pub struct $T(u32);

        impl $T {
            pub const MOD: u32 = $M;

            pub fn inv(self) -> Self {
                if self.0 == 0 {
                    panic!();
                }
                self.pow(Self::MOD - 2)
            }

            pub fn pow<T: num::Unsigned + num::PrimInt>(self, e: T) -> Self {
                if e.is_zero() {
                    return Self::new(1);
                }
                let mut res = self.pow(e >> 1);
                res *= res;
                if e & T::one() == T::one() {
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

        impl std::fmt::Display for $T {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl std::ops::Add for $T {
            type Output = Self;

            fn add(self, rhs: Self) -> Self {
                Self::new((self.0 + rhs.0) as i64)
            }
        }

        impl std::ops::AddAssign for $T {
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }

        impl std::ops::Div for $T {
            type Output = Self;

            fn div(self, rhs: Self) -> Self {
                self * rhs.inv()
            }
        }

        impl std::ops::DivAssign for $T {
            fn div_assign(&mut self, rhs: Self) {
                *self = *self / rhs;
            }
        }

        impl std::ops::Mul for $T {
            type Output = Self;

            fn mul(self, rhs: Self) -> Self {
                Self::new((self.0 as i64) * (rhs.0 as i64))
            }
        }

        impl std::ops::MulAssign for $T {
            fn mul_assign(&mut self, rhs: Self) {
                *self = *self * rhs;
            }
        }

        impl std::ops::Sub for $T {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self {
                Self::new((self.0 as i64) - (rhs.0 as i64))
            }
        }

        impl std::ops::SubAssign for $T {
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }

        impl num::Zero for $T {
            fn zero() -> Self {
                Self::new(0)
            }

            fn is_zero(&self) -> bool {
                *self == Self::zero()
            }
        }

        impl num::One for $T {
            fn one() -> Self {
                Self::new(1)
            }

            fn is_one(&self) -> bool {
                *self == Self::one()
            }
        }
    };
}

#[snippet("mod_int")]
impl_mod_int!(ModInt1000000007, 1000000007);
impl_mod_int!(ModInt998244353, 998244353);

#[snippet("mod_int")]
macro_rules! impl_from {
    ($T: ty, $T2: ty) => {
        impl From<$T2> for $T {
            fn from(n: $T2) -> Self {
                use std::convert::TryFrom;
                Self::new(i64::try_from(n).unwrap())
            }
        }
    };
}

#[snippet("mod_int")]
impl_from!(ModInt1000000007, i32);
#[snippet("mod_int")]
impl_from!(ModInt1000000007, i64);
#[snippet("mod_int")]
impl_from!(ModInt1000000007, isize);
#[snippet("mod_int")]
impl_from!(ModInt1000000007, u32);
#[snippet("mod_int")]
impl_from!(ModInt1000000007, u64);
#[snippet("mod_int")]
impl_from!(ModInt1000000007, usize);

#[snippet("mod_int")]
impl_from!(ModInt998244353, i32);
#[snippet("mod_int")]
impl_from!(ModInt998244353, i64);
#[snippet("mod_int")]
impl_from!(ModInt998244353, isize);
#[snippet("mod_int")]
impl_from!(ModInt998244353, u32);
#[snippet("mod_int")]
impl_from!(ModInt998244353, u64);
#[snippet("mod_int")]
impl_from!(ModInt998244353, usize);

#[snippet("mod_int")]
macro_rules! impl_into {
    ($T: ident, $T2: ty) => {
        impl Into<$T2> for $T {
            fn into(self) -> $T2 {
                self.0 as $T2
            }
        }
    };
}

#[snippet("mod_int")]
impl_into!(ModInt1000000007, i32);
#[snippet("mod_int")]
impl_into!(ModInt1000000007, i64);
#[snippet("mod_int")]
impl_into!(ModInt1000000007, isize);
#[snippet("mod_int")]
impl_into!(ModInt1000000007, u32);
#[snippet("mod_int")]
impl_into!(ModInt1000000007, u64);
#[snippet("mod_int")]
impl_into!(ModInt1000000007, usize);

#[snippet("mod_int")]
impl_into!(ModInt998244353, i32);
#[snippet("mod_int")]
impl_into!(ModInt998244353, i64);
#[snippet("mod_int")]
impl_into!(ModInt998244353, isize);
#[snippet("mod_int")]
impl_into!(ModInt998244353, u32);
#[snippet("mod_int")]
impl_into!(ModInt998244353, u64);
#[snippet("mod_int")]
impl_into!(ModInt998244353, usize);

#[cfg(test)]
mod tests {
    use super::*;
    use num::*;

    #[test]
    fn one() {
        assert_eq!(ModInt1000000007::one(), ModInt1000000007::from(1));
    }

    #[test]
    fn pow() {
        assert_eq!(
            ModInt1000000007::from(2).pow(0_usize),
            ModInt1000000007::from(1)
        );
        assert_eq!(
            ModInt1000000007::from(2).pow(1_u32),
            ModInt1000000007::from(2)
        );
        assert_eq!(
            ModInt1000000007::from(2).pow(2_u64),
            ModInt1000000007::from(4)
        );
        assert_eq!(
            ModInt1000000007::from(2).pow(29_u128),
            ModInt1000000007::from(536_870_912)
        );
        assert_eq!(
            ModInt1000000007::from(2).pow(30_u128),
            ModInt1000000007::from(73_741_817)
        );
    }

    #[test]
    fn inv() {
        fn t(n: i64) {
            assert_eq!(
                ModInt1000000007::from(n) * ModInt1000000007::from(n).inv(),
                ModInt1000000007::from(1)
            );
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
            assert_eq!(
                ModInt1000000007::from(n) * ModInt1000000007::from(n).inv(),
                ModInt1000000007::from(1)
            );
        }
        t(1_000_000_007);
    }

    #[test]
    fn zero() {
        assert_eq!(ModInt1000000007::zero(), ModInt1000000007::from(0));
    }

    #[test]
    fn from() {
        // 型推論のために、(expected, actual) の順番で記述している
        // i32
        assert_eq!(1_000_000_006_i32, ModInt1000000007::from(-1_i32).into());
        assert_eq!(0_i32, ModInt1000000007::from(0_i32).into());
        assert_eq!(1_i32, ModInt1000000007::from(1_i32).into());
        assert_eq!(
            1_000_000_006_i32,
            ModInt1000000007::from(1_000_000_006_i32).into()
        );
        assert_eq!(0_i32, ModInt1000000007::from(1_000_000_007_i32).into());
        assert_eq!(1_i32, ModInt1000000007::from(1_000_000_007_i32 + 1).into());
        // i64
        assert_eq!(
            1_000_000_006_i32,
            ModInt1000000007::from(1_000_000_006_i64).into()
        );
        assert_eq!(0_i32, ModInt1000000007::from(1_000_000_007_i64).into());
        assert_eq!(1_i32, ModInt1000000007::from(1_000_000_008_i64).into());
        assert_eq!(
            1_i32,
            ModInt1000000007::from(1_000_000_007_000_000_001_i64).into()
        );
        // u32
        assert_eq!(
            1_000_000_006_i32,
            ModInt1000000007::from(1_000_000_006_u32).into()
        );
        assert_eq!(0_i32, ModInt1000000007::from(1_000_000_007_u32).into());
        assert_eq!(1_i32, ModInt1000000007::from(1_000_000_008_u32).into());
        // u64
        assert_eq!(
            1_000_000_006_i32,
            ModInt1000000007::from(1_000_000_006_u64).into()
        );
        assert_eq!(0_i32, ModInt1000000007::from(1_000_000_007_u64).into());
        assert_eq!(1_i32, ModInt1000000007::from(1_000_000_008_u64).into());
        assert_eq!(
            1_i32,
            ModInt1000000007::from(1_000_000_007_000_000_001_u64).into()
        );
    }

    #[test]
    fn into() {
        // 型推論のために、(expected, actual) の順番で記述している
        assert_eq!(
            1_000_000_006_i32,
            ModInt1000000007::from(1_000_000_006).into()
        );
        assert_eq!(
            1_000_000_006_i64,
            ModInt1000000007::from(1_000_000_006).into()
        );
        assert_eq!(
            1_000_000_006_isize,
            ModInt1000000007::from(1_000_000_006).into()
        );
        assert_eq!(
            1_000_000_006_u32,
            ModInt1000000007::from(1_000_000_006).into()
        );
        assert_eq!(
            1_000_000_006_u64,
            ModInt1000000007::from(1_000_000_006).into()
        );
        assert_eq!(
            1_000_000_006_usize,
            ModInt1000000007::from(1_000_000_006).into()
        );
    }

    #[test]
    fn fmt() {
        assert_eq!(
            format!("{}", ModInt1000000007::from(1_000_000_006)),
            "1000000006"
        );
        assert_eq!(format!("{}", ModInt1000000007::from(1_000_000_007)), "0");
        assert_eq!(format!("{}", ModInt1000000007::from(1_000_000_008)), "1");
    }

    #[test]
    fn add() {
        assert_eq!(
            ModInt1000000007::from(1_000_000_005) + ModInt1000000007::from(0),
            ModInt1000000007::from(1_000_000_005),
        );
        assert_eq!(
            ModInt1000000007::from(1_000_000_005) + ModInt1000000007::from(1),
            ModInt1000000007::from(1_000_000_006),
        );
        assert_eq!(
            ModInt1000000007::from(1_000_000_005) + ModInt1000000007::from(2),
            ModInt1000000007::from(0),
        );
        assert_eq!(
            ModInt1000000007::from(1_000_000_005) + ModInt1000000007::from(3),
            ModInt1000000007::from(1),
        );
    }

    #[test]
    fn add_assign() {
        let mut n = ModInt1000000007::from(1_000_000_006);
        n += ModInt1000000007::from(1);
        assert_eq!(n, ModInt1000000007::from(0));

        let mut n = ModInt1000000007::from(1_000_000_006);
        n += ModInt1000000007::from(2);
        assert_eq!(n, ModInt1000000007::from(1));
    }

    #[test]
    fn div() {
        assert_eq!(
            ModInt1000000007::from(1),
            ModInt1000000007::from(1) / ModInt1000000007::from(1)
        );
        assert_eq!(
            ModInt1000000007::from(1_000_000_007) / ModInt1000000007::from(1),
            ModInt1000000007::from(0),
        );
        assert_eq!(
            ModInt1000000007::from(1_000_000_006) / ModInt1000000007::from(1_000_000_006),
            ModInt1000000007::from(1),
        );
    }

    #[test]
    fn div_assign() {
        let mut n = ModInt1000000007::from(1_000_000_007);
        n /= ModInt1000000007::from(1);
        assert_eq!(n, ModInt1000000007::from(0));

        let mut n = ModInt1000000007::from(1_000_000_006);
        n /= ModInt1000000007::from(1_000_000_006);
        assert_eq!(n, ModInt1000000007::from(1));
    }

    #[test]
    fn mul() {
        assert_eq!(
            ModInt1000000007::from(0),
            ModInt1000000007::from(1) * ModInt1000000007::from(0)
        );
        assert_eq!(
            ModInt1000000007::from(6),
            ModInt1000000007::from(2) * ModInt1000000007::from(3)
        );
        assert_eq!(
            ModInt1000000007::from(2) * ModInt1000000007::from(500_000_003),
            ModInt1000000007::from(1_000_000_006),
        );
        assert_eq!(
            ModInt1000000007::from(2) * ModInt1000000007::from(500_000_004),
            ModInt1000000007::from(1)
        );
    }

    #[test]
    fn mul_assign() {
        let mut n = ModInt1000000007::from(2);
        n *= ModInt1000000007::from(500_000_003);
        assert_eq!(n, ModInt1000000007::from(1_000_000_006));

        let mut n = ModInt1000000007::from(2);
        n *= ModInt1000000007::from(500_000_004);
        assert_eq!(n, ModInt1000000007::from(1));
    }

    #[test]
    fn sub() {
        assert_eq!(
            ModInt1000000007::from(1),
            ModInt1000000007::from(1) - ModInt1000000007::from(0)
        );
        assert_eq!(
            ModInt1000000007::from(1),
            ModInt1000000007::from(3) - ModInt1000000007::from(2)
        );
        assert_eq!(
            ModInt1000000007::from(0),
            ModInt1000000007::from(3) - ModInt1000000007::from(3)
        );
        assert_eq!(
            ModInt1000000007::from(1_000_000_007) - ModInt1000000007::from(1),
            ModInt1000000007::from(1_000_000_006),
        );
        assert_eq!(
            ModInt1000000007::from(1_000_000_008) - ModInt1000000007::from(1),
            ModInt1000000007::from(0),
        );
    }

    #[test]
    fn sub_assign() {
        let mut n = ModInt1000000007::from(1_000_000_007);
        n -= ModInt1000000007::from(1);
        assert_eq!(n, ModInt1000000007::from(1_000_000_006));

        let mut n = ModInt1000000007::from(1_000_000_008);
        n -= ModInt1000000007::from(1);
        assert_eq!(n, ModInt1000000007::from(0));
    }
}

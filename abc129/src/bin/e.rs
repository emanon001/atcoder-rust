#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ModInt(u32);

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
        let mut n = n % (Self::MOD as i64);
        if n.is_negative() {
            n += Self::MOD as i64;
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

fn main() {
    input! {
      l: Chars
    };

    let l = l
        .into_iter()
        .map(|ch| ch.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    let len = l.len();
    // dp[i][j] = 個数
    // i: 何桁目まで見たか
    // j: 0: nと同じ、1: n未満
    let mut dp = vec![vec![None; 2]; len + 1];
    dp[0][0] = Some(ModInt::from(1));
    for i in 0..len {
        for j in 0..2 {
            let n = l[i];
            let max_n = if j == 0 { n } else { 1 };
            for m in 0..=max_n {
                let to_j = if j == 1 || m < n { 1 } else { 0 };
                let c = ModInt::from(if m == 0 { 1 } else { 2 });
                let v = match (dp[i + 1][to_j], dp[i][j]) {
                    (x, None) => x,
                    (None, x) => x.map(|y| y * c),
                    (Some(x), Some(y)) => Some(x + y * c),
                };
                dp[i + 1][to_j] = v;
            }
        }
    }

    println!(
        "{}",
        dp[len][0].unwrap_or(ModInt::from(0)) + dp[len][1].unwrap_or(ModInt::from(0))
    );
}

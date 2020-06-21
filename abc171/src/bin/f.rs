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

    pub fn one() -> Self {
        Self(1)
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

    pub fn zero() -> Self {
        Self(0)
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

pub struct ModComb {
    max: usize,
    fac: Vec<ModInt>,
    finv: Vec<ModInt>,
}

impl ModComb {
    pub fn new(max: usize) -> Self {
        let mut fac = vec![ModInt::zero(); max + 1];
        let mut finv = vec![ModInt::zero(); max + 1];
        let mut inv = vec![ModInt::zero(); max + 1];
        fac[0] = ModInt::one();
        fac[1] = ModInt::one();
        finv[0] = ModInt::one();
        finv[1] = ModInt::one();
        inv[1] = ModInt::one();
        let modulo = ModInt::MOD as usize;
        for i in 2..=max {
            fac[i] = fac[i - 1] * ModInt::from(i);
            inv[i] =
                ModInt::from(ModInt::from(modulo) - (inv[modulo % i] * ModInt::from(modulo / i)));
            finv[i] = finv[i - 1] * inv[i]
        }
        Self { max, fac, finv }
    }

    pub fn comb(&self, n: usize, k: usize) -> ModInt {
        if n > self.max {
            panic!();
        }
        if n < k {
            return ModInt::zero();
        }
        self.fac[n] * self.finv[k] * self.finv[n - k]
    }
}

fn solve() {
    input! {
        k: usize,
        s: Chars
    };

    let n = s.len();
    let comb = ModComb::new(n + k);
    let mut res = ModInt::one();
    for i in 0..k {
        let v = ModInt::from(26) * comb.comb(n + i + 1, 1);
        res *= v;
    }
    println!("{}", res);
}

fn main() {
    std::thread::Builder::new()
        .name("big stack size".into())
        .stack_size(256 * 1024 * 1024)
        .spawn(|| {
            solve();
        })
        .unwrap()
        .join()
        .unwrap();
}

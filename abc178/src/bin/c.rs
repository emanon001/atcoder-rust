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
        let m = Self::MOD as i64;
        let mut n = n % m;
        if n.is_negative() {
            n += m;
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
impl num::Zero for ModInt {
    fn zero() -> Self {
        Self::new(0)
    }
    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
}
impl num::One for ModInt {
    fn one() -> Self {
        Self::new(1)
    }
    fn is_one(&self) -> bool {
        *self == Self::one()
    }
}
pub struct ModComb {
    n: usize,
    fact: Vec<ModInt>,
    ifact: Vec<ModInt>,
}
impl ModComb {
    pub fn new(n: usize) -> Self {
        assert!(n < ModInt::MOD as usize);
        let mut fact = vec![ModInt::zero(); n + 1];
        let mut ifact = vec![ModInt::zero(); n + 1];
        fact[0] = ModInt::one();
        for i in 1..=n {
            fact[i] = fact[i - 1] * ModInt::from(i);
        }
        ifact[n] = fact[n].inv();
        for i in (1..=n).rev() {
            ifact[i - 1] = ifact[i] * ModInt::from(i);
        }
        Self { n, fact, ifact }
    }
    pub fn c(&self, n: usize, k: usize) -> ModInt {
        assert!(n <= self.n);
        if k > n {
            return ModInt::zero();
        }
        self.fact[n] * self.ifact[k] * self.ifact[n - k]
    }
    pub fn p(&self, n: usize, k: usize) -> ModInt {
        assert!(n <= self.n);
        if k > n {
            return ModInt::zero();
        }
        self.fact[n] * self.ifact[n - k]
    }
}

fn solve() {
    input! {
        n: usize
    };

    let comb = ModComb::new(n);
    let mut res = ModInt::zero();
    for k in 2..=n {
        let a = comb.c(n, k);
        let b = ModInt::from(2).pow(k as u32) - ModInt::from(2);
        let c = ModInt::from(8).pow((n - k) as u32);
        res += a * b * c;
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

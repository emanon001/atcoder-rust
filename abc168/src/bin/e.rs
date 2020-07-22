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

fn normalize(pair: (i64, i64)) -> (i64, i64) {
    let (a, b) = pair;
    if a == 0 && b == 0 {
        return (a, b);
    }
    let gcd = a.gcd(&b);
    let a = a / gcd;
    let b = b / gcd;
    if a == 0 || b == 0 {
        let a = if a != 0 { 1 } else { a };
        let b = if b != 0 { 1 } else { b };
        return (a, b);
    }
    let half_negative = a.is_negative() && !b.is_negative() || !a.is_negative() && b.is_negative();
    if half_negative {
        let a = a.abs();
        let b = b.abs();
        (a * -1, b)
    } else {
        (a.abs(), b.abs())
    }
}

fn main() {
    input! {
        n: usize,
        abv: [(i64, i64); n]
    };

    let mut counts = HashMap::new();
    for ab in abv {
        let pair = normalize(ab);
        *counts.entry(pair).or_insert(0_usize) += 1;
    }
    let mut res = ModInt::one();
    let zero_count = ModInt::from(*counts.get(&(0, 0)).unwrap_or(&0));
    let keys = counts.keys().copied().collect::<Vec<_>>();
    for k in keys {
        if !counts.contains_key(&k) {
            continue;
        }

        if k == (0, 0) {
            continue;
        }

        let (a, b) = k;
        let mut count = ModInt::zero();
        let c1 = *counts.get(&k).unwrap();
        count += ModInt::from(2).pow(c1 as u32) - ModInt::one();
        let k2 = normalize((b, -a));
        match counts.get(&k2) {
            Some(&c2) => {
                count += ModInt::from(2).pow(c2 as u32) - ModInt::one();
            }
            None => {}
        };
        count += ModInt::one();
        res *= count;

        counts.remove(&k);
        counts.remove(&k2);
    }
    res += zero_count;
    res -= ModInt::one();
    println!("{}", res);
}

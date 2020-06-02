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

    fn new(n: i64) -> Self {
        let mut n = n % (Self::MOD as i64);
        if n.is_negative() {
            n += Self::MOD as i64;
        }
        Self(n as u32)
    }

    pub fn zero() -> Self {
        Self(0)
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

fn solve() {
    input! {
        h: usize, w: usize,
        grid: [[usize; w]; h]
    };

    let mut pos_table = BTreeMap::new();
    for i in 0..h {
        for j in 0..w {
            let cell = grid[i][j];
            let x = pos_table.entry(cell).or_insert(Vec::new());
            x.push((i, j));
        }
    }

    let mut counts = vec![vec![ModInt::one(); w]; h];
    let di = vec![-1, 0, 1, 0];
    let dj = vec![0, 1, 0, -1];
    for (_, v) in pos_table {
        for (i, j) in v {
            for d in 0..4 {
                let new_i = (i as isize) + di[d];
                let new_j = (j as isize) + dj[d];
                if new_i < 0 || new_i >= (h as isize) || new_j < 0 || new_j >= (w as isize) {
                    continue;
                }
                let new_i = new_i as usize;
                let new_j = new_j as usize;
                if grid[new_i][new_j] > grid[i][j] {
                    let v = counts[i][j];
                    counts[new_i][new_j] += v;
                }
            }
        }
    }
    let mut res = ModInt::zero();
    for i in 0..h {
        for j in 0..w {
            res += counts[i][j];
        }
    }
    println!("{}", res);
}

fn main() {
    std::thread::Builder::new()
        .name("big stack size".into())
        .stack_size(32 * 1024 * 1024)
        .spawn(|| {
            solve();
        })
        .unwrap()
        .join()
        .unwrap();
}

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
macro_rules! impl_from {
    ($ T : ty ) => {
        impl From<$T> for ModInt {
            fn from(n: $T) -> Self {
                use std::convert::TryFrom;
                Self::new(i64::try_from(n).unwrap())
            }
        }
    };
}
impl_from!(i32);
impl_from!(i64);
impl_from!(isize);
impl_from!(u32);
impl_from!(u64);
impl_from!(usize);
macro_rules! impl_into {
    ($ T : ty ) => {
        impl Into<$T> for ModInt {
            fn into(self) -> $T {
                self.0 as $T
            }
        }
    };
}
impl_into!(i32);
impl_into!(i64);
impl_into!(isize);
impl_into!(u32);
impl_into!(u64);
impl_into!(usize);
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

/// `T` is numeric only
pub struct Bit<T>
where
    T: std::ops::AddAssign + std::ops::Sub<Output = T> + num::Zero + Clone,
{
    n: usize,
    data: Vec<T>,
}
/// 0-origin
/// [0, n)
impl<T> Bit<T>
where
    T: std::ops::AddAssign + std::ops::Sub<Output = T> + num::Zero + Clone,
{
    pub fn new(n: usize) -> Self {
        Self {
            n,
            data: vec![T::zero(); n + 1],
        }
    }
    /// 0-origin
    pub fn add(&mut self, i: usize, x: T) {
        if i >= self.n {
            panic!();
        }
        let mut i = i + 1;
        while i <= self.n {
            self.data[i] += x.clone();
            i += ((i as isize) & -(i as isize)) as usize;
        }
    }
    /// [0, i)
    pub fn sum(&self, i: usize) -> T {
        if i > self.n {
            panic!();
        }
        let mut i = i;
        let mut res = T::zero();
        while i > 0 {
            res += self.data[i].clone();
            i -= ((i as isize) & -(i as isize)) as usize;
        }
        res
    }
    /// [i, j)
    pub fn range_sum(&self, i: usize, j: usize) -> T {
        if i > self.n || j > self.n {
            panic!();
        }
        if i >= j {
            return T::zero();
        }
        self.sum(j) - self.sum(i)
    }
}

fn solve() {
    input! {
        n: usize, w: i64, l: i64, r: i64,
        xv: [i64; n]
    };

    let mut xv = xv.into_iter().collect::<VecDeque<_>>();
    xv.push_front(0);
    xv.push_back(w);
    let mut map = BTreeMap::new();
    for (i, x) in xv.iter().copied().enumerate() {
        map.insert(x, i);
    }
    let mut bit: Bit<ModInt> = Bit::new(xv.len());
    bit.add(0, 1.into());
    for i in 0..xv.len() {
        let x = xv[i];
        let mut range = map.range((x - r)..=(x - l));
        let l = range.clone().next();
        let r = range.next_back();
        if let (Some((_, &lj)), Some((_, &rj))) = (l, r) {
            let sum = bit.range_sum(lj, rj + 1);
            bit.add(i, sum);
        }
    }
    println!("{}", bit.range_sum(xv.len() - 1, xv.len()));
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

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
    pub const MOD: u32 = 998244353;
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

fn to_i(c: char) -> usize {
    (c as usize) - 65
}

fn solve() {
    input! {
        n: usize,
        s: Chars
    };

    let chars = vec!['_', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J'];
    let max_j = 1_usize << 10;
    let mut dp = vec![HashMap::new(); n + 1];
    dp[0].insert((0_usize, '_'), ModInt::one());
    for i in 0..n {
        let ch = s[i];
        let ch_pos = to_i(ch);
        for j in 0..max_j {
            for &last_ch in &chars {
                let key = (j, last_ch);
                let new_v = *dp[i + 1].get(&key).unwrap_or(&ModInt::zero()) + *dp[i].get(&key).unwrap_or(&ModInt::zero());
                dp[i + 1].insert(key, new_v);
                if (1 << ch_pos) & j == 0 || last_ch == ch {
                    let new_j = if last_ch == ch {
                        j
                    } else if ch == '_' {
                        j
                    } else {
                        j | (1 << to_i(ch))
                    };
                    let new_key = (new_j, ch);
                    let new_v = *dp[i + 1].get(&new_key).unwrap_or(&ModInt::zero()) + *dp[i].get(&key).unwrap_or(&ModInt::zero());
                    dp[i + 1].insert(new_key, new_v);
                }
            }
        }
        // eprintln!("{:?}", dp[i + 1]);
    }
    let mut res = ModInt::zero();
    for (key, v) in dp[n].clone() {
        if key.1 != '_' {
            res += v;
        }
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

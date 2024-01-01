#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

pub struct UnionFind {
    n: usize,
    root: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<usize>,
}
impl UnionFind {
    pub fn new(n: usize) -> Self {
        let root = (0..n).collect();
        let rank = vec![0; n];
        let size = vec![1; n];
        Self {
            n,
            root,
            rank,
            size,
        }
    }
    pub fn find(&mut self, x: usize) -> usize {
        assert!(x < self.n);
        if self.root[x] == x {
            x
        } else {
            let root = self.find(self.root[x]);
            self.root[x] = root;
            root
        }
    }
    pub fn unite(&mut self, x: usize, y: usize) {
        assert!(x < self.n && y < self.n);
        let x_root = self.find(x);
        let y_root = self.find(y);
        if x_root == y_root {
            return;
        }
        if self.rank[x_root] < self.rank[y_root] {
            self.root[x_root] = y_root;
            self.size[y_root] += self.size[x_root];
        } else {
            self.root[y_root] = x_root;
            self.size[x_root] += self.size[y_root];
            if self.rank[x_root] == self.rank[y_root] {
                self.rank[x_root] += 1;
            }
        }
    }
    pub fn size(&mut self, x: usize) -> usize {
        assert!(x < self.n);
        let x_root = self.find(x);
        self.size[x_root]
    }
    pub fn is_same(&mut self, x: usize, y: usize) -> bool {
        assert!(x < self.n && y < self.n);
        self.find(x) == self.find(y)
    }
    pub fn groups(&mut self) -> Vec<Vec<usize>> {
        let mut groups = std::collections::HashMap::new();
        for x in 0..self.n {
            let k = self.find(x);
            groups.entry(k).or_insert(Vec::new()).push(x);
        }
        groups.values().cloned().collect::<Vec<_>>()
    }
}

/// 上下左右 (i, j)
pub const UDLR_DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

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

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        H: usize, W: usize,
        grid: [Chars; H],
    };

    let mut uf = UnionFind::new(H * W);
    for i in 0..H {
        for j in 0..W {
            if grid[i][j] == '.' {
                continue;
            }
            let u = i * W + j;
            for (di, dj) in &UDLR_DIRS {
                let ni = i as isize + di;
                let nj = j as isize + dj;
                if ni < 0 || ni >= H as isize || nj < 0 || nj >= W as isize {
                    continue;
                }
                let ni = ni as usize;
                let nj = nj as usize;
                if grid[ni][nj] == '.' {
                    continue;
                }
                let v = ni * W + nj;
                uf.unite(u, v);
            }
        }
    }

    // 緑グリッドの連結成分数を取得
    let mut green_groups = HashSet::new();
    for i in 0..H {
        for j in 0..W {
            if grid[i][j] == '.' {
                continue;
            }
            let u = i * W + j;
            green_groups.insert(uf.find(u));
        }
    }

    let mut red_count = ModInt::zero();
    let mut sum = ModInt::from(0);
    for i in 0..H {
        for j in 0..W {
            if grid[i][j] == '#' {
                continue;
            }
            red_count += 1.into();
            let mut near_green_groups = HashSet::new();
            for (di, dj) in &UDLR_DIRS {
                let ni = i as isize + di;
                let nj = j as isize + dj;
                if ni < 0 || ni >= H as isize || nj < 0 || nj >= W as isize {
                    continue;
                }
                let ni = ni as usize;
                let nj = nj as usize;
                if grid[ni][nj] == '.' {
                    continue;
                }
                let u = ni * W + nj;
                near_green_groups.insert(uf.find(u));
            }
            sum += (green_groups.len() - near_green_groups.len() + 1).into();
        }
    }
    let ans = sum / red_count;
    println!("{}", ans);
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

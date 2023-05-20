#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

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
        n: usize,
        av: [usize; n]
    };

    let max = 10.pow(5) * 2 + 10;
    let mut bit: Bit<u64> = Bit::new(max);
    for &a in &av {
        bit.add(a, 1);
    }
    let mut res = 0;
    for a in av {
        res += bit.sum(a) * bit.range_sum(a + 1, max);
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

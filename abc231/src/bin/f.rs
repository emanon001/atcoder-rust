#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

pub fn bsearch<F>(ok: i64, ng: i64, pred: F) -> Option<i64>
where
    F: Fn(i64) -> bool,
{
    let orig_ok = ok;
    let mut ok = ok;
    let mut ng = ng;
    while (ok - ng).abs() > 1 {
        let mid = (ok + ng) / 2;
        if pred(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    if ok == orig_ok {
        None
    } else {
        Some(ok)
    }
}

pub fn compress_zahyo<T: Ord + std::hash::Hash>(
    zahyo: &[T],
) -> std::collections::HashMap<&T, usize> {
    let mut set = std::collections::BTreeSet::new();
    for x in zahyo {
        set.insert(x);
    }
    let mut map = std::collections::HashMap::new();
    for (i, x) in set.into_iter().enumerate() {
        map.insert(x, i);
    }
    map
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
        n: usize,
        av: [i64; n],
        bv: [i64; n],
    };

    let compressed = compress_zahyo(&bv);
    let mut abv = Vec::new();
    for i in 0..n {
        abv.push((av[i], bv[i]));
    }
    let mut counts = HashMap::new();
    for i in 0..n {
        *counts.entry((av[i], bv[i])).or_insert(0) += 1;
    }
    abv.sort_by_key(|&(a, b)| (a, -b));
    let mut res = 0_i64;
    let mut bit = Bit::new(n + 10);
    let mut cur = (-1, -1);
    let mut cur_c = 0;
    for i in 0..n {
        let ab = abv[i];
        if cur.0 != ab.0 || cur.1 != ab.1 {
            cur = ab;
            cur_c = 0;
        }
        cur_c += 1;
        let (a, b) = ab;
        let count = counts[&(a as i64, b as i64)];
        // println!("{}", count);
        let b = compressed[&b];
        if cur_c == count {
            let c = (1_i64 + bit.sum(n) - bit.range_sum(0, b)) * count as i64;
            // println!("a: {} b: {} c: {}", a, b, c);
            res += c;
        }
        bit.add(b, 1);
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

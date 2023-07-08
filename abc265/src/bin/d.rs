#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

pub trait BinarySearchOk<T>: PartialEq + Copy {
    fn bs_needs_next_search(&self, ng: &T) -> bool;
    fn bs_mid_value(&self, ng: &T) -> Self;
    fn bs_into(&self) -> T;
}
impl BinarySearchOk<i64> for i64 {
    fn bs_needs_next_search(&self, ng: &Self) -> bool {
        (self - ng).abs() > 1
    }
    fn bs_mid_value(&self, ng: &Self) -> Self {
        (self + ng) / 2
    }
    fn bs_into(&self) -> Self {
        *self
    }
}
impl BinarySearchOk<f64> for f64 {
    fn bs_needs_next_search(&self, ng: &Self) -> bool {
        (self - ng).abs() > 1.0
    }
    fn bs_mid_value(&self, ng: &Self) -> Self {
        (self + ng) / 2.0
    }
    fn bs_into(&self) -> Self {
        *self
    }
}
impl BinarySearchOk<usize> for usize {
    fn bs_needs_next_search(&self, ng: &Self) -> bool {
        (*self as isize - *ng as isize).abs() > 1
    }
    fn bs_mid_value(&self, ng: &Self) -> Self {
        (self + ng) / 2
    }
    fn bs_into(&self) -> Self {
        *self
    }
}
pub fn bsearch<T, Num: BinarySearchOk<T>, F>(ok: Num, ng: T, pred: F) -> Option<Num>
where
    F: Fn(T) -> bool,
{
    let orig_ok = ok;
    let mut ok = ok;
    let mut ng = ng;
    while ok.bs_needs_next_search(&ng) {
        let mid = ok.bs_mid_value(&ng);
        if pred(mid.bs_into()) {
            ok = mid;
        } else {
            ng = mid.bs_into();
        }
    }
    if ok == orig_ok {
        None
    } else {
        Some(ok)
    }
}

fn solve() {
    input! {
        n: usize, p: i64, q: i64, r: i64,
        av: [i64; n]
    };

    let mut cusum = vec![0_i64; n + 1];
    for i in 0..n {
        cusum[i + 1] = cusum[i] + av[i];
    }

    for x in 0..n {
        let y = bsearch(x, cusum.len(), |y| cusum[y] - cusum[x] <= p);
        if y.is_none() {
            continue;
        }
        let y = y.unwrap();
        if cusum[y] - cusum[x] != p {
            continue;
        }
        // eprintln!("x: {}, y: {}", x, y);

        let z = bsearch(y, cusum.len(), |z| cusum[z] - cusum[y] <= q);
        if z.is_none() {
            continue;
        }
        let z = z.unwrap();
        if cusum[z] - cusum[y] != q {
            continue;
        }
        // eprintln!("y: {}, z: {}", y, z);

        let w = bsearch(z, cusum.len(), |w| cusum[w] - cusum[z] <= r);
        if w.is_none() {
            continue;
        }
        let w = w.unwrap();
        if cusum[w] - cusum[z] != r {
            continue;
        }
        // eprintln!("z: {}, w: {}", z, w);
        println!("Yes");
        return;
    }
    println!("No");
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

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

pub struct Bit {
    n: usize,
    data: Vec<i64>,
}
impl Bit {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            data: vec![0; n + 1],
        }
    }
    pub fn add(&mut self, i: usize, x: i64) {
        if i >= self.n {
            panic!();
        }
        let mut i = i + 1;
        while i <= self.n {
            self.data[i] += x;
            i += ((i as isize) & -(i as isize)) as usize;
        }
    }
    pub fn sum(&self, i: usize) -> i64 {
        if i > self.n {
            panic!();
        }
        let mut i = i;
        let mut res = 0;
        while i > 0 {
            res += self.data[i];
            i -= ((i as isize) & -(i as isize)) as usize;
        }
        res
    }
}

fn solve() {
    input! {
        q: usize,
        queries: [(usize, usize); q]
    };

    let max_x = 2 * 10.pow(5);
    let mut bit = Bit::new(max_x + 1);
    for (t, x) in queries {
        if t == 1 {
            bit.add(x, 1);
        } else {
            let res = bsearch((max_x + 1) as i64, 0, |a| {
                let a = a as usize;
                let bit = &bit;
                let sum = bit.sum(a + 1);
                sum >= x as i64
            })
            .unwrap();
            println!("{}", res);
            bit.add(res as usize, -1);
        }
    }
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

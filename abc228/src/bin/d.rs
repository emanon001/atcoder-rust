#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn solve() {
    input! {
        q: usize,
        mut queries: [(usize, i64); q]
    };
    let n = 2.pow(20);
    let mut v = vec![-1; n];
    let mut set = (0..n).collect::<BTreeSet<_>>();
    for (t, x) in queries {
        match t {
            1 => {
                let pos = x as usize % n;
                if set.range(pos..).next().is_some() {
                    let pos = *set.range(pos..).next().unwrap();
                    v[pos] = x;
                    set.remove(&pos);
                } else if set.range(0..pos).next().is_some() {
                    let pos = *set.range(0..pos).next().unwrap();
                    v[pos] = x;
                    set.remove(&pos);
                }
            }
            2 => {
                let pos = x as usize % n;
                let res = v[pos];
                println!("{}", res);
            }
            _ => unreachable!(),
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

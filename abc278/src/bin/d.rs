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
        n: usize,
        av: [u64; n],
        q: usize,
    };

    let mut all: Option<u64> = None;
    let mut add_map: HashMap<usize, u64> = HashMap::new();
    for _ in 0..q {
        input! {
            kind: usize
        };
        match kind {
            1 => {
                input! {
                    x: u64
                };
                all = Some(x);
                add_map = HashMap::new();
            }
            2 => {
                input! {
                    i: Usize1,
                    x: u64
                };
                *add_map.entry(i).or_insert(0_u64) += x;
            }
            3 => {
                input! {
                    i: Usize1
                };
                let res = if let Some(all) = all {
                    all + add_map.get(&i).unwrap_or(&0_u64)
                } else {
                    av[i] + add_map.get(&i).unwrap_or(&0_u64)
                };
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

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
    };

    let mut map = BTreeMap::new();
    for _ in 0..q {
        input! {
            kind: usize
        };
        match kind {
            1 => {
                input! {
                    x: usize
                };
                *map.entry(x).or_insert(0) += 1;
            }
            2 => {
                input! {
                    x: usize, c: usize
                };
                if let Some(v) = map.get_mut(&x) {
                    *v -= (*v).min(c);
                    if *v == 0 {
                        map.remove(&x);
                    }
                }
            }
            3 => {
                let (min, _) = map.iter().next().unwrap();
                let (max, _) = map.iter().next_back().unwrap();
                println!("{}", max - min);
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

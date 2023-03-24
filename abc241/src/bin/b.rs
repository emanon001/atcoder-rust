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
        n: usize, m: usize,
        av: [i64; n],
        bv: [i64; m]
    };

    let mut map = HashMap::new();
    for a in av {
        *map.entry(a).or_insert(0) += 1;
    }
    for b in bv {
        if let Some(c) = map.get_mut(&b) {
            if c == &0 {
                println!("No");
                return;
            }
            *c -= 1;
        } else {
            println!("No");
            return;
        }
    }
    println!("Yes");
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

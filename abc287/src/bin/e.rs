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
        sv: [Chars; n]
    };

    let mut count_map = HashMap::new();
    let m = 2_u128.pow(61) - 1;
    let b = 100000007_u128;
    for s in &sv {
        let mut h = 0_u128;
        for ch in s {
            h = (h * b + (*ch as u8) as u128) % m;
            *count_map.entry(h).or_insert(0) += 1;
        }
    }
    for s in sv {
        let mut res = 0;
        let mut h = 0_u128;
        for ch in s {
            h = (h * b + (ch as u8) as u128) % m;
            if count_map.get(&h).unwrap_or(&0) <= &1 {
                break;
            }
            res += 1;
        }
        println!("{}", res);
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

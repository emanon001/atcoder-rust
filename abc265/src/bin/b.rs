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
        n: usize, m: usize, t: i64,
        av: [i64; n - 1],
        xyv: [(Usize1, i64); m]
    };
    let bonus_map = xyv.into_iter().collect::<HashMap<usize, i64>>();
    let mut t = t;
    for (i, a) in av.into_iter().enumerate() {
        if t > a {
            t -= a;
            t += bonus_map.get(&(i + 1)).unwrap_or(&0);
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

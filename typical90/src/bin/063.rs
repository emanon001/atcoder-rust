#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[macro_export]
macro_rules! chmax {
    ($ max : expr , $ v : expr ) => {
        if $max < $v {
            $max = $v;
            true
        } else {
            false
        }
    };
}

fn solve() {
    input! {
        h: usize, w: usize,
        grid: [[usize; w]; h]
    };

    let mut res = 0;
    for bits in 1..1 << h {
        let mut map = HashMap::new();
        let mut score = 0;
        for j in 0..w {
            let mut hc = 0;
            let mut set = HashSet::new();
            for i in 0..h {
                if (bits >> i) & 1 == 1 {
                    hc += 1;
                    set.insert(grid[i][j]);
                }
            }
            if set.len() == 1 {
                let key = *set.iter().next().unwrap();
                *map.entry(key).or_insert(0) += hc;
                chmax!(score, map[&key]);
            }
        }
        chmax!(res, score);
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

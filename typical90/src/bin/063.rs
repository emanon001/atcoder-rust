#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[macro_export]
macro_rules! chmax {
    ($ max : expr , $ v : expr ) => {{
        let v = $v;
        if $max < v {
            $max = v;
            true
        } else {
            false
        }
    }};
}

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        H: usize, W: usize,
        Grid: [[usize; W]; H]
    };

    let mut ans = 0_usize;
    for bits in 0_usize..1 << H {
        let mut count_map = HashMap::new();
        let h_count = bits.count_ones() as usize;
        for j in 0..W {
            let mut v = vec![];
            for i in 0..H {
                if (bits >> i) & 1 == 1 {
                    v.push(Grid[i][j]);
                }
            }
            if v.len() > 0 && v.iter().counts().len() == 1 {
                let x = v[0];
                *count_map.entry(x).or_insert(0_usize) += 1;
            }
        }
        let w_count = count_map.values().max().unwrap_or(&0_usize);
        chmax!(ans, h_count * w_count);
    }
    println!("{}", ans);
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

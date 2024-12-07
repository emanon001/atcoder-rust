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
        H: usize, W: usize, D: isize,
        S: [Chars; H],
    };

    let mut floor_pos = Vec::new();
    for i in 0..H {
        for j in 0..W {
            if S[i][j] == '.' {
                floor_pos.push((i as isize, j as isize));
            }
        }
    }
    let mut ans = 0_isize;
    for (pos1, pos2) in floor_pos.into_iter().tuple_combinations() {
        let mut count = 0;
        for i in 0..H {
            for j in 0..W {
                if S[i][j] == '#' {
                    continue;
                }
                if ((pos1.0 - i as isize).abs() + (pos1.1 - j as isize).abs() <= D)
                    || ((pos2.0 - i as isize).abs() + (pos2.1 - j as isize).abs()) <= D
                {
                    count += 1;
                }
            }
        }
        chmax!(ans, count);
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

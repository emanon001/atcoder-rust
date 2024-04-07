#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        N: usize,
        XY: [(isize, isize); N],
    };

    for i in 0..N {
        let (x, y) = XY[i];
        let mut ans = i;
        let mut max_d = 0;
        for j in 0..N {
            if i == j {
                continue;
            }
            let (x2, y2) = XY[j];
            let xd = (x - x2).abs();
            let yd = (y - y2).abs();
            let d = xd * xd + yd * yd;
            if d > max_d {
                max_d = d;
                ans = j + 1;
            }
        }
        println!("{}", ans);
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

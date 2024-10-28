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
macro_rules! chmin {
    ($ min : expr , $ v : expr ) => {{
        let v = $v;
        if $min > v {
            $min = v;
            true
        } else {
            false
        }
    }};
}

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        N: usize,
        XY: [(i64, i64); N],
    };

    let sorted = XY.iter().cloned().sorted().collect::<Vec<_>>();
    let x_candidates = if N.is_odd() {
        vec![sorted[N / 2].0]
    } else {
        vec![sorted[N / 2 - 1].0, sorted[N / 2].0]
    };

    let sorted = XY
        .iter()
        .cloned()
        .sorted_by_key(|(_, y)| *y)
        .collect::<Vec<_>>();
    let y_candidates = if N.is_odd() {
        vec![sorted[N / 2].1]
    } else {
        vec![sorted[N / 2 - 1].1, sorted[N / 2].1]
    };

    let mut ans = 1_i64 << 60;
    for x in &x_candidates {
        for y in &y_candidates {
            let mut sum = 0;
            for &(x_i, y_i) in &XY {
                sum += (x_i - x).abs() + (y_i - y).abs();
            }
            chmin!(ans, sum);
        }
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

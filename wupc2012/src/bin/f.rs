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
        n: usize,
        points: [(usize, usize); n]
    };

    let mut used = vec![vec![false; 1000]; 1000];
    for &p in &points {
        used[p.0][p.1] = true;
    }
    let mut cusum = vec![vec![0_i32; 1000 + 1]; 1000 + 1];
    for i in 0..1000 {
        for j in 0..1000 {
            let c = cusum[i + 1][j] + cusum[i][j + 1] - cusum[i][j] + if used[i][j] { 1 } else { 0 };
            cusum[i + 1][j + 1] = c;
        }
    }
    let mut res = 0_u32;
    for i in 0..n - 1 {
        for j in i + 1..n {
            let p1 = if points[i].0 < points[j].0 {
                points[i]
            } else {
                points[j]
            };
            let p2 = if points[i].0 < points[j].0 {
                points[j]
            } else {
                points[i]
            };
            if p1.0 == p2.0 || p1.1 == p2.1 {
                continue;
            }
            if p1.1 > p2.1 {
                continue;
            }
            let min_x = p1.0;
            let max_x = p2.0;
            let min_y = p1.1;
            let max_y = p2.1;
            let p3 = (min_x, max_y);
            let p4 = (max_x, min_y);
            let inner_count = cusum[max_x][max_y] - cusum[min_x + 1][max_y] - cusum[max_x][min_y + 1] + cusum[min_x + 1][min_y + 1];
            if used[p3.0][p3.1] && used[p4.0][p4.1] && inner_count == 0 {
                let area = (max_x - min_x) * (max_y - min_y);
                chmax!(res, area as u32);
            }
        }
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

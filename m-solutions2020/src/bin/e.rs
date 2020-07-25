#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn distance(lines: &[(isize, bool)], xyp: (isize, isize, isize)) -> isize {
    let (x, y, p) = xyp;
    let mut res = std::isize::MAX;
    for &(point, is_v) in lines {
        let v = if is_v {
            // vertical
            (point - x).abs() * p
        } else {
            // horizontal
            (point - y).abs() * p
        };
        if v < res {
            res = v;
        }
    }
    res
}

fn solve() {
    input! {
        n: usize,
        xypv: [(isize, isize, isize); n]
    };

    let mut sum = 0;
    let lines = vec![(0, true), (0, false)];
    for &xyp in &xypv {
        sum += distance(&lines, xyp);
    }
    println!("{}", sum);
    for i in 1..=n {
        let mut res = std::isize::MAX;
        for v in (0..n).combinations(i) {
            for bits in 0..2 << i {
                let mut lines = lines.clone();
                for j in 0..i {
                    if (bits >> j) & 1 == 1 {
                        // vertical
                        let x = xypv[v[j]].0;
                        lines.push((x, true));
                    } else {
                        // horizontal
                        let y = xypv[v[j]].1;
                        lines.push((y, false));
                    }
                }
                // println!("{:?}", lines);
                let mut sum = 0;
                for &xyp in &xypv {
                    sum += distance(&lines, xyp);
                }
                if sum < res {
                    res = sum;
                }
            }
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

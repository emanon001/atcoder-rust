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

    let lines = vec![(0, true), (0, false)];
    let mut minv = vec![std::isize::MAX; n + 1];
    for comb in 0..3.pow(n as u32) {
        let mut lines = lines.clone();
        let mut comb = comb;
        let mut add_lines = 0;
        for i in 0..n {
            match comb % 3 {
                0 => {}
                1 => {
                    // vertical
                    let x = xypv[i].0;
                    lines.push((x, true));
                    add_lines += 1;
                }
                2 => {
                    // horizontal
                    let y = xypv[i].1;
                    lines.push((y, false));
                    add_lines += 1;
                }
                _ => unreachable!(),
            }
            comb /= 3;
        }
        let mut sum = 0;
        for &xyp in &xypv {
            sum += distance(&lines, xyp);
        }
        if sum < minv[add_lines] {
            minv[add_lines] = sum;
        }
    }

    for i in 0..=n {
        println!("{}", minv[i]);
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

#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

// let mut res = vec![vec!['.'; (s - r) as usize + 1]; (q - p) as usize + 1];
// let x = (1 - a).max(1 - b);
// let y = (n - a).min(n - b);
// for p in x..=y {
//     if p <= a + z && a + z <= q && r <= b + z && b + z <= s {
//         res[(a + z) as usize - p as usize][(b + z) as usize - r as usize] = '#';
//     }
// }

// let x = (1 - a).max(b - n);
// let y = (n - a).min(b - 1);
// for z in x..=y {
//     if p <= a + z && a + z <= q && r <= b - z && b - z <= s {
//         res[(a + z) as usize - p as usize][(b - z) as usize - r as usize] = '#';
//     }
// }
// for v in res {
//     println!("{}", v.into_iter().join(""));
// }

fn solve() {
    input! {
        n: i64, a: i64, b: i64,
        p: i64, q: i64, r: i64, s: i64
    };

    let mut res = vec![vec!['.'; (s - r) as usize + 1]; (q - p) as usize + 1];
    for i in p..=q {
        for j in r..=s {
            let d1 = a - i;
            let d2 = b - j;
            if d1.abs() != d2.abs() {
                continue;
            }
            let x = (1 - a).max(1 - b);
            let y = (n - a).min(n - b);
            if x <= y && a + x <= i && i <= a + y && b + x <= j && j <= b + y {
                res[i as usize - p as usize][j as usize - r as usize] = '#';
            }
            let x = (1 - a).max(b - n);
            let y = (n - a).min(b - 1);
            if x <= y && a + x <= i && i <= a + y && b - x >= j && j >= b - y {
                res[i as usize - p as usize][j as usize - r as usize] = '#';
            }
        }
    }
    for v in res {
        println!("{}", v.into_iter().join(""));
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

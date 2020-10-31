#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn f(n: i64, k: i64) -> i64 {
    let mut res = 0;
    for a in 1..=n {
        for b in 1..=n {
            for c in 1..=n {
                for d in 1..=n {
                    if a + b - c - d - k == 0 {
                        res += 1;
                    }
                }
            }
        }
    }
    res
}

fn solve_a(n: i64, k: i64) -> i64 {
    let mut res = 0;
    for ab in 2..=n * 2 {
        let cd = ab - k;
        if cd < 2 {
            continue;
        }
        let x = if ab - 1 <= n {
            ab - 1
        } else {
            let over = ab - n - 1;
            (ab - 1 - over * 2).max(0)
        };
        let y = if cd - 1 <= n {
            cd - 1
        } else {
            let over = cd - n - 1;
            (cd - 1 - over * 2).max(0)
        };
        res += x * y;
        // println!("ab:{}, cd:{}, x:{}, y:{}, res:{}", ab, cd, x, y, res);
    }
    res
}

fn solve() {
    input! {
        n: i64, k: i64
    }

    let res = solve_a(n, k);
    println!("{}", res);
}

// fn solve() {
//     for n in 1_i64..5 {
//         for k in (-2 * (n - 1))..=(2 * (n - 1)) {
//             let x = solve_a(n, k);
//             let y = f(n, k);
//             if x != y {
//                 println!("-------------------");
//                 println!("n:{}, k:{}, x:{}, y:{}", n, k, x, y);
//             }
//         }
//     }
// }

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

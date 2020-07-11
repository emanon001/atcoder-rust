#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

pub fn bsearch<F>(ok: i64, ng: i64, pred: F) -> Option<i64>
where
    F: Fn(i64) -> bool,
{
    let orig_ok = ok;
    let mut ok = ok;
    let mut ng = ng;
    while (ok - ng).abs() > 1 {
        let mid = (ok + ng) / 2;
        if pred(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    if ok == orig_ok {
        None
    } else {
        Some(ok)
    }
}

// fn f(n: usize) -> usize {
//     let mut res = 0;
//     for x in 1..=n {
//         for y in 1..=n {
//             let z = bsearch((n + 1) as i64, 0, |z| {
//                 let z = z as usize;
//                 let sum = (x * x) + (y * y) + (z * z) + (x * y) + (y * z) + (z * x);
//                 sum >= n
//             });
//             if let Some(z) = z {
//                 let z = z as usize;
//                 let sum = (x * x) + (y * y) + (z * z) + (x * y) + (y * z) + (z * x);
//                 if sum == n {
//                     res += 1;
//                 }
//             }
//         }
//     }
//     res
// }

fn solve() {
    input! {
        n: usize
    };

    let mut res = vec![0; n + 1];
    for x in 1..=n {
        for y in 1..=n {
            if x * y > n {
                break;
            }
            let xy = x.pow(2) + y.pow(2) + x * y;
            for z in 1..=n {
                let xyz = xy + z.pow(2) + (x + y) * z;
                if xyz > n {
                    break;
                }
                res[xyz] += 1;
            }
        }
    }
    for n in 1..=n {
        println!("{}", res[n]);
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

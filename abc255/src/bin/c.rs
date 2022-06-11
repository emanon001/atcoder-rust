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
macro_rules! chmin {
    ($ min : expr , $ v : expr ) => {
        if $min > $v {
            $min = $v;
            true
        } else {
            false
        }
    };
}

fn solve2(x: i128, a: i128, d: i128, n: i128) -> i128 {
    let mut res = (x - a).abs();
    let mut a = a;
    for _ in 0..n - 1 {
        a = a + d;
        chmin!(res, (x - a).abs());
    }
    res
}

fn solve1(x: i128, a: i128, d: i128, n: i128) -> i128 {
    (if d == 0 {
        x - a
    } else if a <= x && d <= 0 {
        x - a
    } else if a <= x && a + d * (n - 1) <= x {
        x - (a + d * (n - 1))
    } else if x <= a && d >= 0 {
        x - a
    } else if x <= a && x <= a + d * (n - 1) {
        x - (a + d * (n - 1))
    } else {
        let m = (x - a).abs() % d.abs();
        m.min((d.abs() - m).abs())
    })
    .abs()
}

fn solve() {
    input! {
        x: i128, a: i128, d: i128, n: i128
    };

    // 'outer: for x in -10..10 {
    //     for a in -10..10 {
    //         for d in -10..10 {
    //             for n in 1..10 {
    //                 let res1 = solve1(x, a, d, n);
    //                 let res2 = solve2(x, a, d, n);
    //                 if res1 != res2 {
    //                     println!("{} {} {} {} {} {}", x, a, d, n, res1, res2);
    //                     break 'outer;
    //                 }
    //             }
    //         }
    //     }
    // }

    println!("{}", solve1(x, a, d, n));
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

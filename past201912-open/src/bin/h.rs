#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn main() {
    input! {
        n: usize,
        cv: [i64; n],
        q: usize
    };

    let mut buyed_counts = vec![0_i64; n];
    let mut odd_buyed_count = 0_i64;
    let mut even_buyed_count = 0_i64;
    let mut min_odd = 1 << 60;
    let mut min_even = 1 << 60;
    for i in 0..n {
        let c = cv[i];
        let i = i + 1;
        if i % 2 == 0 {
            min_even = c.min(min_even);
        } else {
            min_odd = c.min(min_odd);
        }
    }
    for _ in 0..q {
        input! {
            kind: usize
        };
        match kind {
            1 => {
                input! {
                    x: usize,
                    a: i64
                };
                let is_even = x % 2 == 0;
                let buyed_count = buyed_counts[x - 1]
                    + if is_even {
                        even_buyed_count
                    } else {
                        odd_buyed_count
                    };
                let rest = cv[x - 1] - buyed_count - a;
                if rest >= 0 {
                    buyed_counts[x - 1] += a;
                    if is_even {
                        min_even = rest.min(min_even);
                    } else {
                        min_odd = rest.min(min_odd);
                    }
                }
            }
            2 => {
                input! {
                    a: i64
                };
                if min_odd - a >= 0 {
                    min_odd -= a;
                    odd_buyed_count += a;
                }
            }
            3 => {
                input! {
                    a: i64
                };
                if min_odd - a >= 0 && min_even - a >= 0 {
                    min_odd -= a;
                    odd_buyed_count += a;
                    min_even -= a;
                    even_buyed_count += a;
                }
            }
            _ => unreachable!(),
        }
    }
    let res = (0..n)
        .map(|i| {
            buyed_counts[i]
                + if (i + 1) % 2 == 0 {
                    even_buyed_count
                } else {
                    odd_buyed_count
                }
        })
        .sum::<i64>();
    println!("{}", res);
}

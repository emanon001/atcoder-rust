#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

pub fn is_leap_year(n: usize) -> bool {
    n % 4 == 0 && !(n % 100 == 0 && n % 400 != 0)
}
pub fn is_valid_date(y: usize, m: usize, d: usize) -> bool {
    if !(1..=12).contains(&m) {
        return false;
    }
    let last_day_each_month = vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let last_day = if m == 2 && is_leap_year(y) {
        29
    } else {
        last_day_each_month[m - 1]
    };
    (1..=last_day).contains(&d)
}

fn solve() {
    input! {
        s: String
    };

    for y in 2001..=3111 {
        for m in 1..=12 {
            for d in 1..=31 {
                let date = format!("{:0>4}/{:0>2}/{:0>2}", y, m, d);
                if is_valid_date(y, m, d) && date >= s {
                    let count = date.chars().unique().count(); // with `/`
                    if count <= 3 {
                        println!("{}", date);
                        return;
                    }
                }
            }
        }
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

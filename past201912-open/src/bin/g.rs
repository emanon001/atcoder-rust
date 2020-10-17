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
    };

    let mut points = vec![Vec::new(); n - 1];
    for i in 0..n - 1 {
        input! {
            v: [i64; n - i - 1]
        }
        points[i] = v;
    }

    let mut res = std::i64::MIN;
    for x in 0..3.pow(n as u32) {
        let mut groups = vec![Vec::new(); 3];
        let mut x = x;
        for i in 0..n {
            let g = x % 3;
            x /= 3;
            groups[g].push(i as usize);
        }
        let mut point = 0;
        for g in groups {
            for (i, j) in g.into_iter().tuple_combinations() {
                let i = i.min(j);
                let j = i.max(j) - i - 1;
                point += points[i][j];
            }
        }
        if point > res {
            res = point;
        }
    }
    println!("{}", res);
}

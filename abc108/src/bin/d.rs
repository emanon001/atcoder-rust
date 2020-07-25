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
        l: isize
    };

    let mut v = 1;
    let mut w = 1;
    let mut w_sum = 0;
    let mut res = Vec::new();
    while w_sum + w < l {
        w_sum += w;
        res.push((v, v + 1, 0));
        res.push((v, v + 1, w));
        v += 1;
        w *= 2;
    }
    let mut rest_edges = l - 2.pow(v - 1);
    let mut max_l = l - 1;
    for u in (1..v).rev() {
        let edges = if u == 1 { 1 } else { 2.pow(u - 1) };
        while edges <= rest_edges {
            let w = max_l - if u == 1 { 0 } else { 2.pow(u - 1) - 1 };
            res.push((u, v, w));
            max_l = w - 1;
            rest_edges -= edges;
        }
    }
    println!("{} {}", v, res.len());
    for (u, v, w) in res {
        println!("{} {} {}", u, v, w);
    }
}

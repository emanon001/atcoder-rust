#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[derive(Debug)]
enum Input {
    Add(usize, usize),
    Sub(usize, usize),
    Calc(usize),
}

fn main() {
    input! {
        n: usize, w: usize, c: usize,
        lrpv: [(usize, usize, usize); n]
    };

    let mut inputs = Vec::new();
    inputs.push(Input::Calc(c));
    for &(l, r, p) in &lrpv {
        inputs.push(Input::Add(l, p));
        inputs.push(Input::Sub(r + c, p));
        inputs.push(Input::Calc(std::cmp::min(r + c, w)));
    }
    inputs.sort_by_key(|i| match i {
        Input::Add(x, _) => (*x, 3),
        Input::Sub(x, _) => (*x, 1),
        Input::Calc(x) => (*x, 2),
    });
    let mut res = std::usize::MAX;
    let mut cur = 0_usize;
    for i in inputs {
        match i {
            Input::Add(_, p) => {
                cur += p;
            }
            Input::Sub(_, p) => {
                cur -= p;
            }
            Input::Calc(_) => {
                if cur < res {
                    res = cur;
                }
            }
        }
    }
    println!("{}", res);
}

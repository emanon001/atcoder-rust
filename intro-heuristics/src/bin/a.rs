#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

use rand::prelude::*;

const CN: usize = 26;

fn calc(cv: &[isize], sv: &[Vec<isize>], tv: &[usize]) -> isize {
    let mut last = vec![0; CN];
    let mut res = 0;
    for i in 0..sv.len() {
        let t = tv[i];
        res += sv[i][t];
        last[t] = i + 1;
        for j in 0..CN {
            res -= cv[j] * (i + 1 - last[j]) as isize;
        }
    }
    res
}

fn gen_tv(rng: &mut rand::rngs::ThreadRng, d: usize) -> Vec<usize> {
    (0..d)
        .into_iter()
        .map(|_| rng.gen::<usize>() % CN)
        .collect::<Vec<_>>()
}

fn main() {
    input! {
        d: usize,
        cv: [isize; CN],
        sv: [[isize; CN]; d],
    };

    let mut rng = rand::thread_rng();
    let mut tv = gen_tv(&mut rng, d);
    let mut res = calc(&cv, &sv, &tv);
    for _ in 0..20000 {
        let tv2 = gen_tv(&mut rng, d);
        let p = calc(&cv, &sv, &tv2);
        if p > res {
            tv = tv2;
            res = p;
        }
    }
    for _ in 0..400000 {
        let di = rng.gen::<usize>() % d;
        let q = rng.gen::<usize>() % CN;
        let old = tv[di];
        tv[di] = q;
        let p = calc(&cv, &sv, &tv);
        if p > res {
            res = p;
        } else {
            tv[di] = old;
        }
    }
    println!("{}", tv.into_iter().map(|x| x + 1).join("\n"));
}

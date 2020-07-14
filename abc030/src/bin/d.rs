#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn is_needed_loop(k: &[usize], max_step: usize) -> bool {
    let mut m = 0;
    for x in k {
        m = m * 10 + x;
        if m > max_step {
            return true;
        }
    }
    false
}

fn solve() {
    input! {
        n: usize, a: Usize1,
        k: Chars,
        bv: [Usize1; n]
    };

    let k = k
        .into_iter()
        .map(|ch| ch.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>();
    let mut step_v = vec![-1; n];
    let mut last_pos = a;
    let mut step = 1_isize;
    loop {
        if step_v[last_pos] != -1 {
            step -= 1;
            break;
        }
        step_v[last_pos] = step;
        last_pos = bv[last_pos];
        step += 1;
    }
    if !is_needed_loop(&k, step as usize) {
        let k = k.into_iter().fold(0, |acc, x| acc * 10 + x);
        let i = step_v.into_iter().position(|x| x as usize == k).unwrap();
        let res = bv[i] + 1;
        println!("{}", res);
        return;
    }
    let loop_size = (step - step_v[last_pos] + 1) as usize;
    let mut m = 0;
    for x in k {
        m = (m * 10 + x) % loop_size;
    }
    let before_loop_size = step as usize - loop_size;
    let i = (m + loop_size - (before_loop_size % loop_size) - 1) % loop_size;
    let i = step_v
        .into_iter()
        .position(|x| x as usize == i + before_loop_size + 1)
        .unwrap();
    let res = bv[i] + 1;
    println!("{}", res);
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

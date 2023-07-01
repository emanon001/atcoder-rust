#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
use rand::{thread_rng, Rng};
#[allow(unused_imports)]
use std::collections::*;

fn solve() {
    input! {
        n: usize,
        av: [Usize1; n]
    };
    // loop {
    //     let av = gen();
    //     let n = av.len();
    //     let res1 = solve1(av.clone(), n);
    //     let res2 = solve2(av.clone(), n);
    //     if res1 != res2 {
    //         println!("av: {:?}, expected: {}, actual: {}", av, res2, res1);
    //         return;
    //     }
    // }

    println!("{}", solve1(av, n));
}

fn gen() -> Vec<usize> {
    let mut rng = thread_rng();
    let n = rng.gen_range(1, 1000);
    let mut res = Vec::new();
    for _ in 0..n {
        res.push(rng.gen_range(0, n));
    }
    println!("{:?}", res);
    res
}

fn solve1(av: Vec<usize>, n: usize) -> u64 {
    let mut count1 = 0_u64;
    for i in 0..n {
        if av[i] == i {
            count1 += 1;
        }
    }
    let mut res = if count1 >= 2 {
        count1 * (count1 - 1) / 2
    } else {
        0
    };
    let mut count2 = 0;
    for i in 0..n {
        let j = av[i];
        if i != j && av[j] == i {
            count2 += 1;
        }
    }
    res += count2 / 2;
    res
}

fn solve2(av: Vec<usize>, n: usize) -> u64 {
    let mut res = 0_u64;
    for i in 0..n {
        for j in i + 1..n {
            if av[i].min(av[j]) == i && av[i].max(av[j]) == j {
                res += 1;
            }
        }
    }
    res
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

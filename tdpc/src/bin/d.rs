#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

pub fn prime_factor(n: u64) -> std::collections::HashMap<u64, u64> {
    if n < 2 {
        return std::collections::HashMap::new();
    }
    let mut res = std::collections::HashMap::new();
    let mut n = n;
    let mut i = 2;
    while i * i <= n {
        while n % i == 0 {
            let c = res.entry(i).or_insert(0);
            *c += 1;
            n /= i;
        }
        i += 1;
    }
    if n != 1 {
        res.insert(n, 1);
    }
    res
}

fn main() {
    input! {
        n: usize, d: u64
    };

    if d == 1 {
        println!("1");
        return;
    }

    let pf = prime_factor(d);
    let pset = pf.keys().copied().collect::<HashSet<_>>();
    let dice_pset = vec![2, 3, 5].into_iter().collect::<HashSet<_>>();
    if pset.difference(&dice_pset).count() > 0 {
        println!("0");
        return;
    }

    let p2e = *pf.get(&2).unwrap_or(&0) as usize;
    let p3e = *pf.get(&3).unwrap_or(&0) as usize;
    let p5e = *pf.get(&5).unwrap_or(&0) as usize;

    // dp[i][j][k][l] = 確率
    // i: i回目までサイコロを振った
    // j: 素因数2の指数
    // k: 素因数3の指数
    // l: 素因数5の指数
    let mut dp = vec![vec![vec![vec![0.0_f64; p5e + 1]; p3e + 1]; p2e + 1]; n + 1];
    dp[0][0][0][0] = 1.0;
    for i in 0..n {
        for x in 1..=6 {
            let (p2, p3, p5) = match x {
                1 => (0, 0, 0),
                2 => (1, 0, 0),
                3 => (0, 1, 0),
                4 => (2, 0, 0),
                5 => (0, 0, 1),
                6 => (1, 1, 0),
                _ => unreachable!(),
            };
            for j in 0..p2e + 1 {
                for k in 0..p3e + 1 {
                    for l in 0..p5e + 1 {
                        let to_i = i + 1;
                        let to_j = std::cmp::min(j + p2, p2e);
                        let to_k = std::cmp::min(k + p3, p3e);
                        let to_l = std::cmp::min(l + p5, p5e);
                        dp[to_i][to_j][to_k][to_l] += dp[i][j][k][l] / 6.0;
                    }
                }
            }
        }
    }
    let res = dp[n][p2e][p3e][p5e];
    println!("{}", res);
}

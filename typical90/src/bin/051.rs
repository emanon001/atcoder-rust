#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

pub fn bsearch<F>(ok: i64, ng: i64, pred: F) -> Option<i64>
where
    F: Fn(i64) -> bool,
{
    let orig_ok = ok;
    let mut ok = ok;
    let mut ng = ng;
    while (ok - ng).abs() > 1 {
        let mid = (ok + ng) / 2;
        if pred(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    if ok == orig_ok {
        None
    } else {
        Some(ok)
    }
}

fn solve() {
    input! {
        n: usize, k: usize, p: i64,
        av: [i64; n]
    };

    if n == 1 || k == 1 {
        let mut res = 0;
        for a in av {
            if a <= p {
                res += 1;
            }
        }
        println!("{}", res);
        return;
    }

    let n1 = n / 2;
    let mut dp1 = vec![HashMap::new(); n1 + 1];
    for bits in 0..1 << n1 {
        let mut c = 0;
        let mut p = 0;
        for i in 0..n1 {
            if (bits >> i) & 1 == 1 {
                c += 1;
                p += av[i];
            }
        }
        *dp1[c].entry(p).or_insert(0) += 1;
    }

    let n2 = n - n1;
    let mut dp2 = vec![HashMap::new(); n2 + 1];
    for bits in 0..1 << n2 {
        let mut c = 0;
        let mut p = 0;
        for i in 0..n2 {
            if (bits >> i) & 1 == 1 {
                c += 1;
                p += av[i + n1];
            }
        }
        *dp2[c].entry(p).or_insert(0) += 1;
    }
    let mut sorted2 = vec![Vec::new(); dp2.len()];
    for i in 0..dp2.len() {
        for (k, v) in &dp2[i] {
            for _ in 0..*v {
                sorted2[i].push(k);
            }
        }
        sorted2[i].sort();
    }

    let mut res = 0_i64;
    for i in 0..=n1 {
        if i > k {
            break;
        }
        if k - i > n2 {
            continue;
        }
        for (p1, c1) in &dp1[i] {
            let c2 = bsearch(-1 as i64, sorted2[k - i].len() as i64, |x| {
                p1 + sorted2[k - i][x as usize] <= p
            });
            res += c1 * c2.map(|c| c + 1).unwrap_or(0);
        }
    }
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

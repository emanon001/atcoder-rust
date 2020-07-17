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

enum InputType {
    A,
    B,
    C,
}

fn make_sorted_wv_list(vwv: &[(u64, u64)]) -> Vec<(u64, u64)> {
    let mut map = BTreeMap::new();
    let len = vwv.len();
    for bits in 1..2 << len {
        let mut v_sum = 0_u64;
        let mut w_sum = 0_u64;
        for i in 0..len {
            if (bits >> i) & 1 == 1 {
                let (v, w) = vwv[i];
                v_sum += v;
                w_sum += w;
            }
        }
        let entry = map.entry(w_sum).or_insert(0);
        if v_sum > *entry {
            *entry = v_sum;
        }
    }
    let mut res = Vec::new();
    res.push((0, 0));
    let mut max_v = 0;
    for (w, v) in map {
        if v > max_v {
            res.push((w, v));
            max_v = v;
        }
    }
    res
}

fn solve_a(input: Input) -> u64 {
    let Input { n, w, vwv } = input;
    let a_size = n / 2;
    let a_list = make_sorted_wv_list(&vwv[..a_size]);
    let b_list = make_sorted_wv_list(&vwv[a_size..]);
    let mut res = 0_u64;
    for (aw, av) in a_list {
        if aw > w as u64 {
            continue;
        }
        let bi = bsearch(-1, b_list.len() as i64, |x| {
            let bi = x as usize;
            let (bw, _) = b_list[bi];
            aw + bw <= w as u64
        });
        let v_sum = if let Some(bi) = bi {
            let (_, bv) = b_list[bi as usize];
            av + bv
        } else {
            av
        };
        if v_sum > res {
            res = v_sum;
        }
    }
    res
}

fn solve_b(input: Input) -> u64 {
    let Input { n, w, vwv } = input;
    let max_w = n * 10.pow(3);
    let mut dp = vec![vec![0_u64; max_w + 1]; n + 1];
    for i in 0..n {
        let (v, w) = vwv[i];
        for j in 0..max_w + 1 {
            dp[i + 1][j] = dp[i + 1][j].max(dp[i][j]);
            if j as u64 + w > max_w as u64 {
                continue;
            }
            dp[i + 1][j + w as usize] = dp[i + 1][j + w as usize].max(dp[i][j] + v);
        }
    }
    dp[n][w.min(max_w)]
}

fn solve_c(input: Input) -> u64 {
    let Input { n, w, vwv } = input;
    let max_v = n * 10.pow(3);
    let inf = 1_u64 << 60;
    let mut dp = vec![vec![inf; max_v + 1]; n + 1];
    dp[0][0] = 0;
    for i in 0..n {
        let (v, w) = vwv[i];
        for j in 0..max_v + 1 {
            dp[i + 1][j] = dp[i + 1][j].min(dp[i][j]);
            if j as u64 + v > max_v as u64 {
                continue;
            }
            dp[i + 1][j + v as usize] = dp[i + 1][j + v as usize].min(dp[i][j] + w);
        }
    }
    let mut res = 0_u64;
    for v in (0..max_v + 1).rev() {
        if dp[n][v] <= w as u64 {
            res = v as u64;
            break;
        }
    }
    res
}

struct Input {
    n: usize,
    w: usize,
    vwv: Vec<(u64, u64)>,
}

fn solve() {
    input! {
        n: usize, w: usize,
        vwv: [(u64, u64); n]
    };

    let input_type = if n <= 30 {
        InputType::A
    } else if vwv.iter().all(|(_, w)| *w <= 1000) {
        InputType::B
    } else {
        InputType::C
    };
    let input = Input { n, w, vwv };
    let res = match input_type {
        InputType::A => solve_a(input),
        InputType::B => solve_b(input),
        InputType::C => solve_c(input),
    };
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

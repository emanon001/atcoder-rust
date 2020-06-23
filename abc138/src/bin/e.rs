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

fn main() {
    input! {
        s: Chars,
        t: Chars
    };

    let s_len = s.len();
    let mut pos_table = HashMap::new();
    for (i, ch) in s.into_iter().enumerate() {
        pos_table.entry(ch).or_insert(Vec::new()).push(i as i64);
    }
    let keys = pos_table.keys().cloned().collect::<Vec<_>>();
    for k in keys {
        let v = pos_table.get_mut(&k).unwrap();
        for i in v.clone() {
            v.push(i + s_len as i64);
        }
    }
    let mut res = 0;
    let mut cur_pos = -1;
    for ch in t {
        let posv = match pos_table.get(&ch) {
            Some(v) => v,
            _ => {
                println!("-1");
                return;
            }
        };
        let i = bsearch(posv.len() as i64, -1, |x| {
            let p = posv[x as usize];
            cur_pos < p
        });
        if let Some(i) = i {
            let p = posv[i as usize];
            res += p - cur_pos;
            cur_pos = p % s_len as i64;
        } else {
            unreachable!();
        }
    }
    println!("{}", res);
}

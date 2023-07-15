#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
use regex::Regex;
#[allow(unused_imports)]
use std::collections::*;

fn underscore_comb_count(allow_underscore_count: usize, list_size: usize) -> usize {
    let n = allow_underscore_count;
    let k = list_size - 1;
    let a = (1..=n).fold(1, |acc, x| acc * x);
    let b = (1..=k).fold(1, |acc, x| acc * x);
    a / b
}

fn underscore_comb(allow_underscore_count: usize, list_size: usize) -> Vec<Vec<String>> {
    let n = allow_underscore_count;
    let k = list_size - 1;
    let mut res = Vec::new();
    for comb in (1..=n).combinations(k) {
        let mut comb = comb;
        comb.sort();
        let mut cur = 0;
        let mut sep = Vec::new();
        for i in comb {
            sep.push("_".repeat(i - cur));
            cur = i;
        }
        res.push(sep);
    }
    res
}

fn solve() {
    input! {
        n: usize, m: usize,
        sv: [String; n],
        tv: [String; m]
    };

    // println!("{:?}", max_count(16 - 14, n));
    // for sep in sep_comb(16 - 14, n) {
    //     println!("{:?}", sep);
    // }

    let sep = Regex::new(r"[a-z]+").unwrap();
    let mut map = HashMap::new();
    for t in tv {
        if t.starts_with("-") || t.ends_with("-") {
            continue;
        }
        let v = sep
            .split(&t)
            .filter(|t| !t.is_empty())
            .map(|t| t.to_string())
            .collect::<Vec<_>>();
        let key = t.replace("_", "");
        map.entry(key).or_insert(HashSet::new()).insert(v);
    }
    for perm in (0..n).permutations(n) {
        let s = perm.iter().map(|&i| sv[i].clone()).collect::<Vec<_>>();
        let key = s.join("");
        if !map.contains_key(&key) {
            let res = s.join("_");
            if !(3..=16).contains(&res.len()) {
                continue;
            }
            println!("{}", res);
            return;
        }
        // 個数を確認
        let count = map[&key].len();
        if count >= underscore_comb_count(16 - key.len(), n) {
            continue;
        }
        for comb in underscore_comb(16 - key.len(), n) {
            if map[&key].contains(&comb) {
                continue;
            }
            let res = s
                .clone()
                .into_iter()
                .zip(comb.into_iter().chain(vec!["".to_string()]))
                .map(|(a, b)| format!("{}{}", a, b))
                .join("");
            if !(3..=16).contains(&res.len()) {
                continue;
            }
            println!("{}", res);
            return;
        }
    }
    println!("-1");
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

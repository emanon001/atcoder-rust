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

fn solve() {
    input! {
        s: String
    };

    let regex = Regex::new(r#"(S|H|D|C)(A|J|Q|K|\d+)"#).unwrap();
    let mut deck = Vec::new();
    let mut rsf = HashMap::new();
    for w in regex.captures_iter(&s) {
        let mark = &w[1];
        let num = &w[2];
        deck.push((mark.to_string(), num.to_string()));
        match num {
            "10" | "J" | "Q" | "K" | "A" => {
                rsf.entry(mark.to_string())
                    .or_insert(HashSet::new())
                    .insert(num.to_string());
                if rsf.get(&mark.to_string()).unwrap().len() == 5 {
                    let mut discard = Vec::new();
                    for (m, n) in deck {
                        let n = n.as_ref();
                        if m != mark {
                            discard.push(format!("{}{}", m, n));
                        } else if !vec!["10", "J", "Q", "K", "A"].contains(&n) {
                            discard.push(format!("{}{}", m, n));
                        }
                    }
                    let res = if discard.is_empty() {
                        "0".into()
                    } else {
                        discard.join("")
                    };
                    println!("{}", res);
                    return;
                }
            }
            _ => {}
        }
    }
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

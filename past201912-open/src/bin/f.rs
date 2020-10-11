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

fn main() {
    input! {
        s: String
    };

    let re = Regex::new(r"[A-Z][a-z]*[A-Z]").unwrap();
    let mut words = Vec::new();
    for c in re.captures_iter(&s) {
        words.push(c[0].to_string());
    }
    words.sort_by_key(|w| w.to_ascii_lowercase());
    let res = words.join("");
    println!("{}", res);
}

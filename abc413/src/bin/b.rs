#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[allow(non_snake_case)]
fn main() {
    input_interactive! {
        N: usize,
        S: [String; N],
    };

    let mut set = HashSet::new();
    for i in 0..N {
        for j in i + 1..N {
            let s1 = format!("{}{}", S[i], S[j]);
            set.insert(s1);
            let s2 = format!("{}{}", S[j], S[i]);
            set.insert(s2);
        }
    }
    let ans = set.len();
    println!("{}", ans);
}

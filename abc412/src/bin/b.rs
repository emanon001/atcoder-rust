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
        S: Chars,
        T: Chars,
    };

    let t_set = T.into_iter().collect::<HashSet<_>>();

    let mut chars = vec![];
    for i in 0..S.len() - 1 {
        if S[i + 1].is_uppercase() {
            chars.push(S[i]);
        }
    }

    let is_ok = chars.iter().all(|ch| t_set.contains(ch));
    let ans = if is_ok { "Yes" } else { "No" };
    println!("{}", ans);
}

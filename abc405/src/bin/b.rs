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
        N: usize, M: usize,
        A: [usize; N]
    };

    for i in 0..=N {
        let set = A.iter().take(N - i).collect::<HashSet<_>>();
        if set.len() != M {
            println!("{}", i);
            return;
        }
    }
}

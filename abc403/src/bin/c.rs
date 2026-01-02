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
        N: usize, M: usize, Q: usize,
    };

    let all_contest_page = M;
    let mut can_access = vec![HashSet::new(); N];
    for _ in 0..Q {
        input_interactive! {
            kind: usize,
        };
        match kind {
            1 => {
                input_interactive! {
                    x: Usize1,
                    y: Usize1,
                };
                can_access[x].insert(y);
            }
            2 => {
                input_interactive! {
                    x: Usize1,
                };
                can_access[x].insert(all_contest_page);
            }
            3 => {
                input_interactive! {
                    x: Usize1,
                    y: Usize1,
                };
                let ans = if can_access[x].contains(&all_contest_page) || can_access[x].contains(&y)
                {
                    "Yes"
                } else {
                    "No"
                };
                println!("{}", ans);
            }
            _ => unreachable!(),
        }
    }
}

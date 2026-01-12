#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[macro_export]
macro_rules! chmax {
    ($ max : expr , $ v : expr ) => {{
        let v = $v;
        if $max < v {
            $max = v;
            true
        } else {
            false
        }
    }};
}

#[allow(non_snake_case)]
fn main() {
    input_interactive! {
        N: usize,
        A: [usize; N],
    };

    let mut ans = 0;
    let counts = A.into_iter().counts().into_iter().sorted();
    let mut count_sum = 0;
    for (a, c) in counts {
        let rest = N - count_sum;
        if a <= rest {
            chmax!(ans, a);
        }
        if a >= rest {
            chmax!(ans, rest);
        }
        count_sum += c;
    }
    println!("{}", ans);
}
